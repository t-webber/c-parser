//!Implement the  control flow followed by a semi-colon, such as `Break` and
//!`continue`.

use core::fmt;

use crate::keyword::control_flow::traits::ControlFlow;
use crate::modifiers::push::Push;
use crate::operators::OperatorConversions;
use crate::tree::Ast;
use utils::display;

/// Keyword expects a semicolon: `break;`
#[derive(Debug, PartialEq, Eq)]
pub struct SemiColonCtrl(SemiColonKeyword);

impl ControlFlow for SemiColonCtrl {
    type Keyword = SemiColonKeyword;

    fn as_ast(&self) -> Option<&Ast> {
        None
    }

    fn as_ast_mut(&mut self) -> Option<&mut Ast> {
        None
    }

    fn fill(&mut self) {}

    fn from_keyword(keyword: Self::Keyword) -> Self {
        Self(keyword)
    }

    fn is_full(&self) -> bool {
        true
    }

    fn push_colon(&mut self) -> bool {
        false
    }

    fn push_semicolon(&mut self) -> bool {
        false
    }
}

impl Push for SemiColonCtrl {
    fn push_block_as_leaf(&mut self, _: Ast) -> Result<(), String> {
        debug_assert!(!self.is_full(), "");
        unreachable!("unreachable")
    }

    fn push_op<T>(&mut self, _: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        debug_assert!(!self.is_full(), "");
        unreachable!("unreachable")
    }
}

display!(
    SemiColonCtrl,
    self,
    f,
    write!(
        f,
        "<{}>",
        match self.0 {
            SemiColonKeyword::Break => "break",
            SemiColonKeyword::Continue => "continue",
        }
    )
);

/// C control flow keywords that have the [`SemiColonCtrl`] structure.
#[derive(Debug, PartialEq, Eq)]
pub enum SemiColonKeyword {
    /// `break;`
    Break,
    /// `continue;`
    Continue,
}
