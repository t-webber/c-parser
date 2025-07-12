//! Module that modifies [`FunctionCall`] within an existing node.

use core::mem;

use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::operators::api::{Binary, Ternary, Unary};
use crate::parser::symbols::api::{BracedBlock, FunctionCall, ListInitialiser};
use crate::parser::tree::Ast;

/// Trait to manipulate node to find and edit [`Variable`]s that can be
/// transformed into functions if a `(` is read.
pub trait AsLastVariable {
    /// Checks if an opening parenthesis at this stage is meant as a function.
    ///
    /// # Returns
    ///
    /// The depth of the variable in the AST that is to be made into a function.
    fn can_make_function(&self) -> Option<u32>;

    /// Makes a function out of the variable found in [`can_make_function`].
    fn make_function(&mut self, depth: u32, arguments: Vec<Ast>);
}

impl AsLastVariable for Ast {
    fn can_make_function(&self) -> Option<u32> {
        match self {
            Self::Variable(variable) => Some(variable.can_make_function().unwrap_or_default()),
            Self::Empty
            | Self::Cast(_)
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            | Self::BracedBlock(BracedBlock { full: true, .. })
            | Self::Ternary(Ternary { failure: None, .. })
            | Self::FunctionCall(_)
            | Self::ListInitialiser(ListInitialiser { full: true, .. }) => None,
            Self::Unary(Unary { arg: child, .. })
            | Self::Binary(Binary { arg_r: child, .. })
            | Self::Ternary(Ternary { failure: Some(child), .. }) =>
                child.can_make_function().map(|depth| depth + 1),
            Self::FunctionArgsBuild(vec)
            | Self::ListInitialiser(ListInitialiser { elts: vec, .. })
            | Self::BracedBlock(BracedBlock { elts: vec, .. }) => vec
                .last()
                .and_then(|last| last.can_make_function().map(|depth| depth + 1)),
            Self::ControlFlow(ctrl) => ctrl
                .as_ast()
                .and_then(|last| last.can_make_function().map(|depth| depth + 1)),
        }
    }

    fn make_function(&mut self, depth: u32, arguments: Vec<Self>) {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::custom_print(&format!("get last var of {self}"));
        if depth == 0 {
            if let Self::Variable(variable) = mem::take(self) {
                *self = Self::FunctionCall(FunctionCall { variable, args: arguments });
                return;
            }
            unreachable!("must be variable at depth 0")
        }
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "must be variable at depth 0"
        )]
        let new_depth = depth - 1;
        match self {
            Self::Variable(var) => var.make_function(new_depth, arguments),
            // note: functions cannot be declared with casts
            Self::Empty
            | Self::Cast(_)
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            | Self::BracedBlock(BracedBlock { full: true, .. })
            | Self::Ternary(Ternary { failure: None, .. })
            | Self::FunctionCall(_)
            | Self::ListInitialiser(ListInitialiser { full: true, .. }) =>
                unreachable!("can_make_function checked"),
            Self::Unary(Unary { arg: child, .. })
            | Self::Binary(Binary { arg_r: child, .. })
            | Self::Ternary(Ternary { failure: Some(child), .. }) =>
                child.make_function(new_depth, arguments),
            Self::FunctionArgsBuild(vec)
            | Self::ListInitialiser(ListInitialiser { elts: vec, .. })
            | Self::BracedBlock(BracedBlock { elts: vec, .. }) => vec
                .last_mut()
                .expect("can_make_function checked")
                .make_function(new_depth, arguments),
            Self::ControlFlow(ctrl) => ctrl
                .as_ast_mut()
                .expect("can_make_function_checked")
                .make_function(new_depth, arguments),
        }
    }
}
