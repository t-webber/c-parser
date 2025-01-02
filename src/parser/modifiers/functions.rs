//! Module that modifies [`FunctionCall`] within an existing node.

use core::mem;

use super::super::types::binary::Binary;
use super::super::types::braced_blocks::BracedBlock;
use super::super::types::literal::Literal;
use super::super::types::unary::Unary;
use super::super::types::{Ast, FunctionCall, FunctionOperator, ListInitialiser};
use crate::parser::types::ternary::Ternary;

/// Checks if it is possible to create a function from the last
/// [`Literal::Variable`].
pub fn can_make_function(current: &mut Ast) -> bool {
    get_last_variable(current).is_some()
}

/// Returns the last variable of the [`Ast`].
fn get_last_variable(current: &mut Ast) -> Option<&mut Ast> {
    match current {
        //
        //
        // success
        Ast::Leaf(Literal::Variable(_)) => Some(current),
        //
        //
        // failure
        Ast::Empty
        | Ast::Leaf(_)
        | Ast::ControlFlow(_)
        | Ast::ParensBlock(_)
        | Ast::BracedBlock(BracedBlock { full: true, .. })
        | Ast::Ternary(Ternary { failure: None, .. })
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(ListInitialiser { full: true, .. }) => None,
        //
        //
        // recurse
        // operators
        Ast::Unary(Unary { arg: child, .. })
        | Ast::Binary(Binary { arg_r: child, .. })
        | Ast::Ternary(Ternary {
            failure: Some(child),
            ..
        }) => get_last_variable(child),
        // lists
        Ast::FunctionArgsBuild(vec)
        | Ast::ListInitialiser(ListInitialiser { elts: vec, .. })
        | Ast::BracedBlock(BracedBlock { elts: vec, .. }) => {
            vec.last_mut().and_then(get_last_variable)
        }
    }
}

/// Tries to create a function from the last [`Literal::Variable`].
pub fn make_function(current: &mut Ast, arguments: Vec<Ast>) {
    if let Some(ast) = get_last_variable(current) {
        if let Ast::Leaf(Literal::Variable(variable)) = mem::take(ast) {
            *ast = Ast::FunctionCall(FunctionCall {
                variable,
                op: FunctionOperator,
                args: arguments,
            });
        } else {
            panic!("never happens: apply_last_variable only returns var")
        }
    } else {
        panic!("never happens: can_make_function checked")
    }
}
