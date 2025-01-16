//!Implement the `goto` control flow

use core::fmt;

use crate::EMPTY;
use crate::parser::keyword::control_flow::keyword::ControlFlowKeyword;
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::repr_option;
use crate::parser::types::Ast;

/// Keywords expected a colon then a identifier: `goto: label`
#[derive(Debug, PartialEq, Eq, Default)]
pub struct ColonIdentCtrl {
    /// [`Ast`] after the colon
    after: Option<String>,
    /// Colon found
    colon: bool,
}

impl ControlFlow for ColonIdentCtrl {
    type Keyword = ();

    fn fill(&mut self) {}

    fn from_keyword((): Self::Keyword) -> Self {
        Self::default()
    }

    fn get_ast(&self) -> Option<&Ast> {
        None
    }

    fn get_keyword(&self) -> ControlFlowKeyword {
        ControlFlowKeyword::Goto
    }

    fn get_mut(&mut self) -> Option<&mut Ast> {
        None
    }

    fn is_full(&self) -> bool {
        self.after.is_some()
    }

    fn push_colon(&mut self) -> bool {
        if self.colon {
            false
        } else {
            self.colon = true;
            true
        }
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
        if self.colon {
            if let Ast::Variable(var) = ast {
                self.after = Some(var.into_user_defined_name()?);
                Ok(())
            } else {
                Err("This is not a valid label. Expected an identifier".to_lowercase())
            }
        } else {
            Err("Missing colon after goto.".to_owned())
        }
    }

    fn push_op<T>(&mut self, _op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_op(&_op, self, "goto");
        debug_assert!(!self.is_full(), "");
        if self.colon {
            Err("This is not a valid label. Expected an identifier, found an operator.".to_owned())
        } else {
            Err("This is not a valid symbol. Expected a colon.".to_owned())
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for ColonIdentCtrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<goto{}{}>",
            if self.colon { ":" } else { EMPTY },
            repr_option(&self.after)
        )
    }
}
