use core::fmt;

use crate::errors::location::Location;

use super::numbers::Number;

#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    // Unique
    Eol,
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
pub struct Token {
    location: Location,
    value: TokenValue,
}

impl Token {
    pub fn from_char(ch: char, location: &Location) -> Self {
        Self {
            value: TokenValue::Char(ch),
            location: location.to_owned(),
        }
    }

    pub fn from_identifier(identifier: String, location: &Location) -> Self {
        Self {
            location: location.to_owned().into_past(identifier.len()),
            value: TokenValue::Identifier(identifier),
        }
    }

    pub fn from_number(number: Number, location: &Location) -> Self {
        Self {
            value: TokenValue::Number(number),
            location: location.to_owned(),
        }
    }

    pub fn from_str(str: String, location: &Location) -> Self {
        Self {
            location: location.to_owned().into_past(str.len()),
            value: TokenValue::Str(str),
        }
    }

    pub fn from_symbol(symbol: Symbol, size: usize, location: &Location) -> Self {
        Self {
            value: TokenValue::Symbol(symbol),
            location: location.to_owned().into_past(size),
        }
    }

    pub fn into_value(self) -> TokenValue {
        self.value
    }

    pub const fn get_value(&self) -> &TokenValue {
        &self.value
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenValue {
    Char(char),
    Identifier(String),
    Number(Number),
    Str(String),
    Symbol(Symbol),
}
