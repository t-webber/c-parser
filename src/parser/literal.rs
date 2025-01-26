//! Defines the unary operator nodes.

use core::fmt;

use crate::Number;
use crate::parser::keyword::attributes::AttributeKeyword;

/// Attribute of a variable
#[derive(Debug, PartialEq, Eq)]
pub enum Attribute {
    /// Represents the `*` attribute
    Indirection,
    /// Keyword attribute, like `const` or `int`
    Keyword(AttributeKeyword),
    /// User-defined attribute, like a user defined type
    User(String),
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Indirection => '*'.fmt(f),
            Self::Keyword(keyword) => keyword.fmt(f),
            Self::User(val) => val.fmt(f),
        }
    }
}

/// Literal
#[derive(Debug)]
pub enum Literal {
    /// Char
    Char(char),
    /// Boolean constant: `true` or `false`
    ConstantBool(bool),
    /// `NULL` constant
    Nullptr,
    /// Number constant
    Number(Number),
    /// String constant
    Str(String),
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nullptr => "NULL".fmt(f),
            Self::Char(val) => write!(f, "'{val}'"),
            Self::Str(val) => write!(f, "\"{val}\""),
            Self::Number(val) => val.fmt(f),
            Self::ConstantBool(val) => val.fmt(f),
        }
    }
}

/// Display for a [`Vec<Attribute>`]
pub fn repr_vec_attr(vec: &[Attribute]) -> String {
    vec.iter()
        .map(Attribute::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}
