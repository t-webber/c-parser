//
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
// lints that are not meant to be used
#![allow(clippy::single_call_fn)]
#![allow(clippy::implicit_return)]
#![allow(clippy::pattern_type_mismatch)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::missing_trait_methods)]
//
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::arithmetic_side_effects)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::mod_module_files)]
#![allow(clippy::allow_attributes)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::ref_patterns)]
#![allow(clippy::allow_attributes_without_reason)]
#![allow(clippy::string_add)]
#![allow(clippy::unseparated_literal_suffix)]
#![allow(clippy::pub_with_shorthand)]
//
#![allow(
    // errors to manage
    clippy::panic,
    clippy::unreachable,
    clippy::expect_used,
    clippy::unwrap_in_result,
    clippy::panic_in_result_fn,
    // doc
    clippy::missing_panics_doc,
    clippy::cargo_common_metadata,
    // unknown fix
    clippy::partial_pub_fields,
    clippy::field_scoped_visibility_modifiers,
    clippy::exhaustive_enums,
)]
//
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
pub use crate::lexer::api::{display_tokens, lex_file, Number, TokenValue};
#[expect(clippy::useless_attribute, clippy::pub_use)]
pub use crate::parser::api::parse_tokens;

const EMPTY: &str = "\u{2205} ";
