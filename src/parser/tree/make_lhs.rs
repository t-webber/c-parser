use core::mem;

use super::ast::Ast;
use super::binary::{Binary, BinaryOperator};
use super::blocks::Block;
use super::literal::{Attribute, Literal, Variable};
use super::unary::{Unary, UnaryOperator};
use super::{FunctionCall, ListInitialiser, Ternary};

fn has_attributes(current: &Ast) -> bool {
    match current {
        // success
        Ast::Leaf(Literal::Variable(Variable { attrs, .. })) => !attrs.is_empty(),
        // failure
        Ast::Empty
        | Ast::Block(_)
        | Ast::Leaf(_)
        | Ast::ControlFlow(_)
        | Ast::ParensBlock(_)
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(_) => false,
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
/// This is called when an assign [`Operator`](super::Operator) is created
/// or a function is created, to convert `*` to a type attribute. It
/// also check that the [`Ast`] is a valid LHS.
pub fn make_lhs(current: &mut Ast) -> Result<(), String> {
    if has_attributes(current) {
        /* LHS is a declaration */
        make_lhs_aux(current, false)
    } else {
        /* LHS is an expression */
        Ok(())
    }
}

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
        Ast::ParensBlock(_) => make_error("parenthesis"),
        Ast::Leaf(lit) => make_error(&format!("constant literal {lit}.")),
        Ast::Unary(Unary { op, .. }) => make_error(&format!("unary operator {op}")),
        Ast::Binary(Binary { op, .. }) => make_error(&format!("binary operator '{op}'")),
        Ast::Ternary(_) => make_error("ternary operator"),
        Ast::FunctionCall(FunctionCall { full: true, .. }) => make_error("function"),
        Ast::ListInitialiser(ListInitialiser { full: true, .. }) => make_error("list initialiser"),
        Ast::Block(Block { full: true, .. }) => make_error("block"),
        Ast::ControlFlow(_) => make_error("control flow"),
        Ast::FunctionCall(FunctionCall { .. })
        | Ast::ListInitialiser(ListInitialiser { .. })
        | Ast::Block(Block { .. }) => panic!("Didn't pushed assign operator low enough"),
    }
}
