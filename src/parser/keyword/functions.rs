//! Implements the function keywords

use super::sort::PushInNode;
use crate::parser::modifiers::push::Push as _;
use crate::parser::tree::Ast;
use crate::parser::variable::Variable;
use crate::utils::display;

/// List of existing function keywords
#[derive(Debug, PartialEq, Eq)]
pub enum FunctionKeyword {
    /// Alignof
    ///
    /// Returns the alignment, in bytes, of the input
    Alignof,
    /// Sizeof
    ///
    /// Yields the size in bytes of the object representation of the argument
    /// (the argument is of type type).
    // TODO: works without parents
    Sizeof,
    /// Static assert
    ///
    /// The constant expression is evaluated at compile time and compared to
    /// zero.
    StaticAssert,
}

impl PushInNode for FunctionKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_in_node(&self, "func keyword", node);
        node.push_block_as_leaf(Ast::Variable(Variable::from(self)))
    }
}

display!(
    FunctionKeyword,
    self,
    f,
    match self {
        Self::Alignof => "alignof".fmt(f),
        Self::Sizeof => "sizeof".fmt(f),
        Self::StaticAssert => "static_assert".fmt(f),
    }
);
