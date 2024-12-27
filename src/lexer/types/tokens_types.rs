use core::fmt;

use super::super::numbers::api::Number;
use super::keywords::Keyword;
use super::lexing_state::Ident;
use crate::errors::api::Location;

#[expect(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    // one character
    Ampercent,
    Assign,
    BitwiseNot,
    BitwiseOr,
    BitwiseXor,
    BraceClose,
    BraceOpen,
    BracketClose,
    BracketOpen,
    Colon,
    Comma,
    Divide,
    Dot,
    Gt,
    Interrogation,
    LogicalNot,
    Lt,
    Minus,
    Modulo,
    ParenthesisClose,
    ParenthesisOpen,
    Plus,
    SemiColon,
    Star,
    // two characters
    AddAssign,
    AndAssign,
    Arrow,
    Decrement,
    Different,
    DivAssign,
    Equal,
    Ge,
    Increment,
    Le,
    LogicalAnd,
    LogicalOr,
    ModAssign,
    MulAssign,
    OrAssign,
    ShiftLeft,
    ShiftRight,
    SubAssign,
    XorAssign,
    // three characters
    ShiftLeftAssign,
    ShiftRightAssign,
}

#[derive(Debug)]
pub struct Token {
    location: Location,
    value: TokenValue,
}

impl Token {
    pub(crate) fn from_char(ch: char, location: &Location) -> Self {
        Self {
            value: TokenValue::Char(ch),
            location: location.to_owned().into_past(1),
        }
    }

    pub(crate) fn from_identifier(identifier: &mut Ident, location: &Location) -> Self {
        let value = identifier.take_value();
        let token_value = Keyword::try_from(value.as_str())
            .map_err(|()| value)
            .map_or_else(TokenValue::Identifier, TokenValue::Keyword);
        Self {
            location: location.to_owned().into_past(identifier.len()),
            value: token_value,
        }
    }

    pub(crate) fn from_number(number: Number, location: &Location) -> Self {
        Self {
            value: TokenValue::Number(number),
            location: location.to_owned(),
        }
    }

    pub(crate) fn from_str(str: String, location: &Location) -> Self {
        Self {
            location: location.to_owned().into_past(str.len()),
            value: TokenValue::Str(str),
        }
    }

    pub(crate) fn from_symbol(symbol: Symbol, size: usize, location: &Location) -> Self {
        Self {
            value: TokenValue::Symbol(symbol),
            location: location.to_owned().into_past(size),
        }
    }

    #[inline]
    #[must_use]
    pub const fn get_value(&self) -> &TokenValue {
        &self.value
    }

    pub(crate) fn into_value_location(self) -> (TokenValue, Location) {
        (self.value, self.location)
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Token {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenValue {
    Char(char),
    Identifier(String),
    Keyword(Keyword),
    Number(Number),
    Str(String),
    Symbol(Symbol),
}

impl TokenValue {
    pub(self) const fn type_name(&self) -> &'static str {
        match self {
            Self::Char(_) => "Char",
            Self::Identifier(_) => "Ident",
            Self::Keyword(_) => "Keywd",
            Self::Str(_) => "Str",
            Self::Number(_) | Self::Symbol(_) => "\0",
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for TokenValue {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Char(arg0) => f.debug_tuple(self.type_name()).field(arg0).finish(),
            Self::Keyword(arg0) => f.debug_tuple(self.type_name()).field(arg0).finish(),
            Self::Number(arg0) => f.debug_tuple(self.type_name()).field(arg0).finish(),
            Self::Symbol(arg0) => f.debug_tuple(self.type_name()).field(arg0).finish(),
            Self::Identifier(arg0) | Self::Str(arg0) => {
                f.debug_tuple(self.type_name()).field(arg0).finish()
            }
        }
    }
}
