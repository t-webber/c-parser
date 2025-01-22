//!Implement the `return` control flow

use core::fmt;

use crate::parser::keyword::control_flow::node::try_push_semicolon_control;
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::repr_fullness;
use crate::parser::types::Ast;

/// Keyword expects a node: `return 3+4`
#[derive(Debug, PartialEq, Default)]
pub struct ReturnCtrl {
    /// fullness of the value
    full: bool,
    /// [`Ast`] that is returned.
    value: Box<Ast>,
}

impl ControlFlow for ReturnCtrl {
    type Keyword = ();

    fn fill(&mut self) {
        self.full = true;
    }

    fn from_keyword((): Self::Keyword) -> Self {
        Self::default()
    }

    fn get_ast(&self) -> Option<&Ast> {
        (!self.full).then(|| self.value.as_ref())
    }

    fn get_mut(&mut self) -> Option<&mut Ast> {
        (!self.full).then(|| self.value.as_mut())
    }

    fn is_full(&self) -> bool {
        self.full
    }

    fn push_colon(&mut self) -> bool {
        false
    }

    fn push_semicolon(&mut self) -> bool {
        if self.full {
            false
        } else {
            if try_push_semicolon_control(&mut self.value) {
                if !self.value.can_push_leaf() {
                    self.full = true;
                }
            } else {
                self.full = true;
            }
            true
        }
    }
}

impl Push for ReturnCtrl {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf(&ast, self, "return");
        debug_assert!(!self.is_full(), "");
        self.value.push_block_as_leaf(ast)
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_op(&op, self, "return");
        debug_assert!(!self.is_full(), "");
        self.value.push_op(op)
    }
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for ReturnCtrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<return {}{}>", self.value, repr_fullness(self.full))
    }
}
