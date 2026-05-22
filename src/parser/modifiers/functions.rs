//! Module that modifies [`FunctionCall`] within an existing node.

use core::convert::Infallible;
use core::mem;
use core::ops::{ControlFlow, FromResidual, Residual, Try};

use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::operators::api::{Binary, Ternary, Unary};
use crate::parser::symbols::api::{BracedBlock, FunctionCall, ListInitialiser};
use crate::parser::tree::Ast;

/// Result of call to `[MakeFunction::can_make_function]`, indicated whether a
/// function can be created from a `(` and a pending variable, and if so, at
/// what depth.
#[derive(Debug)]
pub enum CanMakeFnRes {
    /// A function can be made.
    ///
    /// The depth of the variable that is to be turned into a function is
    /// stored. The depth is the number of variable to descend in the
    /// `[Ast]` before reaching the desired variable.
    CanMakeFn(u32),
    /// No function can be made
    ///
    /// No pending variable was found.
    None,
    /// An error occured whilst incrementing the depth.
    ///
    /// This means that the user has more than 2**32 nested variables.
    TooDeep,
}

impl CanMakeFnRes {
    /// Tries to increment the variable depth if it exists.
    ///
    /// If it doesn't exist it initialises the depth to 0.
    const fn increment_or_default(self) -> Self {
        match self {
            Self::CanMakeFn(depth) => match depth.checked_add(1) {
                Some(new_depth) => Self::CanMakeFn(new_depth),
                None => Self::TooDeep,
            },
            Self::None => Self::CanMakeFn(0),
            Self::TooDeep => Self::TooDeep,
        }
    }
}

impl FromResidual<Self> for CanMakeFnRes {
    fn from_residual(residual: Self) -> Self {
        residual
    }
}

impl FromResidual<Option<Infallible>> for CanMakeFnRes {
    fn from_residual(residual: Option<Infallible>) -> Self {
        residual.map_or(Self::None, |_| unreachable!())
    }
}

impl Residual<Self> for CanMakeFnRes {
    type TryType = Self;
}

impl Try for CanMakeFnRes {
    type Output = Self;

    type Residual = Self;

    fn branch(self) -> ControlFlow<Self, Self> {
        ControlFlow::Continue(self)
    }

    fn from_output(output: Self) -> Self {
        output
    }
}

/// Trait to manipulate node to find and edit [`Variable`]s that can be
/// transformed into functions if a `(` is read.
pub trait MakeFunction {
    /// Checks if an opening parenthesis at this stage is meant as a function.
    ///
    /// # Returns
    ///
    /// The depth of the variable in the AST that is to be made into a function.
    fn can_make_function(&self) -> CanMakeFnRes;

    /// Makes a function out of the variable found in [`can_make_function`].
    fn make_function(&mut self, depth: u32, arguments: Vec<Ast>);
}

impl MakeFunction for Ast {
    fn can_make_function(&self) -> CanMakeFnRes {
        match self {
            Self::Variable(variable) => variable.can_make_function().increment_or_default(),
            Self::Empty
            | Self::Cast(_)
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            | Self::BracedBlock(BracedBlock { full: true, .. })
            | Self::FunctionCall(_)
            | Self::ListInitialiser(ListInitialiser { full: true, .. }) => CanMakeFnRes::None,
            Self::Unary(Unary { arg: child, .. })
            | Self::Binary(Binary { arg_r: child, .. })
            | Self::Ternary(
                Ternary { failure: Some(child), .. }
                | Ternary { failure: None, success: child, .. },
            ) => child.can_make_function(),
            Self::FunctionArgsBuild(vec)
            | Self::ListInitialiser(ListInitialiser { elts: vec, .. })
            | Self::BracedBlock(BracedBlock { elts: vec, .. }) => vec.last()?.can_make_function(),
            Self::ControlFlow(ctrl) => ctrl.as_ast()?.can_make_function(),
        }
    }

    fn make_function(&mut self, depth: u32, arguments: Vec<Self>) {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::custom_print(&format!("get last var of {self}"));

        match self {
            Self::Variable(var) => match depth.checked_sub(1) {
                Some(new_depth) => var.make_function(new_depth, arguments),
                None =>
                    *self = Self::FunctionCall(FunctionCall { arguments, variable: mem::take(var) }),
            },
            Self::Empty
            | Self::Cast(_)
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            | Self::BracedBlock(BracedBlock { full: true, .. })
            | Self::FunctionCall(_)
            | Self::ListInitialiser(ListInitialiser { full: true, .. }) =>
                unreachable!("can_make_function checked"),
            Self::Unary(Unary { arg: child, .. })
            | Self::Binary(Binary { arg_r: child, .. })
            | Self::Ternary(
                Ternary { failure: Some(child), .. } | Ternary { success: child, .. },
            ) => child.make_function(depth, arguments),
            Self::FunctionArgsBuild(vec)
            | Self::ListInitialiser(ListInitialiser { elts: vec, .. })
            | Self::BracedBlock(BracedBlock { elts: vec, .. }) => vec
                .last_mut()
                .expect("can_make_function checked")
                .make_function(depth, arguments),
            Self::ControlFlow(ctrl) => ctrl
                .as_ast_mut()
                .expect("can_make_function_checked")
                .make_function(depth, arguments),
        }
    }
}
