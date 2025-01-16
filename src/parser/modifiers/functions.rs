//! Module that modifies [`FunctionCall`] within an existing node.

use core::mem;

use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::types::binary::Binary;
use crate::parser::types::braced_blocks::BracedBlock;
use crate::parser::types::ternary::Ternary;
use crate::parser::types::unary::Unary;
use crate::parser::types::{Ast, FunctionCall, FunctionOperator, ListInitialiser};

/// Checks if it is possible to create a function from the last
/// [`Variable`](crate::parser::types::variable::Variable).
pub fn can_make_function(current: &mut Ast) -> bool {
    get_last_variable(current).is_some()
}

/// Returns the last variable of the [`Ast`].
fn get_last_variable(current: &mut Ast) -> Option<&mut Ast> {
    #[cfg(feature = "debug")]
    crate::errors::api::Print::custom_print(&format!("get last var of {current:?}"));
    match current {
        //
        //
        // success
        Ast::Variable(_) => Some(current),
        //
        //
        // failure
        Ast::Empty
        | Ast::Leaf(_)
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
        Ast::ControlFlow(ctrl) => ctrl.get_mut().and_then(get_last_variable),
    }
}

/// Tries to create a function from the last
/// [`Variable`](crate::parser::types::variable::Variable).
pub fn make_function(current: &mut Ast, arguments: Vec<Ast>) {
    if let Some(ast) = get_last_variable(current) {
        if let Ast::Variable(variable) = mem::take(ast) {
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
