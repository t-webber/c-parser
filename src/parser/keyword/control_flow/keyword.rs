//! Defines the control flow keywords.

use core::fmt;

use super::node::ControlFlowNode;
use super::traits::ControlFlow as _;
use crate::parser::keyword::sort::PushInNode;
use crate::parser::modifiers::push::Push as _;
use crate::parser::types::Ast;

impl From<ControlFlowKeyword> for Ast {
    fn from(keyword: ControlFlowKeyword) -> Self {
        Self::ControlFlow(ControlFlowNode::from_keyword(keyword))
    }
}

/// Control flow keywords
#[derive(Debug, PartialEq, Eq)]
pub enum ControlFlowKeyword {
    /// Break out of a loop or a case
    Break,
    /// Case pattern inside a switch
    Case,
    /// Continue control flow
    ///
    /// Continue the loop
    Continue,
    /// Default control flow
    ///
    /// Default case match arm
    Default,
    /// Do-while loop creation
    Do,
    /// Else conditional keyword
    Enum,
    /// For loop creation
    For,
    /// Goto label
    Goto,
    /// If conditional keyword
    If,
    /// Label
    ///
    /// This is not stricto sensu a control flow, but it acts like one, as
    /// `label` is not a keyword.
    Label(String),
    /// Return function
    Return,
    /// Struct type declaration
    Struct,
    /// Switch pattern creation
    Switch,
    /// Typedef user-defined type
    Typedef,
    /// Union type creation
    Union,
    /// While loop creation
    While,
}

impl PushInNode for ControlFlowKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_in_node(&self, "ctrl", node);
        if let Ast::BracedBlock(block) = node {
            if let Some(last) = block.elts.last_mut() {
                if last.can_push_leaf() && !matches!(self, Self::Case | Self::Default) {
                    return self.push_in_node(last);
                }
            }
            block.elts.push(Ast::from(self));
            Ok(())
        } else if node.is_empty() {
            *node = Ast::from(self);
            Ok(())
        } else if let Ast::ControlFlow(ctrl) = node {
            if ctrl.is_full() {
                Err("Trying to push control flow block to a full control flow.".to_owned())
            } else {
                ctrl.push_block_as_leaf(Ast::from(self))
            }
        } else {
            Err("Applying operator on control flow is illegal.".to_owned())
        }
    }
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for ControlFlowKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Break => "break".fmt(f),
            Self::Case => "case".fmt(f),
            Self::Continue => "continue".fmt(f),
            Self::Default => "default".fmt(f),
            Self::Do => "do".fmt(f),
            Self::Enum => "enum".fmt(f),
            Self::For => "for".fmt(f),
            Self::Goto => "goto".fmt(f),
            Self::If => "if".fmt(f),
            Self::Return => "return".fmt(f),
            Self::Struct => "struct".fmt(f),
            Self::Switch => "switch".fmt(f),
            Self::Typedef => "typedef".fmt(f),
            Self::Union => "union".fmt(f),
            Self::While => "while".fmt(f),
            Self::Label(label) => label.fmt(f),
        }
    }
}
