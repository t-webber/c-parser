//! Implements the function keywords

pub mod keyword;
pub mod node;

use keyword::ControlFlowKeyword;
use node::ControlFlowNode;

use super::super::types::blocks::BracedBlock;
use super::Ast;
use super::sort::PushInNode;

impl PushInNode for ControlFlowKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        let block = Ast::BracedBlock(BracedBlock {
            elts: vec![Ast::ControlFlow(ControlFlowNode::from(self))],
            full: true,
        });
        node.push_braced_block(block);
        Ok(())
    }
}

/// Checks if the current [`Ast`] is writing inside a `case` control flow.
pub fn is_node_case_context(node: &Ast) -> bool {
    match node {
        Ast::Empty
        | Ast::Leaf(_)
        | Ast::ParensBlock(_)
        | Ast::Unary(_)
        | Ast::Binary(_)
        | Ast::Ternary(_)
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(_)
        | Ast::BracedBlock(BracedBlock { full: true, .. }) => false,
        Ast::ControlFlow(ctrl) => {
            *ctrl.get_keyword() == ControlFlowKeyword::Case && !ctrl.is_full()
        }
        Ast::BracedBlock(BracedBlock { elts, full: false }) => {
            elts.last().is_some_and(is_node_case_context)
        }
    }
}
