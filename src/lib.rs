#![doc = include_str!("../docs/README.md")]
#![feature(
    is_ascii_octdigit,
    f128,
    pattern,
    try_trait_v2,
    coverage_attribute,
    stmt_expr_attributes,
    macro_metavar_expr_concat,
    try_trait_v2_residual
)]

mod errors;
mod lexer;
mod parser;
mod utils;

#[expect(
    clippy::useless_attribute,
    clippy::pub_use,
    reason = "re-export for better API"
)]
pub use crate::errors::api::Res;
#[expect(
    clippy::useless_attribute,
    clippy::pub_use,
    reason = "re-export for better API"
)]
pub use crate::lexer::api::{Number, TokenValue, display_tokens, lex_file};
#[expect(
    clippy::useless_attribute,
    clippy::pub_use,
    reason = "re-export for better API"
)]
pub use crate::parser::api::parse_tokens;

/// String to represent an empty node when displaying the AST in a
/// human-readable way.
const EMPTY: &str = "\u{2205} ";
