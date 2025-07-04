//! Module to implement conversions to push an [`Operator`] on top of an
//! [`Ast`].

use core::{marker, mem};

use super::ternary::TernaryOperator;
use crate::parser::modifiers::make_lhs::make_lhs;
use crate::parser::operators::api::{
    Binary, BinaryOperator, Operator, Ternary, Unary, UnaryOperator
};
use crate::parser::tree::api::Ast;

impl OperatorConversions for BinaryOperator {
    fn try_to_node(self) -> Result<Ast, String> {
        Err(format!("Tried to call binary operator {self} on without a left argument."))
    }

    fn try_to_node_with_arg(self, arg: Ast) -> Result<Ast, String> {
        let lvalue = if self.precedence() == 14 {
            let mut old = arg;
            make_lhs(&mut old)?;
            old
        } else {
            arg
        };
        Ok(Ast::Binary(Binary {
            op: self,
            arg_l: lvalue.into_box(),
            arg_r: Ast::empty_box(),
        }))
    }
}

/// Trait that defines methods to insert an [`Operator`] just on top of the
/// current [`Ast`]. The old [`Ast`] because a node of depth 1 (with the depth
/// of the root being 0) in the new [`Ast`].
pub trait OperatorConversions: Operator
where
    Self: marker::Sized,
{
    /// Makes an [`Ast`] from the operator and replaces the `node` by it.
    fn try_convert_and_erase_node(self, node: &mut Ast) -> Result<(), String> {
        *node = self.try_to_node()?;
        Ok(())
    }

    /// Makes an [`Ast`] from the operator and pushes the current [`Ast`] as
    /// an argument.
    fn try_push_op_as_root(self, node: &mut Ast) -> Result<(), String> {
        let old_node = mem::take(node);
        *node = self.try_to_node_with_arg(old_node)?;
        Ok(())
    }

    /// Makes a node from an operator, without any argument.
    fn try_to_node(self) -> Result<Ast, String>;
    /// Makes a node from an operator, with an argument to be pushed as its
    /// leaf.
    fn try_to_node_with_arg(self, arg: Ast) -> Result<Ast, String>;
}

impl OperatorConversions for TernaryOperator {
    fn try_to_node(self) -> Result<Ast, String> {
        Err("Tried to call ternary on empty node: missing condition expression.".into())
    }

    fn try_to_node_with_arg(self, arg: Ast) -> Result<Ast, String> {
        Ok(Ast::Ternary(Ternary {
            op: Self,
            condition: arg.into_box(),
            success: Ast::empty_box(),
            failure: None,
        }))
    }
}

impl OperatorConversions for UnaryOperator {
    fn try_to_node(self) -> Result<Ast, String> {
        Ok(Ast::Unary(Unary { op: self, arg: Ast::empty_box() }))
    }

    fn try_to_node_with_arg(self, arg: Ast) -> Result<Ast, String> {
        Ok(Ast::Unary(Unary { op: self, arg: arg.into_box() }))
    }
}
