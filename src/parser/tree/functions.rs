use core::mem;

use super::ast::Ast;
use super::binary::Binary;
use super::blocks::Block;
use super::literal::Literal;
use super::unary::Unary;
use super::{FunctionCall, FunctionOperator, ListInitialiser, Ternary};

/// Tries to conclude the arguments of a [`FunctionCall`].
///
/// This method is called when `)`. It tries to make the [`FunctionCall`]
/// the nearest to the leaves a full [`FunctionCall`].
///
/// # Returns
///  - `true` if the function was
pub fn try_close_function(current: &mut Ast) -> bool {
    match current {
        //
        //
        // success
        Ast::FunctionCall(FunctionCall {
            full: full @ false,
            args,
            ..
        }) => {
            if !args.last_mut().is_some_and(try_close_function) {
                *full = true;
            }
            true
        }
        //
        //
        // failure
        Ast::Empty
        | Ast::Leaf(_)
        | Ast::ParensBlock(_)
        | Ast::Block(Block { full: true, .. })
        | Ast::Ternary(Ternary { failure: None, .. })
        | Ast::FunctionCall(FunctionCall { full: true, .. })
        | Ast::ListInitialiser(ListInitialiser { full: true, .. }) => false,
        //
        //
        // recurse
        // operators
        Ast::Unary(Unary { arg, .. })
        | Ast::Binary(Binary { arg_r: arg, .. })
        | Ast::Ternary(Ternary {
            failure: Some(arg), ..
        }) => try_close_function(arg),
        // list
        Ast::ListInitialiser(ListInitialiser { elts: vec, .. })
        | Ast::Block(Block { elts: vec, .. }) => vec.last_mut().is_some_and(try_close_function),
    }
}

/// Tries to create a function from the last [`Literal::Variable`].
///
/// # Returns
///  - `true` if the function was created
///  - `false` if the node wasn't full, and the creation failed.
pub fn try_make_function(current: &mut Ast) -> bool {
    match current {
        //
        //
        // success
        Ast::Leaf(Literal::Variable(var)) => {
            *current = Ast::FunctionCall(FunctionCall {
                variable: mem::take(var),
                op: FunctionOperator,
                args: vec![],
                full: false,
            });
            true
        }
        //
        //
        // failure
        Ast::Empty
        | Ast::Leaf(_)
        | Ast::ParensBlock(_)
        | Ast::Block(Block { full: true, .. })
        | Ast::Ternary(Ternary { failure: None, .. })
        | Ast::FunctionCall(FunctionCall { full: true, .. })
        | Ast::ListInitialiser(ListInitialiser { full: true, .. }) => false,
        //
        //
        //
        // recurse
        // operators
        Ast::Unary(Unary { arg, .. })
        | Ast::Binary(Binary { arg_r: arg, .. })
        | Ast::Ternary(Ternary {
            failure: Some(arg), ..
        }) => try_make_function(arg),
        // lists
        Ast::FunctionCall(FunctionCall { args: vec, .. })
        | Ast::ListInitialiser(ListInitialiser { elts: vec, .. })
        | Ast::Block(Block { elts: vec, .. }) => vec.last_mut().is_some_and(try_make_function),
    }
}
