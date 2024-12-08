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
//
#![allow(dead_code)]

mod errors;
mod parse;
mod tree;
use errors::location::Location;
use std::io::stdin;

#[expect(
    clippy::let_underscore_untyped,
    clippy::unwrap_used,
    clippy::print_stdout,
    clippy::use_debug
)]
fn main() {
    println!("Enter an expression.");
    let mut expression = String::new();
    let _ = stdin().read_line(&mut expression).unwrap();
    let mut location = Location::from("test_file.c");
    let tokens = parse::parse(&expression, &mut location).result;
    println!("Tokens = {tokens:?}");
}
