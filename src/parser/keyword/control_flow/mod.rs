//! Implements the function keywords

pub mod keyword;
pub mod node;

use keyword::ControlFlowKeyword;
use node::ControlFlowNode;

use super::Ast;
use super::sort::PushInNode;

impl PushInNode for ControlFlowKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        let ctrl_node = Ast::ControlFlow(ControlFlowNode::from(self));

        if let Ast::BracedBlock(block) = node {
            if block.elts.last() == Some(&Ast::Empty) {
                block.elts.pop();
            }
            block.elts.push(ctrl_node);
            Ok(())
        } else if &Ast::Empty == node {
            *node = ctrl_node;
            Ok(())
        } else {
            Err(format!(
                "Control flow found at root but this is illegal: Tried to push {ctrl_node} in {node}."
            ))
        }
    }
}
