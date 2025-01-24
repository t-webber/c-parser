//! Module that modifies [`ListInitialiser`] within an existing node.

use crate::parser::types::binary::{Binary, BinaryOperator};
use crate::parser::types::braced_blocks::BracedBlock;
use crate::parser::types::parens::Cast;
use crate::parser::types::ternary::Ternary;
use crate::parser::types::unary::Unary;
use crate::parser::types::variable::traits::PureType as _;
use crate::parser::types::{Ast, ListInitialiser};

/// Applies a closure to the current [`ListInitialiser`].
///
/// It applies the closure somewhere in the [`Ast`]. If this closure
/// returns a value, it is returns in `Ok(_)`. If no list initialiser is
/// found, `Err(())` is returned.
///
/// In the case of nested [`ListInitialiser`]s, the closure is applied to
/// the one closest from the leaves.
#[expect(clippy::min_ident_chars)]
pub fn apply_to_last_list_initialiser<T, F>(ast: &mut Ast, f: &F) -> Option<T>
where
    F: Fn(&mut Vec<Ast>, &mut bool) -> T,
{
    match ast {
        Ast::ListInitialiser(ListInitialiser {
            elts,
            full: full @ false,
        }) => {
            if let Some(last) = elts.last_mut() {
                if let res @ Some(_) = apply_to_last_list_initialiser(last, f) {
                    return res;
                }
            }
            Some(f(elts, full))
        }
        Ast::Cast(cast) => {
            if cast.full {
                None
            } else {
                apply_to_last_list_initialiser(&mut cast.value, f)
            }
        }
        Ast::Empty
        | Ast::Leaf(_)
        | Ast::Variable(_)
        | Ast::ControlFlow(_)
        | Ast::ParensBlock(_)
        | Ast::FunctionCall(_)
        | Ast::BracedBlock(BracedBlock { full: true, .. })
        | Ast::ListInitialiser(ListInitialiser { full: true, .. }) => None,
        Ast::Unary(Unary { arg, .. })
        | Ast::Binary(Binary { arg_r: arg, .. })
        | Ast::Ternary(
            Ternary {
                failure: Some(arg), ..
            }
            | Ternary { condition: arg, .. },
        ) => apply_to_last_list_initialiser(arg, f),
        Ast::FunctionArgsBuild(vec)
        | Ast::BracedBlock(BracedBlock {
            elts: vec,
            full: false,
        }) => vec
            .last_mut()
            .and_then(|node| apply_to_last_list_initialiser(node, f)),
    }
}

/// Checks if a `{` is meant as a [`ListInitialiser`] or as a [`BracedBlock`].
///
/// # Returns
///  - `Ok(true)` if the brace is meant as a list initialiser.
///  - `Ok(false)` if the brace is meant as an opening block symbol.
///  - `Err(op)` if the brace is illegal, because the ast is expecting a valid
///    leaf. `op` is the stringified version of the operator that has an empty
///    child. List initialiser is a valid leaf only for
///    [`BinaryOperator::Assign`] and [`BinaryOperator::Comma`].
pub fn can_push_list_initialiser(ast: &mut Ast) -> Result<bool, String> {
    #[cfg(feature = "debug")]
    crate::errors::api::Print::custom_print(&format!("Can push list initialiser in {ast}"));
    match ast {
        Ast::Empty
        | Ast::Leaf(_)
        | Ast::Variable(_)
        | Ast::ControlFlow(_)
        | Ast::BracedBlock(BracedBlock { full: true, .. })
        | Ast::ListInitialiser(ListInitialiser { full: true, .. })
        | Ast::FunctionCall(_) => Ok(false),
        Ast::ParensBlock(parens) => Ok(parens.is_pure_type()),
        Ast::Binary(Binary {
            op: BinaryOperator::Assign | BinaryOperator::Comma,
            arg_r,
            ..
        }) if (*arg_r).is_empty() => Ok(true),
        Ast::ListInitialiser(ListInitialiser {
            full: false,
            elts: vec,
        }) if vec.last().is_none_or(Ast::is_empty) => Ok(true),
        Ast::BracedBlock(BracedBlock { elts, .. }) if elts.last().is_none_or(Ast::is_empty) => {
            Ok(false)
        }
        Ast::Cast(Cast { full, value, .. }) => {
            if *full {
                Ok(false)
            } else if (*value).is_empty() {
                Ok(true)
            } else {
                can_push_list_initialiser(value)
            }
        }
        Ast::Unary(Unary { op, arg }) if (*arg).is_empty() => Err(op.to_string()),
        Ast::Binary(Binary { op, arg_r, .. }) if (*arg_r).is_empty() => Err(op.to_string()),
        Ast::Unary(Unary { arg, .. })
        | Ast::Binary(Binary { arg_r: arg, .. })
        | Ast::Ternary(
            Ternary {
                failure: Some(arg), ..
            }
            | Ternary { success: arg, .. },
        ) => can_push_list_initialiser(arg),
        Ast::FunctionArgsBuild(vec)
        | Ast::BracedBlock(BracedBlock {
            elts: vec,
            full: false,
        })
        | Ast::ListInitialiser(ListInitialiser {
            elts: vec,
            full: false,
        }) => vec.last_mut().map_or(Ok(false), can_push_list_initialiser),
    }
}
