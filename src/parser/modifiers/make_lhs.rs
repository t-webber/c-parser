//! Functions to make an [`Ast`] a LHS.
//!
//! A LHS (left-hand-side) is a node that is on the left-hand-sign of an
//! assignment, or in the arguments of a function declaration.
//!
//! They can be type declaration with attributes, or expressions with
//! assignments.

use core::mem;

use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::types::Ast;
use crate::parser::types::binary::{Binary, BinaryOperator};
use crate::parser::types::braced_blocks::BracedBlock;
use crate::parser::types::literal::Attribute;
use crate::parser::types::ternary::Ternary;
use crate::parser::types::unary::{Unary, UnaryOperator};

/// Checks if the current [`Ast`] has a variable with attributes.
///
/// If it is the case, an expression is not allowed in the LHS because it is a
/// type declaration.
fn has_attributes(current: &Ast) -> bool {
    match current {
        Ast::Variable(var) => !var.has_empty_attrs(),
        Ast::Empty
        | Ast::Cast(_)
        | Ast::Leaf(_)
        | Ast::BracedBlock(_)
        | Ast::ParensBlock(_)
        | Ast::ControlFlow(_)
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(_)
        | Ast::FunctionArgsBuild(_) => false,
        Ast::Binary(Binary { arg_l, arg_r, .. }) => has_attributes(arg_l) || has_attributes(arg_r),
        Ast::Ternary(Ternary {
            condition,
            failure,
            success,
            ..
        }) => {
            has_attributes(condition)
                || failure.as_ref().is_some_and(|node| has_attributes(node))
                || has_attributes(success)
        }
        Ast::Unary(Unary { arg, .. }) => has_attributes(arg),
    }
}

/// Checks if the operator is valid in a LHS.
const fn is_valid_lhs_bin(op: BinaryOperator) -> bool {
    matches!(
        op,
        BinaryOperator::Multiply | BinaryOperator::ArraySubscript
    )
}

/// Checks if the operator is valid in a LHS.
const fn is_valid_lhs_un(op: UnaryOperator) -> bool {
    matches!(op, UnaryOperator::Indirection)
}

/// Make an [`Ast`] a LHS node
///
/// This is called when an assign
/// [`Operator`](crate::parser::types::operator::Operator) is created or a
/// function is created, to convert `*` to a type attribute. It also check that
/// the [`Ast`] is a valid LHS.
pub fn make_lhs(current: &mut Ast) -> Result<(), String> {
    if has_attributes(current) {
        /* LHS is a declaration */
        make_lhs_aux(current, false)
    } else {
        /* LHS is an expression */
        Ok(())
    }
}

/// Used for recursion, with `push_indirection` indicating on whether a `*` was
/// found previously and needs to be pushed. See [`make_lhs`] for more details.
fn make_lhs_aux(current: &mut Ast, push_indirection: bool) -> Result<(), String> {
    #[cfg(feature = "debug")]
    crate::errors::api::Print::custom_print(&format!(
        "Making {current} LHS with * = {push_indirection}"
    ));
    let make_error = |val: &str| {
        Err(format!(
            "LHS: expected a declaration or a modifiable lvalue, found {val}."
        ))
    };

    match current {
        Ast::Variable(var) => {
            if push_indirection {
                var.push_indirection()
            } else {
                Ok(())
            }
        }
        // can't be declaration: finished
        Ast::Binary(Binary {
            op:
                BinaryOperator::StructEnumMemberAccess | BinaryOperator::StructEnumMemberPointerAccess,
            ..
        }) => Ok(()),
        Ast::Unary(Unary {
            op: UnaryOperator::Indirection,
            arg,
        }) => {
            arg.add_attribute_to_left_variable(vec![Attribute::Indirection])?;
            *current = mem::take(arg);
            Ok(())
        }
        Ast::Binary(Binary {
            op: BinaryOperator::Multiply,
            arg_l,
            arg_r,
        }) => {
            make_lhs_aux(arg_l, push_indirection)?;
            if let Ast::Variable(old_var) = *mem::take(arg_l) {
                let mut attrs = old_var.into_attrs()?;
                attrs.push(Attribute::Indirection);
                arg_r.add_attribute_to_left_variable(attrs)?;
                *current = mem::take(arg_r);
                Ok(())
            } else {
                make_error("both")
            }
        }
        Ast::Binary(Binary {
            op: BinaryOperator::ArraySubscript,
            arg_l,
            ..
        }) => make_lhs_aux(arg_l, push_indirection),
        Ast::Empty
        | Ast::Cast(_)
        | Ast::Leaf(_)
        | Ast::BracedBlock(_)
        | Ast::ParensBlock(_)
        | Ast::ControlFlow(_)
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(_)
        | Ast::FunctionArgsBuild(_) => panic!("lhs check returned false"),
        Ast::Unary(_) | Ast::Binary(_) | Ast::Ternary(_) => {
            panic!("operator not rejected but illegal")
        }
    }
}

/// Tries to find a variable declaration and pushes a comma
pub fn try_apply_comma_to_variable(current: &mut Ast) -> Result<bool, String> {
    match current {
        Ast::Unary(Unary { arg, op }) if is_valid_lhs_un(*op) => try_apply_comma_to_variable(arg),
        Ast::Binary(Binary { arg_r: arg, op, .. }) if is_valid_lhs_bin(*op) => {
            try_apply_comma_to_variable(arg)
        }
        Ast::BracedBlock(BracedBlock { elts, full: false }) => elts
            .last_mut()
            .map_or(Ok(false), try_apply_comma_to_variable),
        Ast::Variable(var) => Ok(var.push_comma()),
        Ast::ControlFlow(ctrl) => ctrl
            .get_mut()
            .map_or(Ok(false), try_apply_comma_to_variable),
        Ast::Empty
        | Ast::Leaf(_)
        | Ast::Cast(_)
        | Ast::Unary(_)
        | Ast::Binary(_)
        | Ast::Ternary(_)
        | Ast::ParensBlock(_)
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(_)
        | Ast::FunctionArgsBuild(_)
        | Ast::BracedBlock(BracedBlock { full: true, .. }) => Ok(false),
    }
}
