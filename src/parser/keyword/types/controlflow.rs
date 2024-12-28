use super::super::Ast;
use super::PushInNode;

pub enum ControlFlowKeyword {
    Break,
    Case,
    Continue,
    Default,
    Do,
    Else,
    For,
    Goto,
    If,
    Return,
    Switch,
    While,
}

impl ControlFlowKeyword {
    pub fn is_in_case_context(node: &Ast) -> bool {
        todo!()
    }
}

impl PushInNode for ControlFlowKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        todo!()
    }
}
