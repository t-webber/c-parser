//! Module to deal with keywords that need to be pushed into control flows.

use core::panic;

use super::node::ControlFlowNode;
use crate::parser::keyword::sort::PushInNode;
use crate::parser::types::Ast;
use crate::parser::types::braced_blocks::BracedBlock;

/// Keywords that must be pushed into an existing control flow
#[derive(Debug)]
pub enum PushableKeyword {
    /// Else block of a conditional control flow
    Else,
}

impl PushableKeyword {
    /// Tries to push a [`PushableKeyword`] in the corresponding
    /// [`ControlFlowNode`]
    fn push_in_ctrl(self, ctrl: &mut ControlFlowNode) -> Result<(), String> {
        if let ControlFlowNode::Condition(condition, success, full_s, failure, full) = ctrl {
            if *full {
                panic!("tried to push on panic")
            } else if condition.is_none() {
                Err("missing condition: missing `(` after `if`".to_owned())
            } else if **success == Ast::Empty {
                Err("missing success block after `if` condition".to_owned())
            } else if let Some(fail) = failure {
                self.push_in_node(fail)
            } else {
                *full_s = true;
                *failure = Some(Box::from(Ast::Empty));
                Ok(())
            }
        } else if let Some(arg) = ctrl.get_ast_mut() {
            self.push_in_node(arg)
        } else {
            Err("found `else` without an `if`".to_owned())
        }
    }
}

impl PushInNode for PushableKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        match node {
            Ast::Empty
            | Ast::Unary(_)
            | Ast::Binary(_)
            | Ast::Leaf(_)
            | Ast::Ternary(_)
            | Ast::ParensBlock(_)
            | Ast::ListInitialiser(_)
            | Ast::FunctionArgsBuild(_)
            | Ast::FunctionCall(_) => panic!("found a control flow: pushing {self:?} in {node}"),
            Ast::BracedBlock(BracedBlock { elts, .. }) => self.push_in_node(
                elts.last_mut()
                    .expect("tried to push else in empty block: missing if"),
            ),
            Ast::ControlFlow(ctrl) => self.push_in_ctrl(ctrl),
        }
    }
}
