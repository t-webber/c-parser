//! Defines the control flow keywords.

use core::fmt;

use super::super::super::types::Ast;
use super::node::ControlFlowNode;

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
    Else,
    /// Enum type declaration
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

#[expect(clippy::min_ident_chars)]
impl fmt::Display for ControlFlowKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Break => "break".fmt(f),
            Self::Case => "case".fmt(f),
            Self::Continue => "continue".fmt(f),
            Self::Default => "default".fmt(f),
            Self::Do => "do".fmt(f),
            Self::Else => "else".fmt(f),
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
            ControlFlowKeyword::Break
            | ControlFlowKeyword::Return
            | ControlFlowKeyword::Continue => Self::SemiColon(keyword),
            ControlFlowKeyword::Case | ControlFlowKeyword::Default | ControlFlowKeyword::Goto => {
                Self::ColonAst(keyword, None)
            }
            ControlFlowKeyword::For
            | ControlFlowKeyword::While
            | ControlFlowKeyword::Switch
            | ControlFlowKeyword::If => Self::ParensBlock(keyword, None, None),
            // block
            ControlFlowKeyword::Do | ControlFlowKeyword::Else => {
                Self::Ast(keyword, Box::from(Ast::Empty))
            }
            // special
            ControlFlowKeyword::Typedef => Self::ControlFlow(keyword, None),

            ControlFlowKeyword::Enum | ControlFlowKeyword::Union | ControlFlowKeyword::Struct => {
                Self::IdentBlock(keyword, None, None)
            }
        }
    }
}
