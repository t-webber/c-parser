#![doc = include_str!("../docs/README.md")]
// Rustc lint groups
#![warn(missing_docs)]
#![warn(warnings)]
#![warn(deprecated_safe)]
#![warn(future_incompatible)]
#![warn(keyword_idents)]
#![warn(let_underscore)]
#![warn(nonstandard_style)]
#![warn(refining_impl_trait)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2024_compatibility)]
#![warn(unused)]
// Clippy lint groups
#![warn(
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
#![allow(clippy::single_call_fn)]
#![allow(clippy::implicit_return)]
#![allow(clippy::pattern_type_mismatch)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::missing_trait_methods)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::mod_module_files)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::pub_with_shorthand)]
#![allow(clippy::unseparated_literal_suffix)]
#![allow(clippy::else_if_without_else)]
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
pub use crate::errors::api::{CompileError, Location, Res};
#[expect(clippy::useless_attribute, clippy::pub_use)]
pub use crate::lexer::api::{Number, TokenValue, display_tokens, lex_file};
#[expect(clippy::useless_attribute, clippy::pub_use)]
pub use crate::parser::api::parse_tokens;

/// String to represent the empty symbol, displayed for empty nodes.
const EMPTY: &str = "\u{2205} ";
