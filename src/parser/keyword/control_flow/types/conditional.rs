//!Implement the `if-else` control flow

use core::fmt;

use crate::parser::keyword::control_flow::keyword::ControlFlowKeyword;
use crate::parser::keyword::control_flow::node::try_push_semicolon_control;
use crate::parser::keyword::control_flow::pushable::PushableKeyword;
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::keyword::sort::PushInNode as _;
use crate::parser::modifiers::ast::AstPushContext;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::types::{Ast, ParensBlock};
use crate::parser::{repr_fullness, repr_option};

/// `if` keyword
#[derive(Debug, PartialEq, Default)]
pub struct ConditionCtrl {
    /// condition expression inside parenthesis
    condition: Option<ParensBlock>,
    /// block executed if the condition is false
    failure: Option<Box<Ast>>,
    /// fullness of the failure block
    full_f: bool,
    /// fullness of the success block
    full_s: bool,
    /// block executed if the condition is a success
    success: Box<Ast>,
}

impl ConditionCtrl {
    /// Checks if the control flow is waiting for an `if` keyword
    pub const fn no_else(&self) -> bool {
        self.condition.is_some() && self.failure.is_none() && !self.full_f
    }

    /// Push the `else` keyword in an `if` control flow.
    pub fn push_else(&mut self) -> Result<(), String> {
        if self.full_f {
            panic!("tried to push on panic")
        } else if self.condition.is_none() {
            Err("missing condition: missing `(` after `if`".to_owned())
        } else if *self.success == Ast::Empty {
            Err("missing success block after `if` condition".to_owned())
        } else if let Some(fail) = &mut self.failure {
            PushableKeyword::Else.push_in_node(fail)
        } else {
            self.full_s = true;
            self.failure = Some(Ast::empty_box());
            Ok(())
        }
    }
}

impl ControlFlow for ConditionCtrl {
    type Keyword = ();

    fn fill(&mut self) {
        if self.full_s {
            self.full_f = true;
        } else {
            self.full_s = true;
        }
    }

    fn from_keyword((): Self::Keyword) -> Self {
        Self::default()
    }

    fn get_ast(&self) -> Option<&Ast> {
        Some(self.failure.as_ref().unwrap_or(&self.success))
    }

    fn get_keyword(&self) -> ControlFlowKeyword {
        ControlFlowKeyword::If
    }

    fn get_mut(&mut self) -> Option<&mut Ast> {
        Some(self.failure.as_mut().unwrap_or(&mut self.success))
    }

    fn is_complete(&self) -> bool {
        (self.full_s && self.failure.is_none()) || self.full_f
    }

    fn is_full(&self) -> bool {
        self.full_f
    }

    fn push_colon(&mut self) -> bool {
        false
    }

    fn push_semicolon(&mut self) -> bool {
        let push = |ast: &mut Ast, full: &mut bool| {
            if try_push_semicolon_control(ast) {
                if !ast.can_push_leaf() {
                    *full = true;
                }
            } else {
                ast.fill();
                *full = true;
            }
            true
        };

        if self.full_f {
            false
        } else if self.full_s {
            if let Some(ast) = &mut self.failure {
                push(ast, &mut self.full_f)
            } else {
                false
            }
        } else {
            push(&mut self.success, &mut self.full_s)
        }
    }
}

impl Push for ConditionCtrl {
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
impl fmt::Display for ConditionCtrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<if {} {}{} else {}{}>",
            repr_option(&self.condition),
            self.success,
            repr_fullness(self.full_s),
            repr_option(&self.failure),
            if self.full_f { "" } else { ".\u{b2}." }
        )
    }
}
