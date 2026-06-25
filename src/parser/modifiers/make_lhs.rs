//! Functions to make an [`Ast`] a LHS.
//!
//! A LHS (left-hand-side) is a node that is on the left-hand-sign of an
//! assignment, or in the arguments of a function declaration.
//!
//! They can be type declaration with attributes, or expressions with
//! assignments.

use core::mem;

use crate::errors::api::{ErrorLocation, Located};
use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::literal::Attribute;
use crate::parser::operators::api::{
    Binary, BinaryOperator, Operator as _, Ternary, Unary, UnaryOperator
};
use crate::parser::symbols::api::BracedBlock;
use crate::parser::tree::Ast;
use crate::parser::tree::api::PushAttribute as _;
use crate::parser::variable::api::VariableConversion as _;

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
        | Ast::FunctionArgsBuild(..) => false,
        Ast::Binary(Binary { arg_l, arg_r, .. }) => has_attributes(arg_l) || has_attributes(arg_r),
        Ast::Ternary(Ternary { condition, failure, success }) =>
            has_attributes(condition)
                || failure.as_ref().is_some_and(|node| has_attributes(&node.1))
                || has_attributes(success),
        Ast::Unary(Unary { arg, .. }) => has_attributes(arg),
    }
}

/// Checks if the operator is valid in a LHS.
fn is_valid_lhs_bin(op: &Located<BinaryOperator>) -> bool {
    op.is_array_subscript() || op.as_star().is_some()
}

/// Checks if the operator is valid in a LHS.
fn is_valid_lhs_un(op: &Located<UnaryOperator>) -> bool {
    op.as_star().is_some()
}

/// Make an [`Ast`] a LHS node
///
/// This is called when an assign
/// [`Operator`](crate::parser::operators::api::Operator) is created or a
/// function is created, to convert `*` to a type attribute. It also check that
/// the [`Ast`] is a valid LHS.
pub fn make_lhs(current: &mut Ast) -> Result<(), String> {
    if has_attributes(current) {
        /* LHS is a declaration */
        make_lhs_aux(current, None)
    } else {
        /* LHS is an expression */
        Ok(())
    }
}

/// Used for recursion, with `push_indirection` indicating on whether a `*` was
/// found previously and needs to be pushed. See [`make_lhs`] for more details.
fn make_lhs_aux(current: &mut Ast, push_indirection: Option<ErrorLocation>) -> Result<(), String> {
    #[cfg(feature = "debug")]
    crate::lgp!("Making {current} LHS with * = {}", push_indirection.is_some());
    let make_error = |val: &str| {
        Err(format!("LHS: expected a declaration or a modifiable lvalue, found {val}."))
    };

    match current {
        Ast::Variable(var) =>
            push_indirection.map_or(Ok(()), |location| var.push_indirection(location)),
        // can't be declaration: finished
        Ast::Binary(Binary { op, .. })
            if matches!(
                op.as_value(),
                BinaryOperator::StructEnumMemberAccess
                    | BinaryOperator::StructEnumMemberPointerAccess
            ) =>
            Ok(()),
        Ast::Unary(Unary { op, arg }) =>
            if let Some(loc) = op.as_star() {
                arg.add_attribute_to_left_variable(vec![loc.wrap(Attribute::Indirection)])?;
                *current = mem::take(arg);
                Ok(())
            } else {
                unreachable!("operator not rejected but illegal")
            },
        Ast::Binary(Binary { op, arg_l, arg_r }) =>
            if let Some(loc) = op.as_star() {
                make_lhs_aux(arg_l, push_indirection)?;
                if let Ast::Variable(old_var) = *mem::take(arg_l) {
                    let mut attrs = old_var.into_attrs()?;
                    attrs.push(loc.wrap(Attribute::Indirection));
                    arg_r.add_attribute_to_left_variable(attrs)?;
                    *current = mem::take(arg_r);
                    Ok(())
                } else {
                    make_error("both")
                }
            } else if op.is_array_subscript() {
                make_lhs_aux(arg_l, push_indirection)
            } else {
                unreachable!("operator not rejected but illegal")
            },
        Ast::Empty
        | Ast::Cast(_)
        | Ast::Leaf(_)
        | Ast::BracedBlock(_)
        | Ast::ParensBlock(_)
        | Ast::ControlFlow(_)
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(_)
        | Ast::FunctionArgsBuild(..) => unreachable!("lhs check returned false"),
        Ast::Ternary(_) => {
            unreachable!("operator not rejected but illegal")
        }
    }
}

/// Tries to find a variable declaration and pushes a comma
pub fn try_apply_comma_to_variable(current: &mut Ast) -> Result<bool, String> {
    match current {
        Ast::Unary(Unary { arg, op }) if is_valid_lhs_un(op) => try_apply_comma_to_variable(arg),
        Ast::Binary(Binary { arg_r: arg, op, .. }) if is_valid_lhs_bin(op) =>
            try_apply_comma_to_variable(arg),
        Ast::BracedBlock(BracedBlock { elts, full: false, .. }) => elts
            .last_mut()
            .map_or(Ok(false), try_apply_comma_to_variable),
        Ast::Variable(var) => Ok(var.push_comma()),
        Ast::ControlFlow(ctrl) => ctrl
            .as_ast_mut()
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
        | Ast::FunctionArgsBuild(..)
        | Ast::BracedBlock(BracedBlock { full: true, .. }) => Ok(false),
    }
}
