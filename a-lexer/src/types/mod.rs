//! Module to define all the token types.
//!
//! This module provides the definition and methods of the types needed by
//! [`tokens::Token`].

mod escape;
mod keywords;
mod lex_data;
mod symbols;
mod tokens;

pub use escape::EscapeSequence;
pub use keywords::Keyword;
pub use lex_data::{LexingData, display_tokens};
pub use symbols::Symbol;
pub use tokens::{Ident, Token, TokenValue};
