//! Defines the control flow keywords.

use core::fmt;

use super::super::super::types::Ast;
use super::node::ControlFlowNode;
use crate::parser::keyword::sort::PushInNode;

/// Control flow keywords
// TODO: struct, enum, can be used as attribute
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

impl ControlFlowKeyword {
    /// Convert a [`ControlFlowKeyword`] into an [`Ast`]
    pub fn into_ast(self) -> Ast {
        Ast::ControlFlow(ControlFlowNode::from(self))
    }
}

impl PushInNode for ControlFlowKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        if let Ast::BracedBlock(block) = node {
            if let Some(last) = block.elts.last_mut() {
                if last.can_push_leaf() && !matches!(self, Self::Case | Self::Default) {
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

#[expect(clippy::min_ident_chars)]
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
        }
    }
}

impl From<ControlFlowKeyword> for ControlFlowNode {
    fn from(keyword: ControlFlowKeyword) -> Self {
        match keyword {
            ControlFlowKeyword::Break | ControlFlowKeyword::Continue => Self::SemiColon(keyword),
            ControlFlowKeyword::Case => {
                Self::AstColonAst(keyword, Box::from(Ast::Empty), None, false)
            }
            ControlFlowKeyword::Default => Self::ColonAst(keyword, None, false),
            ControlFlowKeyword::Goto => Self::ColonIdent(keyword, false, None),
            ControlFlowKeyword::For | ControlFlowKeyword::While | ControlFlowKeyword::Switch => {
                Self::ParensBlock(keyword, None, Box::from(Ast::Empty), false)
            }
            ControlFlowKeyword::Do | ControlFlowKeyword::Return => {
                Self::Ast(keyword, Box::from(Ast::Empty), false)
            }
            ControlFlowKeyword::Typedef => Self::ControlFlow(keyword, None),

            ControlFlowKeyword::Enum | ControlFlowKeyword::Union | ControlFlowKeyword::Struct => {
                Self::IdentBlock(keyword, None, None)
            }
            ControlFlowKeyword::If => {
                Self::Condition(None, Box::from(Ast::Empty), false, None, false)
            }
        }
    }
}
