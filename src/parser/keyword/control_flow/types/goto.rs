//!Implement the `goto` control flow

use core::fmt;

use crate::EMPTY;
use crate::parser::keyword::control_flow::keyword::ControlFlowKeyword;
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::modifiers::ast::AstPushContext;
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
}

impl Push for ColonIdentCtrl {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        if let Some(arg) = self.get_mut() {
            if matches!(ast, Ast::BracedBlock(_)) {
                if *arg == Ast::Empty {
                    *arg = ast;
                    self.fill();
                } else {
                    arg.push_braced_block(ast)?;
                    if !arg.can_push_leaf_with_ctx(AstPushContext::UserVariable) {
                        self.fill();
                    }
                }
            } else {
                arg.push_block_as_leaf(ast)?;
            }
            Ok(())
        } else {
            Err(format!("Failed to push block {ast} as leaf in ctrl {self}"))
        }
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        self.get_mut().map_or_else(
            || Err("Operator not pushable in ctrl flow".to_owned()),
            |arg| arg.push_op(op),
        )
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
