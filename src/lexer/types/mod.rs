pub mod api {
    #![allow(clippy::pub_use)]

    pub use super::escape::EscapeSequence;
    pub use super::keywords::Keyword;
    pub use super::lex_data::{display_tokens, LexingData};
    pub use super::symbols::Symbol;
    pub use super::tokens::{Ident, Token, TokenValue};
}

mod escape;
mod keywords;
mod lex_data;
mod symbols;
mod tokens;
