//!Implement the `default` control flow

use core::{fmt, mem};

use crate::parser::keyword::control_flow::keyword::ControlFlowKeyword;
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::keyword::control_flow::types::repr_colon_option;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::repr_fullness;
use crate::parser::types::Ast;
use crate::parser::types::braced_blocks::BracedBlock;

/// Keyword expects a node and then a colon: `case 2:`
#[derive(Debug, PartialEq, Default)]
pub struct ColonAstCtrl {
    /// [`Ast`] after the colon
    after: Option<Box<Ast>>,
    /// fullness of the [`Ast`]
    full: bool,
}

impl ControlFlow for ColonAstCtrl {
    type Keyword = ();

    fn fill(&mut self) {
        self.full = true;
    }

    fn from_keyword((): Self::Keyword) -> Self {
        Self::default()
    }

    fn get_ast(&self) -> Option<&Ast> {
        if self.full {
            None
        } else {
            self.after.as_deref()
        }
    }

    fn get_keyword(&self) -> ControlFlowKeyword {
        ControlFlowKeyword::Goto
    }

    fn get_mut(&mut self) -> Option<&mut Ast> {
        if self.full {
            None
        } else {
            self.after.as_deref_mut()
        }
    }

    fn is_full(&self) -> bool {
        self.full
    }

    fn push_colon(&mut self) -> bool {
        if self.after.is_none() && !self.full {
            self.after = Some(Ast::empty_box());
            true
        } else {
            false
        }
    }

    fn push_semicolon(&mut self) -> bool {
        if !self.full
            && let Some(ast) = &mut self.after
        {
            if let Ast::BracedBlock(BracedBlock { elts, full: false }) = &mut **ast {
                elts.push(Ast::Empty);
            } else {
                *ast = Box::new(Ast::BracedBlock(BracedBlock {
                    elts: vec![mem::take(ast), Ast::Empty],
                    full: false,
                }));
            }
            true
        } else {
            false
        }
    }
}

impl Push for ColonAstCtrl {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        debug_assert!(!self.is_full(), "");
        self.after.as_mut().map_or_else(
            || Err("Missing colon.".to_owned()),
            |arg| arg.push_block_as_leaf(ast),
        )
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        debug_assert!(!self.is_full(), "");
        self.after
            .as_mut()
            .map_or_else(|| Err("Missing colon.".to_owned()), |arg| arg.push_op(op))
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for ColonAstCtrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<default {}{}>",
            repr_colon_option(self.after.as_ref()),
            repr_fullness(self.full)
        )
    }
}
