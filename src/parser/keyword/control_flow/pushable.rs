//! Module to deal with keywords that need to be pushed into control flows.

use core::{fmt, panic};

use super::node::ControlFlowNode;
use super::traits::ControlFlow as _;
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
        if !ctrl.is_full()
            && let ControlFlowNode::Condition(condition) = ctrl
        {
            condition.push_else()
        } else if let Some(arg) = ctrl.get_mut() {
            self.push_in_node(arg)
        } else {
            Err("found `else` without an `if`".to_owned())
        }
    }
}

impl PushInNode for PushableKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_in_node(&self, "else", node);
        match node {
            Ast::Empty
            | Ast::Leaf(_)
            | Ast::Unary(_)
            | Ast::Binary(_)
            | Ast::Ternary(_)
            | Ast::Variable(_)
            | Ast::ParensBlock(_)
            | Ast::ListInitialiser(_)
            | Ast::FunctionArgsBuild(_)
            | Ast::FunctionCall(_) => panic!("found a control flow: pushing {self} in {node}"),
            Ast::BracedBlock(BracedBlock { elts, .. }) => self.push_in_node(
                elts.last_mut()
                    .expect("tried to push else in empty block: missing if"),
            ),
            Ast::ControlFlow(ctrl) => self.push_in_ctrl(ctrl),
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for PushableKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "else".fmt(f)
    }
}
