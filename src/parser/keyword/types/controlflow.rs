#![expect(clippy::arbitrary_source_item_ordering)]

use super::super::Ast;
use super::PushInNode;

pub enum ControlFlowKeyword {
    // cases & loops
    Break,
    Case,
    Continue,
    Default,
    Do,
    For,
    Goto,
    Switch,
    While,
    // condition
    Else,
    If,
    // user defined types
    Typedef,
    Struct,
    Union,
    Enum,
    //
    Return,
}

impl ControlFlowKeyword {
    pub const fn is_in_case_context(node: &Ast) -> bool {
        // todo!()
        true //TODO
    }
}

impl PushInNode for ControlFlowKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        todo!()
    }
}
