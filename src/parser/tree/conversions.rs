use core::mem;

use super::binary::{Binary, BinaryOperator};
use super::node::Ast;
use super::traits::Operator;
use super::unary::{Unary, UnaryOperator};
use super::{Ternary, TernaryOperator};

#[expect(clippy::missing_trait_methods)]
impl OperatorConversions for BinaryOperator {
    fn try_to_node(self) -> Result<Ast, String> {
        Err("Tried to call binary on empty node".into())
    }

    fn try_to_node_with_arg(self, arg: Ast) -> Result<Ast, String> {
        Ok(Ast::Binary(Binary {
            op: self,
            arg_l: Box::new(arg),
            arg_r: None,
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
        Err("Tried to call ternary on empty node: Condition missing before '?' character.".into())
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
            arg: None,
        }))
    }

    fn try_to_node_with_arg(self, arg: Ast) -> Result<Ast, String> {
        Ok(Ast::Unary(Unary {
            op: self,
            arg: Some(Box::from(arg)),
        }))
    }
}
