//! Implements the function keywords

use core::fmt;

use super::Ast;
use super::sort::PushInNode;
use crate::parser::modifiers::push::Push as _;
use crate::parser::types::variable::Variable;

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
    // TODO: works without parens
    Sizeof,
    /// Static assert
    ///
    /// The constant expression is evaluated at compile time and compared to
    /// zero.
    StaticAssert,
    /// Type of
    ///
    /// Returns the type of a variable, *with* the qualifiers
    ///
    /// # Examples
    ///
    /// `typeof(const int)` is `const int`
    ///
    /// ```c
    /// static long int a;
    /// typeof(a) b = 1; // b is of type `static long int`
    /// ```
    Typeof,
    /// Type of unqualified
    ///
    /// Returns the type of a variable, *without* the qualifiers
    ///
    /// # Examples
    ///
    /// `typeof(const int)` is `int`
    ///
    /// ```c
    /// static long int a;
    /// typeof(a) b = 1; // b is of type `long int`
    /// ```
    TypeofUnqual,
}

impl PushInNode for FunctionKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_in_node(&self, "func keyword", node);
        node.push_block_as_leaf(Ast::Variable(Variable::from(self)))
    }
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for FunctionKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alignof => "alignof".fmt(f),
            Self::Sizeof => "sizeof".fmt(f),
            Self::StaticAssert => "static_assert".fmt(f),
            Self::Typeof => "typeof".fmt(f),
            Self::TypeofUnqual => "typeof_unqual".fmt(f),
        }
    }
}
