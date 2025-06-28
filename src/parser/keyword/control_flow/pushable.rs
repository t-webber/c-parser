//! Module to deal with keywords that need to be pushed into control flows.

use core::panic;

use crate::parser::keyword::control_flow::node::ControlFlowNode;
use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::keyword::sort::PushInNode;
use crate::parser::symbols::api::BracedBlock;
use crate::parser::tree::Ast;
use crate::utils::display;

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
        } else if let Some(arg) = ctrl.as_ast_mut() {
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
            | Ast::Cast(_)
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

display!(PushableKeyword, self, f, "else".fmt(f));
