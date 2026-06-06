//! Implements the method for pushing in and looking at an [`Ast`].

#![expect(clippy::inline_modules, reason = "clearer api")]
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
use crate::errors::api::ErrorLocation;
use crate::utils::display;

/// Struct to represent the Abstract Syntax Tree of the whole C source file.
///
/// # Note
///
/// Can't derive [`Eq`] because it is not implemented for [`f32`].
#[derive(Default, Debug)]
pub struct Ast {
    /// Location of the AST
    pub location: Option<ErrorLocation>, // TODO: remove Option when implementation complete.
    /// Value of the AST
    pub value: AstValue,
}

// TODO: remove this when implementation complete.
impl From<AstValue> for Ast {
    fn from(value: AstValue) -> Self {
        Self { location: None, value }
    }
}

display!(Ast, self, f, self.value.fmt(f));

/// Struct to represent the Abstract Syntax Tree of the whole C source file.
///
/// # Note
///
/// Can't derive [`Eq`] because it is not implemented for [`f32`].
#[non_exhaustive]
#[derive(Debug, Default)]
pub enum AstValue {
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

impl AstValue {
    /// Converts an [`AstValue`] to an [`Ast`] with a given error location.
    pub const fn with_location(self, location: ErrorLocation) -> Ast {
        Ast { location: Some(location), value: self }
    }
}

display!(
    AstValue,
    self,
    f,
    match self {
        AstValue::Empty => EMPTY.fmt(f),
        AstValue::Cast(cast) => cast.fmt(f),
        AstValue::Unary(val) => val.fmt(f),
        AstValue::Leaf(val) => val.fmt(f),
        AstValue::Binary(val) => val.fmt(f),
        AstValue::Ternary(val) => val.fmt(f),
        AstValue::Variable(var) => var.fmt(f),
        AstValue::FunctionCall(val) => val.fmt(f),
        AstValue::BracedBlock(block) => block.fmt(f),
        AstValue::ParensBlock(parens) => parens.fmt(f),
        AstValue::ControlFlow(ctrl) => ctrl.fmt(f),
        Self::FunctionArgsBuild(vec) => write!(f, "(\u{b0}{})", repr_vec(vec)),
        AstValue::ListInitialiser(list_initialiser) => list_initialiser.fmt(f),
    }
);
