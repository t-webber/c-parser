//! Functions to make an [`Ast`] a LHS.
//!
//! A LHS (left-hand-side) is a node that is on the left-hand-sign of an
//! assignment, or in the arguments of a function declaration.
//!
//! They can be type declaration with attributes, or expressions with
//! assignments.

use core::mem;

use crate::parser::api::AstValue;
use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::literal::Attribute;
use crate::parser::operators::api::{Binary, BinaryOperator, Ternary, Unary, UnaryOperator};
use crate::parser::symbols::api::BracedBlock;
use crate::parser::tree::Ast;
use crate::parser::tree::api::PushAttribute as _;
use crate::parser::variable::api::VariableConversion as _;

/// Checks if the current [`Ast`] has a variable with attributes.
///
/// If it is the case, an expression is not allowed in the LHS because it is a
/// type declaration.
fn has_attributes(current: &Ast) -> bool {
    match &current.value {
        AstValue::Variable(var) => !var.has_empty_attrs(),
        AstValue::Empty
        | AstValue::Cast(_)
        | AstValue::Leaf(_)
        | AstValue::BracedBlock(_)
        | AstValue::ParensBlock(_)
        | AstValue::ControlFlow(_)
        | AstValue::FunctionCall(_)
        | AstValue::ListInitialiser(_)
        | AstValue::FunctionArgsBuild(_) => false,
        AstValue::Binary(Binary { arg_l, arg_r, .. }) =>
            has_attributes(arg_l) || has_attributes(arg_r),
        AstValue::Ternary(Ternary { condition, failure, success }) =>
            has_attributes(condition)
                || failure.as_ref().is_some_and(|node| has_attributes(node))
                || has_attributes(success),
        AstValue::Unary(Unary { arg, .. }) => has_attributes(arg),
    }
}

/// Checks if the operator is valid in a LHS.
const fn is_valid_lhs_bin(op: BinaryOperator) -> bool {
    matches!(op, BinaryOperator::Multiply | BinaryOperator::ArraySubscript)
}

/// Checks if the operator is valid in a LHS.
const fn is_valid_lhs_un(op: UnaryOperator) -> bool {
    matches!(op, UnaryOperator::Indirection)
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
    crate::lgp!("Making {current} LHS with * = {push_indirection}");
    let make_error = |val: &str| {
        Err(format!("LHS: expected a declaration or a modifiable lvalue, found {val}."))
    };

    match &mut current.value {
        AstValue::Variable(var) =>
            if push_indirection {
                var.push_indirection()
            } else {
                Ok(())
            },
        // can't be declaration: finished
        AstValue::Binary(Binary {
            op:
                BinaryOperator::StructEnumMemberAccess | BinaryOperator::StructEnumMemberPointerAccess,
            ..
        }) => Ok(()),
        AstValue::Unary(Unary { op: UnaryOperator::Indirection, arg }) => {
            arg.add_attribute_to_left_variable(vec![Attribute::Indirection])?;
            *current = mem::take(arg);
            Ok(())
        }
        AstValue::Binary(Binary { op: BinaryOperator::Multiply, arg_l, arg_r }) => {
            make_lhs_aux(arg_l, push_indirection)?;
            if let AstValue::Variable(old_var) = mem::take(arg_l).value {
                let mut attrs = old_var.into_attrs()?;
                attrs.push(Attribute::Indirection);
                arg_r.add_attribute_to_left_variable(attrs)?;
                *current = mem::take(arg_r);
                Ok(())
            } else {
                make_error("both")
            }
        }
        AstValue::Binary(Binary { op: BinaryOperator::ArraySubscript, arg_l, .. }) =>
            make_lhs_aux(arg_l, push_indirection),
        AstValue::Empty
        | AstValue::Cast(_)
        | AstValue::Leaf(_)
        | AstValue::BracedBlock(_)
        | AstValue::ParensBlock(_)
        | AstValue::ControlFlow(_)
        | AstValue::FunctionCall(_)
        | AstValue::ListInitialiser(_)
        | AstValue::FunctionArgsBuild(_) => unreachable!("lhs check returned false"),
        AstValue::Unary(_) | AstValue::Binary(_) | AstValue::Ternary(_) => {
            unreachable!("operator not rejected but illegal")
        }
    }
}

/// Tries to find a variable declaration and pushes a comma
pub fn try_apply_comma_to_variable(current: &mut Ast) -> Result<bool, String> {
    match &mut current.value {
        AstValue::Unary(Unary { arg, op }) if is_valid_lhs_un(*op) =>
            try_apply_comma_to_variable(arg),
        AstValue::Binary(Binary { arg_r: arg, op, .. }) if is_valid_lhs_bin(*op) =>
            try_apply_comma_to_variable(arg),
        AstValue::BracedBlock(BracedBlock { elts, full: false }) => elts
            .last_mut()
            .map_or(Ok(false), try_apply_comma_to_variable),
        AstValue::Variable(var) => Ok(var.push_comma()),
        AstValue::ControlFlow(ctrl) => ctrl
            .as_ast_mut()
            .map_or(Ok(false), try_apply_comma_to_variable),
        AstValue::Empty
        | AstValue::Leaf(_)
        | AstValue::Cast(_)
        | AstValue::Unary(_)
        | AstValue::Binary(_)
        | AstValue::Ternary(_)
        | AstValue::ParensBlock(_)
        | AstValue::FunctionCall(_)
        | AstValue::ListInitialiser(_)
        | AstValue::FunctionArgsBuild(_)
        | AstValue::BracedBlock(BracedBlock { full: true, .. }) => Ok(false),
    }
}
