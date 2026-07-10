//! Defines the unary operator nodes.

use core::hash::{Hash, Hasher};
use core::mem::discriminant;

use crate::Number;
use crate::parser::keyword::attributes::AttributeKeyword;
use crate::utils::display;

/// Helper macro to create a type attribute.
macro_rules! attr {
    ($y:ident $t:ident) => {
        $crate::parser::api::Attribute::Keyword($crate::parser::api::AttributeKeyword::$y(
            $crate::parser::api::$y::$t,
        ))
    };
}

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

impl Literal {
    /// Builds and returns the type of a literal.
    pub fn to_type(&self) -> Vec<Attribute> {
        let mut ty = vec![attr!(Qualifiers Const)];
        ty.extend(match self {
            Self::Char(_) => vec![attr!(BasicDataType Char)],
            Self::ConstantBool(_) => vec![attr!(BasicDataType Bool)],
            Self::Null => vec![
                attr!(BasicDataType Void),
                Attribute::Indirection,
                attr!(Qualifiers Const),
            ],
            Self::Str(_) => vec![
                attr!(BasicDataType Char),
                Attribute::Indirection,
                attr!(Qualifiers Const),
            ],
            Self::Number(Number::Int(_)) => vec![attr!(BasicDataType Int)],
            Self::Number(Number::Long(_)) => vec![attr!(Modifiers Long)],
            Self::Number(Number::LongLong(_)) =>
                vec![attr!(Modifiers Long), attr!( Modifiers Long)],
            Self::Number(Number::Float(_)) => vec![attr!(BasicDataType Float)],
            Self::Number(Number::Double(_)) => vec![attr!(BasicDataType Double)],
            Self::Number(Number::LongDouble(_)) =>
                vec![attr!(Modifiers Long), attr!(BasicDataType Double)],
            Self::Number(Number::UInt(_)) =>
                vec![attr!(Modifiers Unsigned), attr!(BasicDataType Int)],
            Self::Number(Number::ULong(_)) =>
                vec![attr!(Modifiers Unsigned), attr!(Modifiers Long)],
            Self::Number(Number::ULongLong(_)) => vec![
                attr!(Modifiers Unsigned),
                attr!(Modifiers Long),
                attr!(Modifiers Long),
            ],
        });
        ty
    }
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
