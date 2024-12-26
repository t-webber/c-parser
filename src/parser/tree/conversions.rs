use core::mem;

use super::binary::{Binary, BinaryOperator};
use super::node::Node;
use super::traits::Operator;
use super::unary::{Unary, UnaryOperator};
use super::{Ternary, TernaryOperator};

#[allow(clippy::missing_trait_methods)]
impl OperatorConversions for BinaryOperator {
    fn try_to_node(self) -> Result<Node, String> {
        Err("Tried to call binary on empty node".into())
    }

    fn try_to_node_with_arg(self, arg: Node) -> Result<Node, String> {
        Ok(Node::Binary(Binary {
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
    fn try_convert_and_erase_node(self, node: &mut Node) -> Result<(), String> {
        //TODO: check that this is called only on unary operators
        *node = self.try_to_node()?;
        Ok(())
    }
    fn try_push_op_as_root(self, node: &mut Node) -> Result<(), String> {
        let old_node = mem::take(node);
        *node = self.try_to_node_with_arg(old_node)?;
        Ok(())
    }
    fn try_to_node(self) -> Result<Node, String>;
    fn try_to_node_with_arg(self, arg: Node) -> Result<Node, String>;
}

#[allow(clippy::missing_trait_methods)]
impl OperatorConversions for TernaryOperator {
    fn try_to_node(self) -> Result<Node, String> {
        Err("Tried to call ternary on empty node: Condition missing before '?' character.".into())
    }

    fn try_to_node_with_arg(self, arg: Node) -> Result<Node, String> {
        Ok(Node::Ternary(Ternary {
            op: Self,
            condition: Box::new(arg),
            success: Box::new(Node::Empty),
            failure: None,
        }))
    }
}

#[allow(clippy::missing_trait_methods)]
impl OperatorConversions for UnaryOperator {
    fn try_to_node(self) -> Result<Node, String> {
        Ok(Node::Unary(Unary {
            op: self,
            arg: None,
        }))
    }

    fn try_to_node_with_arg(self, arg: Node) -> Result<Node, String> {
        Ok(Node::Unary(Unary {
            op: self,
            arg: Some(Box::from(arg)),
        }))
    }
}
