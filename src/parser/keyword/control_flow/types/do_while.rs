//!Implement the `do-while` control flow

use core::fmt;

use crate::parser::keyword::control_flow::keyword::ControlFlowKeyword;
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::repr_option;
use crate::parser::types::{Ast, ParensBlock};

/// `do` keyword
#[derive(Debug, PartialEq, Default)]
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

    fn fill(&mut self) {}

    fn from_keyword((): Self::Keyword) -> Self {
        Self::default()
    }

    fn get_ast(&self) -> Option<&Ast> {
        self.while_found.then(|| self.loop_block.as_ref())
    }

    fn get_keyword(&self) -> ControlFlowKeyword {
        ControlFlowKeyword::Do
    }

    fn get_mut(&mut self) -> Option<&mut Ast> {
        self.while_found.then(|| self.loop_block.as_mut())
    }

    fn is_full(&self) -> bool {
        self.condition.is_some()
    }

    fn push_colon(&mut self) -> bool {
        false
    }
}

impl Push for DoWhileCtrl {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        debug_assert!(!self.is_full(), "");
        if self.while_found {
            debug_assert!(self.condition.is_none(), "");
            if let Ast::ParensBlock(parens) = ast {
                self.condition = Some(parens);
                Ok(())
            } else {
                Err("Missing condition: expect (.".to_owned())
            }
        } else {
            self.loop_block.push_block_as_leaf(ast)
        }
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        debug_assert!(!self.is_full(), "");
        if self.while_found {
            Err("Expected condition, found operator".to_owned())
        } else {
            self.loop_block.push_op(op)
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for DoWhileCtrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<do {}{}{}>",
            self.loop_block,
            if self.while_found { " while" } else { "" },
            repr_option(&self.condition),
        )
    }
}
