//!Implement the  control flow followed by a semi-colon, such as `Break` and
//!`continue`.

use core::fmt;

use crate::BracedBlock;
use crate::errors::api::{ErrorLocation, Located};
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::modifiers::push::Push;
use crate::parser::operators::api::OperatorConversions;
use crate::parser::tree::Ast;
use crate::utils::{StringResolver, display};

/// Keyword expects a semicolon: `break;`
#[derive(Debug)]
pub struct SemiColonCtrl(Located<SemiColonKeyword>);

impl ControlFlow for SemiColonCtrl {
    type Keyword = Located<SemiColonKeyword>;

    fn as_ast(&self) -> Option<&Ast> {
        None
    }

    fn as_ast_mut(&mut self) -> Option<&mut Ast> {
        None
    }

    fn display(&self, _: &StringResolver<BracedBlock>) -> String {
        format!("{:?}", self.0.as_value())
    }

    fn fill(&mut self) {}

    fn from_keyword(keyword: Self::Keyword) -> Self {
        Self(keyword)
    }

    fn is_full(&self) -> bool {
        true
    }

    fn location(&self) -> ErrorLocation {
        self.0.as_location()
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
        T: OperatorConversions + fmt::Display,
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
        match self.0.as_value() {
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
