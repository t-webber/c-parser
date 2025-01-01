//! Module to lex source files into a list of valid
//! [`Token`](types::api::Token): keywords, number constants,
//! identifiers, symbols, strings and chars.

pub mod api {
    //! Api module to choose what functions to export.

    #![allow(clippy::pub_use)]

    pub use super::lex_content::lex_file;
    pub use super::numbers::api::Number;
    pub use super::types::api::{Keyword, Symbol, Token, TokenValue, display_tokens};
}

mod lex_content;
mod numbers;
mod state;
mod types;
