//!Implement the `do-while` control flow

use core::fmt;

use crate::parser::keyword::control_flow::keyword::ControlFlowKeyword;
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::modifiers::ast::AstPushContext;
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
