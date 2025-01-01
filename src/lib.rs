// Rustc lint groups
// #![warn(missing_docs)]
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
// Disabled lints
#![allow(clippy::exhaustive_enums)]
// TODO
#![allow(clippy::allow_attributes_without_reason)]
#![allow(clippy::arithmetic_side_effects)]
#![allow(clippy::unseparated_literal_suffix)]
#![allow(
    // errors to manage
    clippy::panic,
    clippy::expect_used,
    clippy::unwrap_in_result,
    clippy::panic_in_result_fn,
    // doc
    clippy::missing_docs_in_private_items,
)]
// Features
#![feature(
    is_ascii_octdigit,
    f128,
    concat_idents,
    pattern,
    let_chains,
    try_trait_v2,
    const_vec_string_slice
)]

mod errors;
mod lexer;
mod parser;

#[expect(clippy::useless_attribute, clippy::pub_use)]
pub use crate::errors::api::{CompileError, Location, Res};
#[expect(clippy::useless_attribute, clippy::pub_use)]
pub use crate::lexer::api::{Number, TokenValue, display_tokens, lex_file};
#[expect(clippy::useless_attribute, clippy::pub_use)]
pub use crate::parser::api::parse_tokens;

const EMPTY: &str = "\u{2205} ";
