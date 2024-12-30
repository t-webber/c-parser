//! Module to define all the token types.
//!
//! This module provides the definition and methods of the types needed by
//! [`tokens::Token`].

pub mod api {
    //! Api module to choose what functions to export.

    #![allow(clippy::pub_use)]

    pub use super::escape::EscapeSequence;
    pub use super::keywords::Keyword;
    pub use super::lex_data::{LexingData, display_tokens};
    pub use super::symbols::Symbol;
    pub use super::tokens::{Ident, Token, TokenValue};
}

mod escape;
mod keywords;
mod lex_data;
mod symbols;
mod tokens;
