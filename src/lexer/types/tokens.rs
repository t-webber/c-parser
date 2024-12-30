use core::str::pattern;
use core::{fmt, mem};

use super::super::numbers::api::Number;
use super::super::types::api::LexingData;
use super::keywords::{Keyword, TryKeyword};
use super::symbols::Symbol;
use crate::errors::api::Location;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Ident(String);

impl Ident {
    pub fn contains<P: pattern::Pattern>(&self, pat: P) -> bool {
        self.0.contains(pat)
    }

    pub fn first(&self) -> Option<char> {
        self.0.chars().next()
    }

    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_number(&self) -> bool {
        self.first().unwrap_or('x').is_ascii_digit()
    }

    pub fn last_is_exp(&self) -> bool {
        self.is_number()
            && match self.0.chars().last() {
                Some('p' | 'P') => self.0.starts_with("0x"),
                Some('e' | 'E') => !self.0.starts_with("0x"), /* if the number expression starts with 0 and contains an exponent, the number is considered decimal, not octal. */
                Some(_) | None => false,
            }
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, ch: char) {
        self.0.push(ch);
    }

    pub fn take_value(&mut self) -> String {
        mem::take(&mut self.0)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<String> for Ident {
    fn from(value: String) -> Self {
        Self(value)
    }
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
            location: location.to_owned().into_past_with_length(1),
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
            TryKeyword::Success(keyword) => TokenValue::Keyword(keyword),
            TryKeyword::Deprecated(keyword) => {
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
                lex_data.push_err(location.to_owned().into_past_with_length(len).to_warning(format!("Underscore operators are deprecated since C23. Consider using the new keyword: {new_keyword}")));
                TokenValue::Keyword(keyword)
            }
            TryKeyword::Failure => TokenValue::Identifier(value),
        };
        Self {
            location: location.to_owned().into_past_with_length(len),
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
            location: location.to_owned().into_past_with_length(str.len()),
            value: TokenValue::Str(str),
        }
    }

    pub(crate) fn from_symbol(symbol: Symbol, size: usize, location: &Location) -> Self {
        Self {
            value: TokenValue::Symbol(symbol),
            location: location.to_owned().into_past_with_length(size),
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

#[expect(clippy::min_ident_chars, clippy::use_debug)]
impl fmt::Display for TokenValue {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Char(arg0) => write!(f, "'{arg0}'"),
            Self::Keyword(arg0) => write!(f, "Keyword({arg0})"),
            Self::Number(arg0) => write!(f, "{arg0}"),
            Self::Symbol(arg0) => write!(f, "{arg0:?}"),
            Self::Identifier(arg0) => write!(f, "Ident({arg0})"),
            Self::Str(arg0) => write!(f, "\"{arg0}\""),
        }
    }
}
