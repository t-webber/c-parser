//!Implement the  control flow followed by a semi-colon, such as `Break` and
//!`continue`.

use core::fmt;

use crate::parser::keyword::control_flow::keyword::ControlFlowKeyword;
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::modifiers::ast::AstPushContext;
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

    fn get_keyword(&self) -> ControlFlowKeyword {
        match self.0 {
            Self::Keyword::Break => ControlFlowKeyword::Break,
            Self::Keyword::Continue => ControlFlowKeyword::Continue,
        }
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
}

impl Push for SemiColonCtrl {
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
