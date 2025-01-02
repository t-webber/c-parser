//! Handlers to be called when a symbol can represent by multiple operator.

use super::super::modifiers::list_initialiser::apply_to_last_list_initialiser;
use super::super::types::binary::{Binary, BinaryOperator};
use super::super::types::blocks::BracedBlock;
use super::super::types::unary::{Unary, UnaryOperator};
use super::super::types::{Ast, FunctionCall, ListInitialiser};
use crate::parser::modifiers::functions::try_add_function_argument;
use crate::parser::types::ternary::Ternary;

/// Handler to push a symbol that can be represented by a binary and a unary
/// operator.
pub fn handle_binary_unary(
    current: &mut Ast,
    bin_op: BinaryOperator,
    un_op: UnaryOperator,
) -> Result<(), String> {
    current
        .push_op(bin_op)
        .map_or_else(|_| current.push_op(un_op), |()| Ok(()))
}

/// Adds the colon of a
/// [`TernaryOperator`](super::super::types::ternary::TernaryOperator).
///
/// This method finds a ternary operator, and changes its reading state to
/// failure.
pub fn handle_colon(current: &mut Ast) -> Result<(), String> {
    match current {
        //
        //
        // success
        Ast::Ternary(Ternary {
            failure: failure @ None,
            ..
        }) => {
            *failure = Some(Box::from(Ast::Empty));
            Ok(())
        }
        //
        //
        // failure
        Ast::Empty
        | Ast::Leaf(_)
        | Ast::ParensBlock(_)
        | Ast::FunctionCall(FunctionCall { full: true, .. })
        | Ast::ListInitialiser(ListInitialiser { full: true, .. })
        | Ast::BracedBlock(BracedBlock { full: true, .. }) => {
            Err("Ternary symbol mismatched: found a ':' symbol without '?'.".to_owned())
        }
        //
        //
        // recurse
        // operators
        Ast::Unary(Unary { arg, .. })
        | Ast::Binary(Binary { arg_r: arg, .. })
        | Ast::Ternary(Ternary {
            failure: Some(arg), ..
        }) => handle_colon(arg),
        // lists
        Ast::FunctionCall(FunctionCall {
            full: false,
            args: vec,
            ..
        })
        | Ast::ListInitialiser(ListInitialiser {
            full: false,
            elts: vec,
        })
        | Ast::BracedBlock(BracedBlock {
            elts: vec,
            full: false,
        }) => handle_colon(vec.last_mut().expect("Created with one elt")),
        Ast::ControlFlow(ctrl) => ctrl.push_colon(),
    }
}

/// Handler to push a comma into an [`Ast`]
pub fn handle_comma(current: &mut Ast) -> Result<(), String> {
    if apply_to_last_list_initialiser(current, &|vec, _| vec.push(Ast::Empty)).is_err()
        && !try_add_function_argument(current)
    {
        current.push_op(BinaryOperator::Comma)?;
    }
    Ok(())
}

/// Handler to push a symbol that can be represented by 2 different unary
/// operators.
pub fn handle_double_unary(
    current: &mut Ast,
    first: UnaryOperator,
    second: UnaryOperator,
) -> Result<(), String> {
    current
        .push_op(first)
        .map_or_else(|_| current.push_op(second), |()| Ok(()))
}
