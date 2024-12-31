use super::super::types::binary::{Binary, BinaryOperator};
use super::super::types::blocks::Block;
use super::super::types::unary::Unary;
use super::super::types::{Ast, FunctionCall, ListInitialiser};
use crate::parser::types::ternary::Ternary;

/// Applies a closure to the current [`ListInitialiser`].
///
/// It applies the closure somewhere in the [`Ast`]. If this closure
/// returns a value, it is returns in `Ok(_)`. If no list initialiser is
/// found, `Err(())` is returned.
///
/// In the case of nested [`ListInitialiser`]s, the closure is applied to
/// the one closest from the leaves.
#[expect(clippy::min_ident_chars)]
pub fn apply_to_last_list_initialiser<T, F: Fn(&mut Vec<Ast>, &mut bool) -> T>(
    ast: &mut Ast,
    f: &F,
) -> Result<T, ()> {
    match ast {
            //
            //
            // success
            Ast::ListInitialiser(ListInitialiser {
                elts,
                full: full @ false,
            }) => {
                if let Some(last) = elts.last_mut() {
                    if let res @ Ok(_) = apply_to_last_list_initialiser(last, f) {
                        return res;
                    }
                }
                Ok(f(elts, full))
            }
            //
            //
            // failure
            // atomic
            Ast::Empty
            | Ast::Leaf(_)
            | Ast::ControlFlow(_)
            | Ast::ParensBlock(_)
            // full lists
            | Ast::Block(Block{full: true, ..})
            | Ast::FunctionCall(FunctionCall{full: true, ..})
            | Ast::ListInitialiser(ListInitialiser{full: true, ..}) => Err(()),
            //
            //
            // recurse
            Ast::Unary(Unary { arg, .. })
            | Ast::Binary(Binary { arg_r: arg, .. })
            | Ast::Ternary(Ternary { failure: Some(arg), .. } | Ternary { condition: arg, .. }) => {
                apply_to_last_list_initialiser(arg, f)
            }
            //
            //
            // try recurse on non-full lists
            Ast::FunctionCall(FunctionCall {
                full: false, args: vec, ..
            })
            | Ast::Block(Block { elts: vec, full: false }) => vec
                .last_mut()
                .map_or(Err(()), |node| apply_to_last_list_initialiser(node, f)),
        }
}

/// Checks if a `{` is meant as a [`ListInitialiser`] or as a [`Block`].
///
/// # Returns
///  - `Ok(true)` if the brace is meant as a list initialiser.
///  - `Ok(false)` if the brace is meant as an opening block symbol.
///  - `Err(op)` if the brace is illegal, because the ast is expecting a valid
///    leaf. `op` is the stringified version of the operator that has an empty
///    child. List initialiser is a valid leaf only for
///    [`BinaryOperator::Assign`] and [`BinaryOperator::Comma`].
pub fn can_push_list_initialiser(ast: &mut Ast) -> Result<bool, String> {
    match ast {
            //
            //
            // can push
            Ast::Binary(Binary {
                op: BinaryOperator::Assign | BinaryOperator::Comma,
                arg_r,
                ..
            }) if **arg_r == Ast::Empty => Ok(true),
            //
            Ast::FunctionCall(FunctionCall {
                full: false, args: vec, ..
            })
            | Ast::ListInitialiser(ListInitialiser { full: false, elts: vec })
                if vec.last().is_none_or(|node| *node == Ast::Empty) =>
            {
                Ok(true)
            }
            //
            //
            // empty: can't push
            Ast::Block(Block { elts, .. }) if elts.last().is_none_or(|node| *node == Ast::Empty) => Ok(false),
            //
            Ast::Empty
            // full: can't push
            | Ast::Leaf(_)
            | Ast::ControlFlow(_)
            | Ast::ParensBlock(_)
            | Ast::Block(Block { full: true, .. })
            | Ast::ListInitialiser(ListInitialiser { full: true, .. })
            | Ast::FunctionCall(FunctionCall { full: true, .. }) => Ok(false),
            //
            //
            // illegal leaf: can't push
            Ast::Unary(Unary { op, arg }) if **arg == Ast::Empty => Err(op.to_string()),
            Ast::Binary(Binary { op, arg_r, .. }) if **arg_r == Ast::Empty => Err(op.to_string()),
            //
            //
            // recurse
            Ast::Unary(Unary { arg, .. })
            | Ast::Binary(Binary { arg_r: arg,  .. })
            | Ast::Ternary(Ternary { failure: Some(arg), .. } | Ternary { success: arg, .. }) => {
                can_push_list_initialiser(arg)
            }
            //
            //
            // lists
            Ast::Block(Block { elts: vec, full: false })
            | Ast::FunctionCall(FunctionCall { args: vec, full: false, .. })
            | Ast::ListInitialiser(ListInitialiser { elts: vec, full: false }) => {
                vec.last_mut().map_or( Ok(false), can_push_list_initialiser)
            }
        }
}
