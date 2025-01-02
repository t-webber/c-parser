//! Functions to make an [`Ast`] a LHS.
//!
//! A LHS (left-hand-side) is a node that is on the left-hand-sign of an
//! assignment, or in the arguments of a function declaration.
//!
//! They can be type declaration with attributes, or expressions with
//! assignments.

use core::mem;

use super::super::types::binary::{Binary, BinaryOperator};
use super::super::types::braced_blocks::BracedBlock;
use super::super::types::literal::{Attribute, Literal, Variable};
use super::super::types::unary::{Unary, UnaryOperator};
use super::super::types::{Ast, ListInitialiser};
use crate::parser::types::ternary::Ternary;

/// Checks if the current [`Ast`] has a variable with attributes.
///
/// If it is the case, an expression is not allowed in the LHS because it is a
/// type declaration.
fn has_attributes(current: &Ast) -> bool {
    match current {
        // success
        Ast::Leaf(Literal::Variable(Variable { attrs, .. })) => !attrs.is_empty(),
        // failure
        Ast::Empty
        | Ast::Leaf(_)
        | Ast::BracedBlock(_)
        | Ast::ParensBlock(_)
        | Ast::ControlFlow(_)
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(_)
        | Ast::FunctionArgsBuild(_) => false,
        // recurse
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

/// Make an [`Ast`] a LHS node
///
/// This is called when an assign
/// [`Operator`](super::super::types::operator::Operator) is created or a
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
    let make_error = |val: &str| {
        Err(format!(
            "LHS: expected a declaration or a modifiable lvalue, found {val}."
        ))
    };

    match current {
        // success
        Ast::Leaf(Literal::Variable(var)) => {
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
        // recurse
        Ast::Unary(Unary {
            op: UnaryOperator::Indirection,
            arg,
        }) => arg.add_attribute_to_left_variable(vec![Attribute::Indirection]),
        Ast::Binary(Binary {
            op: BinaryOperator::Multiply,
            arg_l,
            arg_r,
        }) => {
            make_lhs_aux(arg_l, push_indirection)?;
            if let Ast::Leaf(Literal::Variable(old_var)) = *mem::take(arg_l) {
                let mut new_var = old_var;
                new_var.push_indirection()?;
                arg_r.add_attribute_to_left_variable(new_var.attrs)?;
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
        // failure
        Ast::Empty => make_error("nothing"),
        Ast::FunctionArgsBuild(_) => make_error("function argument"),
        Ast::ParensBlock(_) => make_error("parenthesis"),
        Ast::Leaf(lit) => make_error(&format!("constant literal {lit}.")),
        Ast::Unary(Unary { op, .. }) => make_error(&format!("unary operator {op}")),
        Ast::Binary(Binary { op, .. }) => make_error(&format!("binary operator '{op}'")),
        Ast::Ternary(_) => make_error("ternary operator"),
        Ast::FunctionCall(_) => make_error("function"),
        Ast::ListInitialiser(ListInitialiser { full: true, .. }) => make_error("list initialiser"),
        Ast::BracedBlock(BracedBlock { full: true, .. }) => make_error("block"),
        Ast::ControlFlow(_) => make_error("control flow"),
        Ast::ListInitialiser(ListInitialiser { .. }) | Ast::BracedBlock(BracedBlock { .. }) => {
            panic!("Didn't pushed assign operator low enough")
        }
    }
}
