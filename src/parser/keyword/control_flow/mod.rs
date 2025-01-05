//! Implements the function keywords

pub mod keyword;
pub mod node;
pub mod pushable;

use keyword::ControlFlowKeyword;
use node::ControlFlowNode;

use super::Ast;
use super::sort::PushInNode;

impl ControlFlowKeyword {
    /// Convert a [`ControlFlowKeyword`] into an [`Ast`]
    fn into_ast(self) -> Ast {
        Ast::ControlFlow(ControlFlowNode::from(self))
    }
}

impl PushInNode for ControlFlowKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        if let Ast::BracedBlock(block) = node {
            if let Some(last) = block.elts.last_mut() {
                if last.can_push_leaf(false) {
                    return self.push_in_node(last);
                }
            }
            block.elts.push(self.into_ast());
            Ok(())
        } else if &Ast::Empty == node {
            *node = self.into_ast();
            Ok(())
        } else if let Ast::ControlFlow(ctrl) = node {
            if ctrl.is_full() {
                Err("Trying to push control flow block to a full control flow.".to_owned())
            } else {
                ctrl.push_block_as_leaf(self.into_ast())
            }
        } else {
            Err("Applying operator on control flow is illegal.".to_owned())
        }
    }
}
