//! Handlers to be called when a symbol can represent by multiple operator.

use crate::parser::modifiers::list_initialiser::apply_to_last_list_initialiser;
use crate::parser::modifiers::make_lhs::try_apply_comma_to_variable;
use crate::parser::types::binary::{Binary, BinaryOperator};
use crate::parser::types::braced_blocks::BracedBlock;
use crate::parser::types::ternary::Ternary;
use crate::parser::types::unary::{Unary, UnaryOperator};
use crate::parser::types::{Ast, ListInitialiser};

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
/// [`TernaryOperator`](crate::parser::types::ternary::TernaryOperator).
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
        | Ast::Variable(_)
        | Ast::ParensBlock(_)
        | Ast::FunctionCall(_)
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
        Ast::ListInitialiser(ListInitialiser {
            full: false,
            elts: vec,
        })
        | Ast::BracedBlock(BracedBlock {
            elts: vec,
            full: false,
        })
        | Ast::FunctionArgsBuild(vec) => {
            handle_colon(vec.last_mut().expect("Created with one elt"))
        }
        Ast::ControlFlow(ctrl) => ctrl.push_colon(),
    }
}

/// Handler to push a comma into an [`Ast`]
pub fn handle_comma(current: &mut Ast) -> Result<(), String> {
    if let Ast::FunctionArgsBuild(vec) = current {
        vec.push(Ast::Empty);
    } else if apply_to_last_list_initialiser(current, &|vec, _| vec.push(Ast::Empty)).is_err()
        && !try_apply_comma_to_variable(current)?
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
