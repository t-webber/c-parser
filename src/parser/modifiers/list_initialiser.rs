//! Module that modifies [`ListInitialiser`] within an existing node.

use crate::parser::api::AstValue;
use crate::parser::operators::api::{Binary, BinaryOperator, Ternary, Unary};
use crate::parser::symbols::api::{BracedBlock, Cast, ListInitialiser};
use crate::parser::tree::Ast;
use crate::parser::variable::api::PureType as _;

/// Applies a closure to the current [`ListInitialiser`].
///
/// It applies the closure somewhere in the [`Ast`]. If this closure
/// returns a value, it is returns in `Ok(_)`. If no list initialiser is
/// found, `Err(())` is returned.
///
/// In the case of nested [`ListInitialiser`]s, the closure is applied to
/// the one closest from the leaves.
pub fn apply_to_last_list_initialiser<T, F>(ast: &mut Ast, visitor: &F) -> Option<T>
where
    F: Fn(&mut Vec<Ast>, &mut bool) -> T,
{
    match &mut ast.value {
        AstValue::ListInitialiser(ListInitialiser { elts, full: full @ false }) => {
            if let Some(last) = elts.last_mut()
                && let res @ Some(_) = apply_to_last_list_initialiser(last, visitor)
            {
                res
            } else {
                Some(visitor(elts, full))
            }
        }

        AstValue::Cast(cast) =>
            if cast.full {
                None
            } else {
                apply_to_last_list_initialiser(&mut cast.value, visitor)
            },
        AstValue::Empty
        | AstValue::Leaf(_)
        | AstValue::Variable(_)
        | AstValue::ControlFlow(_)
        | AstValue::ParensBlock(_)
        | AstValue::FunctionCall(_)
        | AstValue::BracedBlock(BracedBlock { full: true, .. })
        | AstValue::ListInitialiser(ListInitialiser { full: true, .. }) => None,
        AstValue::Unary(Unary { arg, .. })
        | AstValue::Binary(Binary { arg_r: arg, .. })
        | AstValue::Ternary(Ternary { failure: Some(arg), .. } | Ternary { condition: arg, .. }) =>
            apply_to_last_list_initialiser(arg, visitor),
        AstValue::FunctionArgsBuild(vec)
        | AstValue::BracedBlock(BracedBlock { elts: vec, full: false }) => {
            let node = vec.last_mut()?;
            apply_to_last_list_initialiser(node, visitor)
        }
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
    crate::lgp!("Can push list initialiser in {ast}");
    match &mut ast.value {
        AstValue::Empty
        | AstValue::Leaf(_)
        | AstValue::Variable(_)
        | AstValue::ControlFlow(_)
        | AstValue::BracedBlock(BracedBlock { full: true, .. })
        | AstValue::ListInitialiser(ListInitialiser { full: true, .. })
        | AstValue::FunctionCall(_) => Ok(false),
        AstValue::ParensBlock(parens) => Ok(parens.is_pure_type()),
        AstValue::Binary(Binary {
            op: BinaryOperator::Assign | BinaryOperator::Comma,
            arg_r,
            ..
        }) if (*arg_r).is_empty() => Ok(true),
        AstValue::ListInitialiser(ListInitialiser { full: false, elts: vec })
            if vec.last().is_none_or(Ast::is_empty) =>
            Ok(true),
        AstValue::BracedBlock(BracedBlock { elts, .. })
            if elts.last().is_none_or(Ast::is_empty) =>
            Ok(false),
        AstValue::Cast(Cast { full, value, .. }) =>
            if *full {
                Ok(false)
            } else if (*value).is_empty() {
                Ok(true)
            } else {
                can_push_list_initialiser(value)
            },
        AstValue::Unary(Unary { op, arg }) if (*arg).is_empty() => Err(op.to_string()),
        AstValue::Binary(Binary { op, arg_r, .. }) if (*arg_r).is_empty() => Err(op.to_string()),
        AstValue::Unary(Unary { arg, .. })
        | AstValue::Binary(Binary { arg_r: arg, .. })
        | AstValue::Ternary(Ternary { failure: Some(arg), .. } | Ternary { success: arg, .. }) =>
            can_push_list_initialiser(arg),
        AstValue::FunctionArgsBuild(vec)
        | AstValue::BracedBlock(BracedBlock { elts: vec, full: false })
        | AstValue::ListInitialiser(ListInitialiser { elts: vec, full: false }) =>
            vec.last_mut().map_or(Ok(false), can_push_list_initialiser),
    }
}
