//! Implements the methods of [`Ast`]
//!
//! These methods are used for the implementation of
//! [`Push`](crate::parser::modifiers::push::Push) for [`Ast`], but also to
//! simplify the api of [`Ast`].

use super::Ast;
use super::can_push::{AstPushContext, CanPush as _};
use crate::errors::api::ErrorLocation;
use crate::parser::keyword::control_flow::node::ControlFlowNode;
use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::literal::Attribute;
use crate::parser::modifiers::push::Push as _;
use crate::parser::operators::api::{Binary, Ternary, Unary};
use crate::parser::symbols::api::{BracedBlock, Cast, FunctionCall, ListInitialiser};
use crate::parser::variable::api::{PureType as _, VariableConversion as _};

impl Ast {
    /// Wrapper for [`CanPush`](super::can_push::CanPush) with additional
    /// context
    #[must_use]
    pub(crate) fn can_push_leaf_with_ctx(&self, ctx: AstPushContext) -> bool {
        #[cfg(feature = "debug")]
        crate::lgp!("Can push leaf in {self} with ctx {ctx:?}");
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
    #[must_use]
    pub fn empty_box() -> Box<Self> {
        Self::Empty.into_box()
    }

    /// Marks the [`Ast`] as full.
    pub fn fill(&mut self) {
        #[cfg(feature = "debug")]
        crate::lgp!("Filling ast {self}");
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
    #[must_use]
    pub fn into_box(self) -> Box<Self> {
        Box::new(self)
    }

    /// Takes the attributes from inside self it is a type;
    #[must_use]
    pub fn into_type(self) -> Option<Vec<Attribute>> {
        if let Self::Variable(var) = self {
            var.into_type()
        } else {
            None
        }
    }

    /// Checks if an [`Ast`] is empty
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, &Self::Empty)
    }

    /// Returns the full location of the [`Ast`]
    #[must_use]
    #[expect(clippy::todo, reason = "todo")] // TODO:
    pub fn location(&self) -> ErrorLocation {
        match self {
            Self::Binary(_) => todo!(),
            Self::BracedBlock(_) => todo!(),
            Self::Cast(_) => todo!(),
            Self::ControlFlow(_) => todo!(),
            Self::Empty => todo!(),
            Self::FunctionArgsBuild(_) => todo!(),
            Self::FunctionCall(_) => todo!(),
            Self::Leaf(lit) => lit.as_location().clone(),
            Self::ListInitialiser(_) => todo!(),
            Self::ParensBlock(_) => todo!(),
            Self::Ternary(_) => todo!(),
            Self::Unary(Unary { arg, op }) => arg.location().into_extended(op.as_location()),
            Self::Variable(_) => todo!(),
        }
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
    pub(crate) fn push_braced_block(&mut self, braced_block: BracedBlock) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf_in(&braced_block, "braced", self, "ast");
        match self {
            Self::BracedBlock(BracedBlock { elts, full: false }) => {
                if let Some(last_mut) = elts.last_mut() {
                    if let Self::ControlFlow(ctrl) = last_mut
                        && !ctrl.is_full()
                    {
                        ctrl.push_block_as_leaf(Self::BracedBlock(braced_block))?;
                    } else if let Self::Variable(var) = last_mut
                        && let Some((keyword, name)) = var.as_partial_typedef()
                    {
                        *last_mut =
                            Self::ControlFlow(keyword.to_control_flow(name, Some(braced_block)));
                    } else if let Self::FunctionCall(FunctionCall {
                        function_body: body @ None,
                        ..
                    }) = last_mut
                    {
                        *body = Some(braced_block);
                        elts.push(Self::Empty);
                    } else if last_mut.is_empty() {
                        *last_mut = Self::BracedBlock(braced_block);
                    } else {
                        return Err("Found 2 successive literals, missing semi-colon.".to_owned());
                    }
                } else {
                    elts.push(Self::BracedBlock(braced_block));
                }
            }
            Self::ControlFlow(ctrl) if !ctrl.is_full() =>
                ctrl.push_block_as_leaf(Self::BracedBlock(braced_block))?,
            Self::Empty => *self = Self::BracedBlock(braced_block),
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
                unreachable!("Trying to push block {braced_block} in {self}")
            }
        }
        Ok(())
    }
}
