use core::fmt;

use super::super::numbers::api::Number;
use super::keywords::{Keyword, TryKeywordType};
use super::lexing_data::LexingData;
use super::lexing_state::Ident;
use crate::errors::api::Location;

#[expect(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    // one character
    Ampersand,
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

    pub(crate) fn from_identifier(
        lex_data: &mut LexingData,
        literal: &mut Ident,
        location: &Location,
    ) -> Self {
        let len = literal.len();
        let value = literal.take_value();
        let token_value = match Keyword::from_value_or_res(&value) {
            TryKeywordType::Success(keyword) => TokenValue::Keyword(keyword),
            TryKeywordType::Deprecated(keyword) => {
                let new_keyword = value
                    .char_indices()
                    .filter_map(|(idx, ch)| {
                        if idx == 0 {
                            None
                        } else if idx == 1 {
                            Some(ch.to_ascii_lowercase())
                        } else {
                            Some(ch)
                        }
                    })
                    .collect::<String>();
                lex_data.push_err(location.to_owned().into_past(len).to_warning(format!("Underscore operators are deprecated since C23. Consider using the new keyword: {new_keyword}")));
                TokenValue::Keyword(keyword)
            }
            TryKeywordType::Failure => TokenValue::Identifier(value),
        };
        Self {
            location: location.to_owned().into_past(len),
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

    pub(crate) const fn get_location(&self) -> &Location {
        &self.location
    }

    #[inline]
    #[must_use]
    pub const fn get_value(&self) -> &TokenValue {
        &self.value
    }

    pub(crate) const fn get_value_mut(&mut self) -> &mut TokenValue {
        &mut self.value
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
