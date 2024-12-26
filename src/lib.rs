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
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::implicit_return)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::arithmetic_side_effects)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::mod_module_files)]
#![allow(clippy::print_stderr)]
#![allow(clippy::allow_attributes)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::ref_patterns)]
#![allow(clippy::allow_attributes_without_reason)]
#![allow(clippy::pattern_type_mismatch)]
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
    try_trait_v2
)]

mod errors;
mod lexer;
mod parser;

#[allow(clippy::pub_use, unused_imports)]
pub mod prelude {
    pub use crate::errors::location::Location;
    pub use crate::errors::result::Res;
    pub use crate::lexer::api::{number_types, tokens_types};
    pub use crate::lexer::lex_file;
    pub use crate::parser::parse_tokens;
}
