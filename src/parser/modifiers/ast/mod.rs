//! Implements the method for pushing in and looking at an [`Ast`].

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
    // /// Gets the context from the last [`Ast`] of a vec and the new [`Ast`] to
    // /// push.
    // const fn from_vec_last_new(vec_last: &Ast, is_var: bool) -> Self {
    //     match vec_last {
    //         Ast::Binary(Binary {
    //             op: BinaryOperator::Multiply,
    //             ..
    //         }) => {
    //             if is_var {
    //                 Self::UserVariable
    //             } else {
    //                 Self::None
    //             }
    //         }
    //         Ast::BracedBlock(braced_block) => todo!(),
    //         Ast::Cast(cast) => todo!(),
    //         Ast::ControlFlow(control_flow_node) => todo!(),
    //         Ast::Empty => todo!(),
    //         Ast::FunctionArgsBuild(vec) => todo!(),
    //         Ast::FunctionCall(function_call) => todo!(),
    //         Ast::Leaf(literal) => todo!(),
    //         Ast::ListInitialiser(list_initialiser) => todo!(),
    //         Ast::ParensBlock(parens_block) => todo!(),
    //         Ast::Ternary(ternary) => todo!(),
    //         Ast::Unary(unary) => todo!(),
    //         Ast::Variable(variable) => todo!(),
    //     }
    // }

    /// Checks if the context can have an `else`
    const fn is_else(&self) -> bool {
        matches!(self, &Self::Any | &Self::Else)
    }

    /// Checks if the context can have a variable
    const fn is_user_variable(&self) -> bool {
        matches!(self, &Self::Any | &Self::UserVariable)
    }
}
