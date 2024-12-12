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
    clippy::try_err
)]
//
#![feature(is_ascii_octdigit, f128, concat_idents, pattern)]

mod errors;
mod lexer;
mod parser;
use errors::{compile::Res, display::display_errors, location::Location};
use lexer::lex_file;
use std::fs;

const SOURCE: &str = "data/test.c";

#[expect(clippy::print_stdout, clippy::panic, clippy::dbg_macro)]
fn main() {
    let content: &str = &fs::read_to_string(SOURCE)
        .unwrap_or_else(|_| panic!("The provided path is incorrect. No file found at {SOURCE}."));
    let files: &[(String, &str)] = &[(SOURCE.to_owned(), content)];
    let mut location = Location::from(SOURCE);
    let Res {
        result: tokens,
        errors,
    } = lex_file(content, &mut location);
    println!("{SOURCE} was parsed.");
    dbg!(&tokens);
    display_errors(errors, files);
}
