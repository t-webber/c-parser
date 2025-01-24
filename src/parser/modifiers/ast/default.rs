//! Implements the methods of [`Ast`]
//!
//! These methods are used for the implementation of
//! [`Push`](crate::parser::modifiers::push::Push) for [`Ast`], but also to
//! simplify the api of [`Ast`].

use super::can_push::{AstPushContext, CanPush as _};
use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::modifiers::push::Push as _;
use crate::parser::types::binary::{Binary, BinaryOperator};
use crate::parser::types::braced_blocks::BracedBlock;
use crate::parser::types::parens::Cast;
use crate::parser::types::ternary::Ternary;
use crate::parser::types::unary::{Unary, UnaryOperator};
use crate::parser::types::variable::Variable;
use crate::parser::types::variable::traits::VariableConversion as _;
use crate::parser::types::{Ast, ListInitialiser};

impl Ast {
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
            | Self::Ternary(
                Ternary {
                    failure: Some(arg), ..
                }
                | Ternary { success: arg, .. },
            ) => arg.fill(),
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

    /// Checks if the last element is a leaf, and another attribute/name can be
    /// pushed.
    fn is_var_and<F: Fn(&Variable) -> bool>(&self, check: F) -> bool {
        match self {
            Self::Variable(var) => check(var),
            Self::Binary(Binary {
                op: BinaryOperator::Multiply | BinaryOperator::Comma,
                arg_r: arg,
                ..
            })
            | Self::Unary(Unary {
                arg,
                op: UnaryOperator::Indirection,
            }) => arg.is_var_and(check),

            Self::BracedBlock(BracedBlock { elts, full: false }) => {
                elts.last().is_some_and(|last| last.is_var_and(check))
            }
            Self::ControlFlow(ctrl) if !ctrl.is_full() => {
                ctrl.get_ast().is_some_and(|last| last.is_var_and(check))
            }
            Self::Empty
            | Self::Cast(_)
            | Self::Leaf(_)
            | Self::Unary(_)
            | Self::Binary(_)
            | Self::Ternary(_)
            | Self::ParensBlock(_)
            | Self::ControlFlow(_)
            | Self::BracedBlock(_)
            | Self::FunctionCall(_)
            | Self::ListInitialiser(_)
            | Self::FunctionArgsBuild(_) => false,
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
        crate::errors::api::Print::push_in_vec(&node, vec, "vec");
        if let Some(last) = vec.last_mut() {
            let ctx = if matches!(node, Self::Variable(_)) {
                AstPushContext::UserVariable
            } else if let Self::Binary(Binary { op, .. }) = node
                && (op == BinaryOperator::Comma && last.is_var_and(|_| true)
                    || op == BinaryOperator::Multiply && last.is_var_and(|var| !var.has_eq()))
            {
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
                        && let Some((keyword, name)) = var.get_partial_typedef()
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
