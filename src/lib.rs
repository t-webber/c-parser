#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery,
    // clippy::cargo
)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::implicit_return)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::missing_docs_in_private_items, clippy::arithmetic_side_effects)]
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
//
#![allow(
    dead_code,
    clippy::expect_used,
    clippy::arbitrary_source_item_ordering,
    clippy::panic,
    clippy::partial_pub_fields,
    clippy::panic_in_result_fn,
    clippy::try_err,
    clippy::field_scoped_visibility_modifiers,
    clippy::unwrap_in_result,
    clippy::useless_attribute,
    clippy::missing_panics_doc
)]
//
#![feature(is_ascii_octdigit, f128, concat_idents, pattern)]

mod errors;
mod lexer;
mod parser;

#[allow(clippy::pub_use, unused_imports)]
pub mod prelude {
    pub use crate::errors::{compile::Res, display::display_errors, location::Location};
    pub use crate::lexer::lex_file;
    pub use crate::parser::parse_tokens;
}
