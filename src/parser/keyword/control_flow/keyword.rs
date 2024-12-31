use core::fmt;

use super::super::super::types::Ast;
use super::node::ControlFlowNode;

#[derive(Debug, PartialEq, Eq)]
pub enum ControlFlowKeyword {
    // cases & loops
    Break,
    Case,
    Continue,
    Default,
    Do,
    Else,
    Enum,
    For,
    Goto,
    If,
    Return,
    Struct,
    Switch,
    Typedef,
    Union,
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
