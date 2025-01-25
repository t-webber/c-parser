//!Implement the `do-while` control flow

use core::{fmt, mem};

use crate::parser::keyword::control_flow::node::{ControlFlowNode, try_push_semicolon_control};
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::repr_option;
use crate::parser::types::Ast;
use crate::parser::types::braced_blocks::BracedBlock;
use crate::parser::types::parens::ParensBlock;

/// `do` keyword
#[derive(Debug, Default)]
pub struct DoWhileCtrl {
    /// looping condition, after the `while` keyword
    condition: Option<ParensBlock>,
    /// [`Ast`] executed at each interaction
    loop_block: Box<Ast>,
    /// `while` keyword found or not.
    ///
    /// Used from `loop_block` to `condition`
    while_found: bool,
}

impl ControlFlow for DoWhileCtrl {
    type Keyword = ();

    fn as_ast(&self) -> Option<&Ast> {
        (!self.while_found).then(|| self.loop_block.as_ref())
    }

    fn as_ast_mut(&mut self) -> Option<&mut Ast> {
        (!self.while_found).then(|| self.loop_block.as_mut())
    }

    fn fill(&mut self) {}

    fn from_keyword((): Self::Keyword) -> Self {
        Self::default()
    }

    fn is_full(&self) -> bool {
        self.condition.is_some()
    }

    fn push_colon(&mut self) -> bool {
        false
    }

    fn push_semicolon(&mut self) -> bool {
        if self.while_found {
            false
        } else {
            try_push_semicolon_control(&mut self.loop_block) || {
                if let Ast::BracedBlock(BracedBlock { elts, full: false }) = &mut *self.loop_block {
                    elts.push(Ast::Empty);
                } else if !self.loop_block.is_empty() {
                    *self.loop_block = Ast::BracedBlock(BracedBlock {
                        elts: vec![mem::take(&mut self.loop_block), Ast::Empty],
                        full: false,
                    });
                }
                true
            }
        }
    }
}

impl Push for DoWhileCtrl {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf(&ast, self, "do-while");
        debug_assert!(!self.is_full(), "");
        if self.while_found {
            debug_assert!(self.condition.is_none(), "");
            if let Ast::ParensBlock(parens) = ast {
                self.condition = Some(parens);
                Ok(())
            } else {
                Err("Missing condition: expect (.".to_owned())
            }
        } else if let Ast::ControlFlow(ControlFlowNode::ParensBlock(ctrl)) = &ast
            && ctrl.is_while()
        {
            self.loop_block.fill();
            self.while_found = true;
            Ok(())
        } else {
            self.loop_block.push_block_as_leaf(ast)
        }
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_op(&op, self, "do-while");
        debug_assert!(!self.is_full(), "");
        if self.while_found {
            Err("Expected condition, found operator".to_owned())
        } else {
            self.loop_block.push_op(op)
        }
    }
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for DoWhileCtrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<do {}{}>",
            self.loop_block,
            if self.while_found {
                format!(" while {}", repr_option(&self.condition))
            } else {
                self.condition
                    .as_ref()
                    .map_or_else(|| "..".to_owned(), |cond| format!("{cond}"))
            }
        )
    }
}
