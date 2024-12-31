pub mod keyword;
pub mod node;

use keyword::ControlFlowKeyword;
use node::ControlFlowNode;

use super::Ast;
use super::types::PushInNode;
use crate::parser::tree::blocks::Block;

impl PushInNode for ControlFlowKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        let block = Ast::Block(Block {
            elts: vec![Ast::ControlFlow(ControlFlowNode::from(self))],
            full: true,
        });
        node.push_braced_block(block);
        Ok(())
    }
}

pub fn is_node_case_context(node: &Ast) -> bool {
    match node {
            //
            //
            // true
            Ast::ControlFlow(ctrl) if *ctrl.get_keyword() == ControlFlowKeyword::Case && !ctrl.is_full() => true,
            //
            //
            // false
            // empty
            Ast::Empty
            | Ast::Leaf(_)
            | Ast::ParensBlock(_)
            // control flows are not expressions
            | Ast::Unary(_)
            | Ast::Binary(_)
            | Ast::Ternary(_)
            | Ast::FunctionCall(_)
            | Ast::ListInitialiser(_) |
            Ast::ControlFlow(_)
            // content is full
            | Ast::Block(Block { full: true, .. }) => false,
            //
            //
            // recurse
            Ast::Block(Block { elts, full: false }) => elts.last().is_some_and(is_node_case_context),
        }
}
