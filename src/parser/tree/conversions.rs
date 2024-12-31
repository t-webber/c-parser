use core::mem;

use super::ast::Ast;
use super::binary::{Binary, BinaryOperator};
use super::make_lhs::make_lhs;
use super::traits::Operator;
use super::unary::{Unary, UnaryOperator};
use super::{Ternary, TernaryOperator};

#[expect(clippy::missing_trait_methods)]
impl OperatorConversions for BinaryOperator {
    fn try_to_node(self) -> Result<Ast, String> {
        Err(format!(
            "Tried to call binary operator {self} on without a left argument."
        ))
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
            arg_l: Box::new(lvalue),
            arg_r: Box::new(Ast::Empty),
        }))
    }
}
pub trait OperatorConversions: Operator
where
    Self: Sized,
{
    fn try_convert_and_erase_node(self, node: &mut Ast) -> Result<(), String> {
        //TODO: check that this is called only on unary operators
        *node = self.try_to_node()?;
        Ok(())
    }
    fn try_push_op_as_root(self, node: &mut Ast) -> Result<(), String> {
        let old_node = mem::take(node);
        *node = self.try_to_node_with_arg(old_node)?;
        Ok(())
    }
    fn try_to_node(self) -> Result<Ast, String>;
    fn try_to_node_with_arg(self, arg: Ast) -> Result<Ast, String>;
}

#[expect(clippy::missing_trait_methods)]
impl OperatorConversions for TernaryOperator {
    fn try_to_node(self) -> Result<Ast, String> {
        Err("Tried to call ternary on empty node: missing condition expression.".into())
    }

    fn try_to_node_with_arg(self, arg: Ast) -> Result<Ast, String> {
        Ok(Ast::Ternary(Ternary {
            op: Self,
            condition: Box::new(arg),
            success: Box::new(Ast::Empty),
            failure: None,
        }))
    }
}

#[expect(clippy::missing_trait_methods)]
impl OperatorConversions for UnaryOperator {
    fn try_to_node(self) -> Result<Ast, String> {
        Ok(Ast::Unary(Unary {
            op: self,
            arg: Box::new(Ast::Empty),
        }))
    }

    fn try_to_node_with_arg(self, arg: Ast) -> Result<Ast, String> {
        Ok(Ast::Unary(Unary {
            op: self,
            arg: Box::from(arg),
        }))
    }
}
