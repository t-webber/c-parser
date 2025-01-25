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
// Bad clippy lints
#![allow(
    clippy::single_call_fn,
    clippy::implicit_return,
    clippy::pattern_type_mismatch,
    clippy::blanket_clippy_restriction_lints,
    clippy::missing_trait_methods,
    clippy::question_mark_used,
    clippy::mod_module_files,
    clippy::module_name_repetitions,
    clippy::pub_with_shorthand,
    clippy::unseparated_literal_suffix,
    clippy::else_if_without_else
)]
// Disabled lints
#![allow(clippy::doc_include_without_cfg, reason = "see issue #13918")]
#![allow(clippy::exhaustive_enums)]
#![allow(clippy::allow_attributes_without_reason)]
#![allow(
    dropping_references,
    reason = "even though it does nothing, it prevents using the reference in the future."
)]
// Errors to manage
#![allow(
    clippy::panic,
    clippy::expect_used,
    clippy::unwrap_in_result,
    clippy::panic_in_result_fn
)]
// Features
#![feature(
    is_ascii_octdigit,
    f128,
    concat_idents,
    pattern,
    let_chains,
    try_trait_v2,
    const_vec_string_slice,
    coverage_attribute,
    stmt_expr_attributes
)]
#![allow(clippy::absolute_paths)]

mod errors;
mod lexer;
mod parser;

#[expect(clippy::useless_attribute, clippy::pub_use)]
pub use crate::errors::api::{LocationPointer, Res};
#[expect(clippy::useless_attribute, clippy::pub_use)]
pub use crate::lexer::api::{Number, TokenValue, display_tokens, lex_file};
#[expect(clippy::useless_attribute, clippy::pub_use)]
pub use crate::parser::api::parse_tokens;

/// String to represent the empty symbol, displayed for empty nodes.
const EMPTY: &str = "\u{2205} ";
