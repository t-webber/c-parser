//! Module to lex source files into a list of valid
//! [`Token`](types::Token): keywords, number constants,
//! identifiers, symbols, strings and chars.
#![doc = include_str!("../../docs/README.md")]
#![feature(
    is_ascii_octdigit,
    f128,
    pattern,
    coverage_attribute,
    stmt_expr_attributes,
    macro_metavar_expr_concat
)]
#![allow(clippy::pub_use, reason = "expose simple API")]

mod lex_content;
mod numbers;
mod state;
mod types;

pub use lex_content::lex_file;
pub use numbers::Number;
pub use types::{Keyword, Symbol, Token, TokenValue, display_tokens};
