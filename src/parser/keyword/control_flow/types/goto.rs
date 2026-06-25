//!Implement the `goto` control flow

use core::fmt;

use crate::errors::api::{ErrorLocation, Located};
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::modifiers::push::Push;
use crate::parser::operators::api::OperatorConversions;
use crate::parser::tree::Ast;
use crate::utils::{display, repr_option};

/// Keywords expected a colon then a identifier: `goto: label`
#[derive(Debug, Default)]
pub struct ColonIdentCtrl {
    /// Location of the `goto` keyword.
    keyword_location: ErrorLocation,
    /// name of the label to jump to
    label: Option<Located<String>>,
}

impl ControlFlow for ColonIdentCtrl {
    type Keyword = ErrorLocation;

    fn as_ast(&self) -> Option<&Ast> {
        None
    }

    fn as_ast_mut(&mut self) -> Option<&mut Ast> {
        None
    }

    fn fill(&mut self) {}

    fn from_keyword(keyword: ErrorLocation) -> Self {
        Self { keyword_location: keyword, ..Self::default() }
    }

    fn is_full(&self) -> bool {
        self.label.is_some()
    }

    fn location(&self) -> ErrorLocation {
        self.label.as_ref().map_or_else(
            || self.keyword_location,
            |label| self.keyword_location.into_extended(label.as_location()),
        )
    }

    fn push_colon(&mut self) -> bool {
        false
    }

    fn push_semicolon(&mut self) -> bool {
        false
    }
}

impl Push for ColonIdentCtrl {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf(&ast, self, "goto");
        debug_assert!(!self.is_full(), "");
        if let Ast::Variable(var) = ast {
            self.label = Some(var.into_user_defined_name()?);
            Ok(())
        } else {
            Err("This is not a valid label. Expected an identifier".to_owned())
        }
    }

    fn push_op<T>(&mut self, _op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display,
    {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_op(&_op, self, "goto");
        debug_assert!(!self.is_full(), "");
        Err("This is not a valid label. Expected an identifier, found an operator.".to_owned())
    }
}

display!(ColonIdentCtrl, self, f, write!(f, "<goto {}>", repr_option(&self.label)));
