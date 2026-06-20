//! Defines the unary operator nodes.

use core::hash::{Hash, Hasher};
use core::mem::discriminant;

use crate::Number;
use crate::parser::keyword::attributes::AttributeKeyword;
use crate::utils::display;

/// Attribute of a variable
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Attribute {
    /// Represents the `*` attribute
    Indirection,
    /// Keyword attribute, like `const` or `int`
    Keyword(AttributeKeyword),
    /// User-defined attribute, like a user defined type
    User(String),
}

display!(
    Attribute,
    self,
    f,
    match self {
        Self::Indirection => '*'.fmt(f),
        Self::Keyword(keyword) => keyword.fmt(f),
        Self::User(val) => val.fmt(f),
    }
);

/// Literal
#[derive(Debug, PartialEq)]
pub enum Literal {
    /// Char
    Char(char),
    /// Boolean constant: `true` or `false`
    ConstantBool(bool),
    /// `NULL` constant
    Null,
    /// Number constant
    Number(Number),
    /// String constant
    Str(String),
}

impl Eq for Literal {}

impl Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
    }
}

display!(
    Literal,
    self,
    f,
    match self {
        Self::Null => "NULL".fmt(f),
        Self::Char(val) => write!(f, "'{val}'"),
        Self::Str(val) => write!(f, "\"{val}\""),
        Self::Number(val) => val.fmt(f),
        Self::ConstantBool(val) => val.fmt(f),
    }
);
