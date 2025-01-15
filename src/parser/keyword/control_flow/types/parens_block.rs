//!Implement the control flow with a parenthesised block and an ast, such as
//!`for`, `switch` and `while.`

use core::fmt;

use crate::parser::keyword::control_flow::keyword::ControlFlowKeyword;
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::types::{Ast, ParensBlock};
use crate::parser::{repr_fullness, repr_option};

/// Keyword expects a parenthesised block and a braced block: `switch (cond){}`
#[derive(Debug, PartialEq)]
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

    fn get_ast(&self) -> Option<&Ast> {
        self.full.then(|| self.block.as_ref())
    }

    fn get_keyword(&self) -> ControlFlowKeyword {
        match self.keyword {
            Self::Keyword::For => ControlFlowKeyword::For,
            Self::Keyword::Switch => ControlFlowKeyword::Switch,
            Self::Keyword::While => ControlFlowKeyword::While,
        }
    }

    fn get_mut(&mut self) -> Option<&mut Ast> {
        self.full.then(|| self.block.as_mut())
    }

    fn is_full(&self) -> bool {
        self.full
    }

    fn push_colon(&mut self) -> bool {
        false
    }
}

impl Push for ParensBlockCtrl {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        debug_assert!(!self.is_full(), "");
        if self.parens.is_some() {
            self.block.push_block_as_leaf(ast)
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
        debug_assert!(!self.is_full(), "");
        if self.parens.is_some() {
            self.block.push_op(op)
        } else {
            Err("Expected (, found operator".to_owned())
        }
    }
}

#[expect(clippy::min_ident_chars)]
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
impl fmt::Display for ParensBlockKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::For => "for".fmt(f),
            Self::Switch => "switch".fmt(f),
            Self::While => "while".fmt(f),
        }
    }
}
