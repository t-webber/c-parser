//! Implements the methods of [`Ast`]
//!
//! These methods are used for the implementation of
//! [`Push`](crate::parser::modifiers::push::Push) for [`Ast`], but also to
//! simplify the api of [`Ast`].

use super::Ast;
use super::can_push::{AstPushContext, CanPush as _};
use crate::parser::keyword::control_flow::node::ControlFlowNode;
use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::modifiers::push::Push as _;
use crate::parser::operators::api::{Binary, Ternary, Unary};
use crate::parser::symbols::api::{BracedBlock, Cast, ListInitialiser};
use crate::parser::variable::api::{PureType as _, VariableConversion as _};

impl Ast {
    /// Wrapper for [`CanPush`](super::can_push::CanPush) with additional
    /// context
    pub fn can_push_leaf_with_ctx(&self, ctx: AstPushContext) -> bool {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::custom_print(&format!(
            "Can push leaf in {self} with ctx {ctx:?}"
        ));
        match self {
            Self::Empty
            | Self::Cast(Cast { full: true, .. })
            | Self::Ternary(Ternary { failure: None, .. }) => true,
            Self::Variable(var) => ctx.is_user_variable() || var.can_push_leaf(),
            Self::ParensBlock(parens) => parens.is_pure_type() && ctx.is_user_variable(),
            Self::Leaf(_) | Self::FunctionCall(_) => false,
            Self::Cast(Cast { full: false, value: arg, .. })
            | Self::Unary(Unary { arg, .. })
            | Self::Binary(Binary { arg_r: arg, .. })
            | Self::Ternary(Ternary { failure: Some(arg), .. }) => arg.can_push_leaf_with_ctx(ctx),
            Self::FunctionArgsBuild(vec) => vec
                .last()
                .is_none_or(|child| child.can_push_leaf_with_ctx(ctx)),
            Self::BracedBlock(BracedBlock { elts: vec, full })
            | Self::ListInitialiser(ListInitialiser { full, elts: vec }) =>
                !*full
                    && vec
                        .last()
                        .is_none_or(|child| child.can_push_leaf_with_ctx(ctx)),
            // Full not complete because: `if (0) 1; else 2;`
            Self::ControlFlow(ctrl) if ctx.is_else() => {
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
            Self::ControlFlow(ctrl) => !ctrl.is_complete(),
        }
    }

    /// Creates an empty [`Ast`] inside a [`Box`] to initialise nodes
    pub fn empty_box() -> Box<Self> {
        Self::Empty.into_box()
    }

    /// Marks the [`Ast`] as full.
    pub fn fill(&mut self) {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::custom_print(&format!("Filling ast {self}"));
        match self {
            Self::Unary(Unary { arg, .. })
            | Self::Binary(Binary { arg_r: arg, .. })
            | Self::Ternary(Ternary { failure: Some(arg), .. } | Ternary { success: arg, .. }) =>
                arg.fill(),
            Self::ControlFlow(ctrl) => ctrl.fill(),
            Self::Cast(Cast { full, .. })
            | Self::BracedBlock(BracedBlock { full, .. })
            | Self::ListInitialiser(ListInitialiser { full, .. }) => *full = true,
            Self::Variable(var) => var.fill(),
            Self::FunctionCall(_)
            | Self::FunctionArgsBuild(_)
            | Self::Empty
            | Self::Leaf(_)
            | Self::ParensBlock(_) => (),
        }
    }

    /// Convert an [`Ast`] into a [`Box<Ast>`]
    pub fn into_box(self) -> Box<Self> {
        Box::new(self)
    }

    /// Checks if an [`Ast`] is empty
    pub const fn is_empty(&self) -> bool {
        matches!(self, &Self::Empty)
    }

    /// Push an [`Ast`] as leaf into a vector [`Ast`].
    ///
    /// This is a wrapper for [`Ast::push_block_as_leaf`].
    pub(super) fn push_block_as_leaf_in_vec(
        vec: &mut Vec<Self>,
        mut node: Self,
    ) -> Result<Option<Self>, String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_in_vec(&node, vec, "vec");
        if let Some(last) = vec.last_mut() {
            let ctx = if matches!(node, Self::Variable(_)) {
                AstPushContext::UserVariable
            } else {
                AstPushContext::None
            };
            if let Self::ParensBlock(parens) = last
                && let Some(new_ast) = Cast::parens_node_into_cast(parens, &mut node)
            {
                *last = new_ast;
            } else if last.can_push_leaf_with_ctx(ctx) {
                last.push_block_as_leaf(node)?;
            } else if matches!(last, Self::BracedBlock(_)) {
                // Example: `{{a}b}`
                vec.push(node);
            } else if let Self::ControlFlow(ctrl) = last
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
        match self {
            Self::BracedBlock(BracedBlock { elts, full: false }) => {
                if let Some(last_mut) = elts.last_mut() {
                    if let Self::ControlFlow(ctrl) = last_mut
                        && !ctrl.is_full()
                    {
                        ctrl.push_block_as_leaf(braced_block)?;
                    } else if let Self::Variable(var) = last_mut
                        && let Some((keyword, name)) = var.as_partial_typedef()
                    {
                        if let Self::BracedBlock(block) = braced_block {
                            *last_mut =
                                Self::ControlFlow(keyword.to_control_flow(name, Some(block)));
                        } else {
                            panic!("see above: still block")
                        }
                    } else {
                        elts.push(braced_block);
                    }
                } else {
                    elts.push(braced_block);
                }
            }
            Self::ControlFlow(ctrl) if !ctrl.is_full() => ctrl.push_block_as_leaf(braced_block)?,
            Self::Empty => *self = braced_block,
            Self::Leaf(_)
            | Self::Cast(_)
            | Self::Unary(_)
            | Self::Binary(_)
            | Self::Ternary(_)
            | Self::Variable(_)
            | Self::ParensBlock(_)
            | Self::BracedBlock(_)
            | Self::ControlFlow(_)
            | Self::FunctionCall(_)
            | Self::ListInitialiser(_)
            | Self::FunctionArgsBuild(_) => {
                panic!("Trying to push block {braced_block} in {self}")
            }
        }
        Ok(())
    }
}
