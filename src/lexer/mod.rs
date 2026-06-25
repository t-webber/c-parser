//! Module to lex source files into a list of valid
//! [`Token`](types::api::Token): keywords, number constants,
//! identifiers, symbols, strings and chars.

#[expect(clippy::inline_modules, reason = "clearer api")]
pub mod api {
    //! Api module to choose what functions to export.

    #![allow(clippy::pub_use, reason = "expose simple API")]

    pub use super::lex_content::lex;
    pub use super::numbers::api::Number;
    pub use super::types::api::{Keyword, StringId, Symbol, Token, TokenValue};
}

mod lex_content;
mod numbers;
mod state;
mod types;
