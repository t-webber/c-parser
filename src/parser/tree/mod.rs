//! Implements the method for pushing in and looking at an [`Ast`].

pub mod api {
    //! Api module to choose what functions to export.

    #![allow(clippy::pub_use, reason = "expose simple API")]

    pub use super::Ast;
    pub use super::can_push::{AstPushContext, CanPush, PushAttribute};
}

mod can_push;
mod default;
mod push;

use super::display::repr_vec;
use super::keyword::control_flow::node::ControlFlowNode;
use super::literal::Literal;
use super::operators::api::{Binary, Ternary, Unary};
use super::symbols::api::{BracedBlock, Cast, FunctionCall, ListInitialiser, ParensBlock};
use super::variable::Variable;
use crate::EMPTY;
use crate::utils::display;

/// Struct to represent the Abstract Syntax Tree of the whole C source file.
///
/// # Note
///
/// Can't derive [`Eq`] because it is not implemented for [`f32`].
#[derive(Debug, Default)]
pub enum Ast {
    /// Binary operator
    Binary(Binary),
    /// Braced-block, in `{...}`.
    ///
    /// A whole file is considered to be a block.
    BracedBlock(BracedBlock),
    /// Cast
    Cast(Cast),
    /// Control Flow blocks
    ControlFlow(ControlFlowNode),
    /// Empty AST
    #[default]
    Empty,
    /// Function arguments: `(x+y, !g(z), (a, !b)++, )`
    FunctionArgsBuild(Vec<Ast>),
    /// Function call
    FunctionCall(FunctionCall),
    /// Literal (constants, variables, etc.)
    Leaf(Literal),
    /// List initialiser: `{1, 2, 3, [6]=7}`
    ListInitialiser(ListInitialiser),
    /// Ast surrounded by parenthesis: `(x=2)`
    ParensBlock(ParensBlock),
    /// Ternary operator
    Ternary(Ternary),
    /// Unary operator
    Unary(Unary),
    /// Variables
    Variable(Variable),
}

display!(
    Ast,
    self,
    f,
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
);
