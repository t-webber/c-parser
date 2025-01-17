//!Implement the  control flow followed by a semi-colon, such as `Break` and
//!`continue`.

use core::fmt;

use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::types::Ast;

/// Keyword expects a semicolon: `break;`
#[derive(Debug, PartialEq, Eq)]
pub struct SemiColonCtrl(SemiColonKeyword);

impl ControlFlow for SemiColonCtrl {
    type Keyword = SemiColonKeyword;

    fn fill(&mut self) {}

    fn from_keyword(keyword: Self::Keyword) -> Self {
        Self(keyword)
    }

    fn get_ast(&self) -> Option<&Ast> {
        None
    }

    fn get_mut(&mut self) -> Option<&mut Ast> {
        None
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
        panic!("unreachable")
    }

    fn push_op<T>(&mut self, _: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        debug_assert!(!self.is_full(), "");
        panic!("unreachable")
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for SemiColonCtrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", match self.0 {
            SemiColonKeyword::Break => "break",
            SemiColonKeyword::Continue => "continue",
        })
    }
}

/// C control flow keywords that have the [`SemiColonCtrl`] structure.
#[derive(Debug, PartialEq, Eq)]
pub enum SemiColonKeyword {
    /// `break;`
    Break,
    /// `continue;`
    Continue,
}
