#![doc = include_str!("../docs/README.md")]
#![warn(
    missing_docs,
    warnings,
    deprecated_safe,
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unused,
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
#![expect(clippy::blanket_clippy_restriction_lints, reason = "enable all lints")]
#![expect(
    clippy::single_call_fn,
    clippy::implicit_return,
    clippy::pattern_type_mismatch,
    clippy::question_mark_used,
    clippy::else_if_without_else,
    clippy::missing_trait_methods,
    reason = "bad lints"
)]
#![allow(clippy::missing_inline_in_public_items, reason = "bad lint")]
#![expect(
    clippy::mod_module_files,
    clippy::module_name_repetitions,
    clippy::pub_with_shorthand,
    clippy::unseparated_literal_suffix,
    reason = "style"
)]
#![expect(clippy::doc_include_without_cfg, reason = "see issue #13918")]
#![expect(
    dropping_references,
    reason = "even though it does nothing, it prevents using the reference in the future."
)]
#![expect(
    clippy::panic,
    clippy::expect_used,
    clippy::unwrap_in_result,
    clippy::panic_in_result_fn,
    reason = "unreachable patterns"
)]
#![feature(
    is_ascii_octdigit,
    f128,
    pattern,
    let_chains,
    try_trait_v2,
    coverage_attribute,
    stmt_expr_attributes,
    macro_metavar_expr_concat
)]

mod errors;
mod lexer;
mod parser;

#[expect(
    clippy::useless_attribute,
    clippy::pub_use,
    reason = "re-export for better API"
)]
pub use crate::errors::api::{LocationPointer, Res};
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
