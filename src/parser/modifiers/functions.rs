//! Module that modifies [`FunctionCall`] within an existing node.

use core::mem;

use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::operators::api::{Binary, Ternary, Unary};
use crate::parser::symbols::api::{BracedBlock, FunctionCall, ListInitialiser};
use crate::parser::tree::Ast;

/// Returns the last variable of the [`Ast`].
///
/// This function is used to try and find out if a parenthesis is meant as a
/// function call or not. In order to do that, we try and get the last variable
/// in the [`Ast`] that could be a function name.
fn as_last_variable(current: &mut Ast) -> Option<&mut Ast> {
    #[cfg(feature = "debug")]
    crate::errors::api::Print::custom_print(&format!("get last var of {current}"));
    match current {
        Ast::Variable(_) => Some(current),
        // note: functions cannot be declared with casts
        Ast::Empty
        | Ast::Cast(_)
        | Ast::Leaf(_)
        | Ast::ParensBlock(_)
        | Ast::BracedBlock(BracedBlock { full: true, .. })
        | Ast::Ternary(Ternary { failure: None, .. })
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(ListInitialiser { full: true, .. }) => None,
        Ast::Unary(Unary { arg: child, .. })
        | Ast::Binary(Binary { arg_r: child, .. })
        | Ast::Ternary(Ternary { failure: Some(child), .. }) => as_last_variable(child),
        Ast::FunctionArgsBuild(vec)
        | Ast::ListInitialiser(ListInitialiser { elts: vec, .. })
        | Ast::BracedBlock(BracedBlock { elts: vec, .. }) =>
            vec.last_mut().and_then(as_last_variable),
        Ast::ControlFlow(ctrl) => ctrl.as_ast_mut().and_then(as_last_variable),
    }
}

/// Checks if it is possible to create a function from the last
/// [`Variable`](crate::parser::variable::Variable).
pub fn can_make_function(current: &mut Ast) -> bool {
    as_last_variable(current).is_some()
}

/// Tries to create a function from the last
/// [`Variable`](crate::parser::variable::Variable).
pub fn make_function(current: &mut Ast, arguments: Vec<Ast>) {
    if let Some(ast) = as_last_variable(current) {
        if let Ast::Variable(variable) = mem::take(ast) {
            *ast = Ast::FunctionCall(FunctionCall { variable, args: arguments });
        } else {
            panic!("never happens: apply_last_variable only returns var")
        }
    } else {
        panic!("never happens: can_make_function checked")
    }
}
