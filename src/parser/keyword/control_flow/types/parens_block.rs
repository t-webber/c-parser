//!Implement the control flow with a parenthesised block and an ast, such as
//!`for`, `switch` and `while.`

use core::fmt;

use crate::parser::keyword::control_flow::node::try_push_semicolon_control;
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::modifiers::ast::can_push::AstPushContext;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::types::Ast;
use crate::parser::types::parens::ParensBlock;
use crate::parser::{repr_fullness, repr_option};

/// Keyword expects a parenthesised block and a braced block: `switch (cond){}`
#[derive(Debug)]
pub struct ParensBlockCtrl {
    /// Block expression after parens
    block: Box<Ast>,
    /// Fullness of the block
    full: bool,
    /// Control flow keyword
    keyword: ParensBlockKeyword,
    /// Parens expression
    parens: Option<ParensBlock>,
}

impl ControlFlow for ParensBlockCtrl {
    type Keyword = ParensBlockKeyword;

    fn as_ast(&self) -> Option<&Ast> {
        (!self.full).then(|| self.block.as_ref())
    }

    fn as_ast_mut(&mut self) -> Option<&mut Ast> {
        (!self.full).then(|| self.block.as_mut())
    }

    fn fill(&mut self) {
        self.full = true;
    }

    fn from_keyword(keyword: Self::Keyword) -> Self {
        Self {
            keyword,
            parens: None,
            block: Ast::empty_box(),
            full: false,
        }
    }

    fn is_full(&self) -> bool {
        self.full
    }

    fn is_switch(&self) -> bool {
        self.keyword == Self::Keyword::Switch
    }

    fn is_while(&self) -> bool {
        self.keyword == Self::Keyword::While
    }

    fn push_colon(&mut self) -> bool {
        false
    }

    fn push_semicolon(&mut self) -> bool {
        if self.full {
            false
        } else {
            if try_push_semicolon_control(&mut self.block) {
                if !self.block.can_push_leaf_with_ctx(AstPushContext::Any) {
                    self.full = true;
                }
            } else {
                self.full = true;
            }
            true
        }
    }
}

impl Push for ParensBlockCtrl {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf(&ast, self, "parens block");
        debug_assert!(!self.is_full(), "");
        if self.parens.is_some() {
            if matches!(ast, Ast::BracedBlock(_)) {
                if self.block.is_empty() {
                    self.block = ast.into_box();
                    self.full = true;
                } else {
                    self.block.push_braced_block(ast)?;
                    if !self
                        .block
                        .can_push_leaf_with_ctx(AstPushContext::UserVariable)
                    {
                        self.full = true;
                    }
                }
            } else {
                self.block.push_block_as_leaf(ast)?;
            }
            Ok(())
        } else if let Ast::ParensBlock(parens) = ast {
            self.parens = Some(parens);
            Ok(())
        } else {
            Err("Missing (.".to_owned())
        }
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_op(&op, self, "parens block");
        debug_assert!(!self.is_full(), "");
        if self.parens.is_some() {
            self.block.push_op(op)
        } else {
            Err("Expected (, found operator".to_owned())
        }
    }
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for ParensBlockCtrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<{} {} {}{}>",
            self.keyword,
            repr_option(&self.parens),
            self.block,
            repr_fullness(self.full),
        )
    }
}

/// C control flow keywords that have the [`ParensBlockCtrl`] structure.
#[derive(Debug, PartialEq, Eq)]
pub enum ParensBlockKeyword {
    /// `for (...) { }`
    For,
    /// `switch (...) { }`
    Switch,
    /// `while (...) { }`
    While,
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for ParensBlockKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::For => "for".fmt(f),
            Self::Switch => "switch".fmt(f),
            Self::While => "while".fmt(f),
        }
    }
}
