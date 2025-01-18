//! Implements the method for pushing in and looking at an [`Ast`].

mod default;
mod push;

use core::fmt;

use crate::EMPTY;
use crate::parser::repr_vec;
use crate::parser::types::Ast;

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => EMPTY.fmt(f),
            Self::Cast(cast) => cast.fmt(f),
            Self::Unary(val) => val.fmt(f),
            Self::Leaf(val) => val.fmt(f),
            Self::Binary(val) => val.fmt(f),
            Self::Ternary(val) => val.fmt(f),
            Self::Variable(var) => var.fmt(f),
            Self::FunctionCall(val) => val.fmt(f),
            Self::BracedBlock(block) => block.fmt(f),
            Self::ParensBlock(parens) => parens.fmt(f),
            Self::ControlFlow(ctrl) => ctrl.fmt(f),
            Self::FunctionArgsBuild(vec) => write!(f, "(\u{b0}{})", repr_vec(vec)),
            Self::ListInitialiser(list_initialiser) => list_initialiser.fmt(f),
        }
    }
}

/// Context to specify what are we trying to push into the [`Ast`].
///
/// See [`Ast::can_push_leaf`] for more information.
#[derive(Debug, Default, PartialEq, Eq)]
pub enum AstPushContext {
    /// Any context is good
    Any,
    /// Trying to see if an `else` block ca be added
    Else,
    /// Nothing particular
    #[default]
    None,
    /// Trying to see if the last element of the [`Ast`] waiting for variables.
    UserVariable,
}

impl AstPushContext {
    /// Checks if the context can have an `else`
    #[inline]
    const fn is_else(&self) -> bool {
        matches!(self, &Self::Any | &Self::Else)
    }

    /// Checks if the context can have a variable
    #[inline]
    const fn is_user_variable(&self) -> bool {
        matches!(self, &Self::Any | &Self::UserVariable)
    }
}
