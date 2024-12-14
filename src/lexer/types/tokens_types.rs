use super::keywords::Keyword;
use super::lexing_data::LexingData;
use super::lexing_state::{Ident, LexingStatus};
use crate::{errors::location::Location, lexer::numbers::Number};
use core::fmt;

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
pub struct Token {
    location: Location,
    value: TokenValue,
}

impl Token {
    pub fn from_char(ch: char, location: &Location) -> Self {
        Self {
            value: TokenValue::Char(ch),
            location: location.to_owned().into_past(1),
        }
    }

    pub fn from_identifier(identifier: &mut Ident, location: &Location) -> Self {
        let value = identifier.take_value();
        let token_value = Keyword::try_from(value.as_str())
            .map_err(|()| value)
            .map_or_else(TokenValue::Identifier, TokenValue::Keyword);
        Self {
            location: location.to_owned().into_past(identifier.len()),
            value: token_value,
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

    pub fn into_value_location(self) -> (TokenValue, Location) {
        (self.value, self.location)
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
    Keyword(Keyword),
    Number(Number),
    Str(String),
    Symbol(Symbol),
}

pub struct LexingStruct<'lex_char> {
    data: &'lex_char mut LexingData,
    status: &'lex_char mut LexingStatus,
    location: &'lex_char Location,
}
