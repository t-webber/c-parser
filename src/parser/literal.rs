//! Defines the unary operator nodes.

use crate::Number;
use crate::errors::api::ErrorLocation;
use crate::parser::keyword::attributes::AttributeKeyword;
use crate::utils::display;

/// Attribute of a variable
#[derive(Debug)]
pub enum Attribute {
    /// Represents the `*` attribute
    Indirection,
    /// Keyword attribute, like `const` or `int`
    Keyword(AttributeKeyword, ErrorLocation),
    /// User-defined attribute, like a user defined type
    User(String),
}

display!(
    Attribute,
    self,
    f,
    match self {
        Self::Indirection => '*'.fmt(f),
        Self::Keyword(keyword, _) => keyword.fmt(f),
        Self::User(val) => val.fmt(f),
    }
);

/// Literal
#[derive(Debug)]
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

/// Display for a [`Vec<Attribute>`]
pub fn repr_vec_attr(vec: &[Attribute]) -> String {
    vec.iter()
        .map(Attribute::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}
