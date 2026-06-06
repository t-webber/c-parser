//! Implements the methods of [`Ast`]
//!
//! These methods are used for the implementation of
//! [`Push`](crate::parser::modifiers::push::Push) for [`Ast`], but also to
//! simplify the api of [`Ast`].

use super::Ast;
use super::can_push::{AstPushContext, CanPush as _};
use crate::parser::api::AstValue;
use crate::parser::keyword::control_flow::node::ControlFlowNode;
use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::literal::Attribute;
use crate::parser::modifiers::push::Push as _;
use crate::parser::operators::api::{Binary, Ternary, Unary};
use crate::parser::symbols::api::{BracedBlock, Cast, ListInitialiser};
use crate::parser::variable::api::{PureType as _, VariableConversion as _};

impl Ast {
    /// Wrapper for [`CanPush`](super::can_push::CanPush) with additional
    /// context
    pub fn can_push_leaf_with_ctx(&self, ctx: AstPushContext) -> bool {
        #[cfg(feature = "debug")]
        crate::lgp!("Can push leaf in {self} with ctx {ctx:?}");
        match &self.value {
            AstValue::Empty
            | AstValue::Cast(Cast { full: true, .. })
            | AstValue::Ternary(Ternary { failure: None, .. }) => true,
            AstValue::Variable(var) => ctx.is_user_variable() || var.can_push_leaf(),
            AstValue::ParensBlock(parens) => parens.is_pure_type() && ctx.is_user_variable(),
            AstValue::Leaf(_) | AstValue::FunctionCall(_) => false,
            AstValue::Cast(Cast { full: false, value: arg, .. })
            | AstValue::Unary(Unary { arg, .. })
            | AstValue::Binary(Binary { arg_r: arg, .. })
            | AstValue::Ternary(Ternary { failure: Some(arg), .. }) =>
                arg.can_push_leaf_with_ctx(ctx),
            AstValue::FunctionArgsBuild(vec) => vec
                .last()
                .is_none_or(|child| child.can_push_leaf_with_ctx(ctx)),
            AstValue::BracedBlock(BracedBlock { elts: vec, full })
            | AstValue::ListInitialiser(ListInitialiser { full, elts: vec }) =>
                !*full
                    && vec
                        .last()
                        .is_none_or(|child| child.can_push_leaf_with_ctx(ctx)),
            // Full not complete because: `if (0) 1; else 2;`
            AstValue::ControlFlow(ctrl) if ctx.is_else() => {
                if let ControlFlowNode::Condition(cond) = ctrl
                    && cond.no_else()
                {
                    true
                } else {
                    ctrl.as_ast()
                        .is_some_and(|ast| ast.can_push_leaf_with_ctx(ctx))
                }
            }
            // Complete not full because: `if (0) 1; 2;`
            AstValue::ControlFlow(ctrl) => !ctrl.is_complete(),
        }
    }

    /// Creates an empty [`Ast`] inside a [`Box`] to initialise nodes
    pub fn empty_box() -> Box<Self> {
        Into::<Self>::into(AstValue::Empty).into_box()
    }

    /// Marks the [`Ast`] as full.
    pub fn fill(&mut self) {
        #[cfg(feature = "debug")]
        crate::lgp!("Filling ast {self}");
        match &mut self.value {
            AstValue::Unary(Unary { arg, .. })
            | AstValue::Binary(Binary { arg_r: arg, .. })
            | AstValue::Ternary(
                Ternary { failure: Some(arg), .. } | Ternary { success: arg, .. },
            ) => arg.fill(),
            AstValue::ControlFlow(ctrl) => ctrl.fill(),
            AstValue::Cast(Cast { full, .. })
            | AstValue::BracedBlock(BracedBlock { full, .. })
            | AstValue::ListInitialiser(ListInitialiser { full, .. }) => *full = true,
            AstValue::Variable(var) => var.fill(),
            AstValue::FunctionCall(_)
            | AstValue::FunctionArgsBuild(_)
            | AstValue::Empty
            | AstValue::Leaf(_)
            | AstValue::ParensBlock(_) => (),
        }
    }

    /// Convert an [`Ast`] into a [`Box<Ast>`]
    pub fn into_box(self) -> Box<Self> {
        Box::new(self)
    }

    /// Takes the attributes from inside self it is a type;
    pub fn into_type(self) -> Option<Vec<Attribute>> {
        if let AstValue::Variable(var) = self.value {
            var.into_type()
        } else {
            None
        }
    }

    /// Checks if an [`Ast`] is empty
    pub const fn is_empty(&self) -> bool {
        matches!(self.value, AstValue::Empty)
    }

    /// Push an [`Ast`] as leaf into a vector [`Ast`].
    ///
    /// This is a wrapper for [`Ast::push_block_as_leaf`].
    pub(super) fn push_block_as_leaf_in_vec(
        vec: &mut Vec<Self>,
        mut node: Self,
    ) -> Result<Option<Self>, String> {
        #[cfg(feature = "debug")]
        crate::lgp!(
            "Pushing {node} as leaf in vec [{}]",
            vec.iter()
                .map(|n| format!("{n}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
        if let Some(last) = vec.last_mut() {
            let ctx = if matches!(node.value, AstValue::Variable(_)) {
                AstPushContext::UserVariable
            } else {
                AstPushContext::None
            };
            if let AstValue::ParensBlock(parens) = &mut last.value
                && let Some(new_ast) = Cast::parens_node_into_cast(parens, &mut node)
            {
                *last = new_ast;
            } else if last.can_push_leaf_with_ctx(ctx) {
                last.push_block_as_leaf(node)?;
            } else if matches!(last.value, AstValue::BracedBlock(_)) {
                // Example: `{{a}b}`
                vec.push(node);
            } else if let AstValue::ControlFlow(ctrl) = &last.value
                && ctrl.is_complete()
            {
                // Example `if(a) {return x} b`
                vec.push(node);
            } else {
                // Example: {a b}
                // Error
                return Ok(Some(node));
            }
        } else {
            vec.push(node);
        }
        Ok(None)
    }

    /// Pushes a [`BracedBlock`] into an [`Ast`]
    pub fn push_braced_block(&mut self, braced_block: Self) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf_in(&braced_block, "braced", self, "ast");
        match &mut self.value {
            AstValue::BracedBlock(BracedBlock { elts, full: false }) => {
                if let Some(last_mut) = elts.last_mut() {
                    if let AstValue::ControlFlow(ctrl) = &mut last_mut.value
                        && !ctrl.is_full()
                    {
                        ctrl.push_block_as_leaf(braced_block)?;
                    } else if let AstValue::Variable(var) = &mut last_mut.value
                        && let Some((keyword, name)) = var.as_partial_typedef()
                    {
                        if let AstValue::BracedBlock(block) = braced_block.value {
                            *last_mut =
                                AstValue::ControlFlow(keyword.to_control_flow(name, Some(block)))
                                    .into();
                        } else {
                            unreachable!("see above: still block")
                        }
                    } else {
                        elts.push(braced_block);
                    }
                } else {
                    elts.push(braced_block);
                }
            }
            AstValue::ControlFlow(ctrl) if !ctrl.is_full() =>
                ctrl.push_block_as_leaf(braced_block)?,
            AstValue::Empty => *self = braced_block,
            AstValue::Leaf(_)
            | AstValue::Cast(_)
            | AstValue::Unary(_)
            | AstValue::Binary(_)
            | AstValue::Ternary(_)
            | AstValue::Variable(_)
            | AstValue::ParensBlock(_)
            | AstValue::BracedBlock(_)
            | AstValue::ControlFlow(_)
            | AstValue::FunctionCall(_)
            | AstValue::ListInitialiser(_)
            | AstValue::FunctionArgsBuild(_) => {
                unreachable!("Trying to push block {braced_block} in {self}")
            }
        }
        Ok(())
    }
}
