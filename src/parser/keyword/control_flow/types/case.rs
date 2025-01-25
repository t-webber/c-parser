//!Implement the `case` control flow

use core::{fmt, mem};

use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::keyword::control_flow::types::repr_colon_option;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::repr_fullness;
use crate::parser::types::Ast;
use crate::parser::types::braced_blocks::BracedBlock;

/// Keyword expects a colon and a node: `case x: y`
#[derive(Debug, Default)]
pub struct AstColonAstCtrl {
    /// [`Ast`] after the colon
    after: Option<Box<Ast>>,
    /// [`Ast`] before the colon
    before: Box<Ast>,
    /// fullness of the control flow
    full: bool,
}

impl ControlFlow for AstColonAstCtrl {
    type Keyword = ();

    fn as_ast(&self) -> Option<&Ast> {
        (!self.full).then(|| &**self.after.as_ref().unwrap_or(&self.before))
    }

    fn as_ast_mut(&mut self) -> Option<&mut Ast> {
        (!self.full).then(|| &mut **self.after.as_mut().unwrap_or(&mut self.before))
    }

    fn fill(&mut self) {
        self.full = true;
    }

    fn from_keyword((): Self::Keyword) -> Self {
        Self::default()
    }

    fn is_full(&self) -> bool {
        self.full
    }

    fn push_colon(&mut self) -> bool {
        if self.after.is_none() && !self.is_full() {
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
            } else if !ast.is_empty() {
                *ast = Ast::BracedBlock(BracedBlock {
                    elts: vec![mem::take(ast), Ast::Empty],
                    full: false,
                })
                .into_box();
            }
            true
        } else {
            false
        }
    }
}

impl Push for AstColonAstCtrl {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf(&ast, self, "case");
        debug_assert!(!self.is_full(), "");
        if let Some(after) = &mut self.after {
            after.push_block_as_leaf(ast)
        } else {
            self.before.push_block_as_leaf(ast)
        }
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_op(&op, self, "case");
        debug_assert!(!self.is_full(), "");
        if let Some(after) = &mut self.after {
            after.push_op(op)
        } else {
            self.before.push_op(op)
        }
    }
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for AstColonAstCtrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<case {}{}{}>",
            self.before,
            repr_colon_option(self.after.as_ref()),
            repr_fullness(self.full)
        )
    }
}
