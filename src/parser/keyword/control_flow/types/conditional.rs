//! Implement the `if-else` control flow

use core::fmt;

use crate::parser::keyword::control_flow::keyword::ControlFlowKeyword;
use crate::parser::keyword::control_flow::node::try_push_semicolon_control;
use crate::parser::keyword::control_flow::pushable::PushableKeyword;
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::keyword::sort::PushInNode as _;
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
        #[cfg(feature = "debug")]
        println!("\tPushing {ast} in conditional {self}");
        debug_assert!(!self.is_full(), "");
        if let Some(failure) = &mut self.failure {
            if **failure == Ast::Empty && matches!(ast, Ast::BracedBlock(_)) {
                *failure = Box::new(ast);
                self.full_f = true;
                Ok(())
            } else {
                failure.push_block_as_leaf(ast)
            }
        } else if !self.full_s && self.condition.is_some() {
            if *self.success == Ast::Empty && matches!(ast, Ast::BracedBlock(_)) {
                self.success = Box::new(ast);
                self.full_s = true;
                Ok(())
            } else {
                self.success.push_block_as_leaf(ast)
            }
        } else if self.condition.is_none()
            && let Ast::ParensBlock(parens) = ast
        {
            self.condition = Some(parens);
            Ok(())
        } else {
            panic!("Tried to push to complete conditional")
        }
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        println!("\tPushing {op} in conditional {self}");
        debug_assert!(!self.is_full(), "");
        if let Some(failure) = &mut self.failure {
            failure.push_op(op)
        } else if self.full_s {
            Err("Missing else.".to_owned())
        } else if self.condition.is_none() {
            Err("Missing condition: expected (.".to_owned())
        } else {
            self.success.push_op(op)
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for ConditionCtrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<if {} {}{}{}{}>",
            repr_option(&self.condition),
            self.success,
            repr_fullness(self.full_s),
            self.failure
                .as_ref()
                .map_or_else(String::new, |failure| format!(" else {failure}")),
            if self.full_f { "" } else { ".\u{b2}." },
        )
    }
}
