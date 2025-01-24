//! Implements the method for pushing in and looking at an [`Ast`].

pub mod can_push;
mod default;
mod push;

use core::fmt;

use crate::EMPTY;
use crate::parser::repr_vec;
use crate::parser::types::Ast;

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
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
