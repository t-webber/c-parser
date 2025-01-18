//! Handlers to be called when a symbol can represent by multiple operator.

use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::keyword::control_flow::types::colon_ast::ColonAstCtrl;
use crate::parser::modifiers::list_initialiser::apply_to_last_list_initialiser;
use crate::parser::modifiers::make_lhs::try_apply_comma_to_variable;
use crate::parser::modifiers::push::Push as _;
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
        Ast::Ternary(Ternary {
            failure: failure @ None,
            ..
        }) => {
            *failure = Some(Ast::empty_box());
            Ok(())
        }
        // label
        Ast::Variable(var) => var.take_user_defined().map_or_else(
            || Err("Invalid label name or missing '?'".to_owned()),
            |name| {
                *current = ColonAstCtrl::from_label_with_colon(name);
                Ok(())
            },
        ),
        Ast::Empty
        | Ast::Leaf(_)
        | Ast::ParensBlock(_)
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(ListInitialiser { full: true, .. })
        | Ast::BracedBlock(BracedBlock { full: true, .. }) => {
            Err("Ternary symbol mismatched: found a ':' symbol without '?'.".to_owned())
        }
        Ast::Unary(Unary { arg, .. })
        | Ast::Binary(Binary { arg_r: arg, .. })
        | Ast::Ternary(Ternary {
            failure: Some(arg), ..
        }) => handle_colon(arg),
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
        Ast::ControlFlow(ctrl) => {
            if ctrl.push_colon() {
                Ok(())
            } else {
                Err(
                    "Found extra ':': Tried to push colon in a control flow that wasn't expecting one.".to_owned(),
                )
            }
        }
        Ast::Cast(cast) => {
            if cast.full {
                Err("Found extra ':': colon is illegal for cast.".to_owned())
            } else {
                handle_colon(&mut cast.value)
            }
        }
    }
}

/// Handler to push a comma into an [`Ast`]
pub fn handle_comma(current: &mut Ast) -> Result<(), String> {
    if let Ast::FunctionArgsBuild(vec) = current {
        vec.push(Ast::Empty);
    } else if apply_to_last_list_initialiser(current, &|vec, _| vec.push(Ast::Empty)).is_none()
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
