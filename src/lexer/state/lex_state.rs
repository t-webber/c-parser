use super::super::state::api::SymbolState;
use crate::lexer::types::api::Ident;

#[derive(Debug, PartialEq, Eq)]
pub enum CommentState {
    False,
    Star,
    True,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum LexingState {
    Char(Option<char>),
    Comment(CommentState),
    Identifier(Ident),
    #[default]
    StartOfLine,
    Str(String),
    Symbols(SymbolState),
    Unset,
}

impl LexingState {
    pub fn clear_last_symbol(&mut self) {
        if let Self::Symbols(symbol) = self {
            symbol.clear_last();
        } else {
            panic!("Didn't check if allowed before calling on symbol")
        }
    }

    pub fn new_ident(&mut self, ch: char) {
        *self = Self::Identifier(Ident::from(ch.to_string()));
    }

    pub fn new_ident_str(&mut self, str: String) {
        *self = Self::Identifier(Ident::from(str));
    }

    pub const fn repr(&self) -> &'static str {
        match self {
            Self::StartOfLine => "start of line",
            Self::Unset => "no context",
            Self::Symbols(_) => "symbols",
            Self::Identifier(_) => "identifier",
            Self::Char(_) => "char",
            Self::Str(_) => "string",
            Self::Comment(_) => "comment",
        }
    }

    pub const fn symbol(&self) -> Option<&SymbolState> {
        if let Self::Symbols(symbol) = self {
            Some(symbol)
        } else {
            None
        }
    }
}
