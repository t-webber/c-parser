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
//
#![feature(is_ascii_octdigit)]

mod errors;
mod parse;
mod tree;
use errors::{display::display_errors, location::Location};
use std::{collections::HashMap, fs};

const SOURCE: &str = ".src/test.c";

#[expect(clippy::print_stdout, clippy::panic, clippy::iter_on_single_items)]
fn main() {
    let content: &str = &fs::read_to_string(SOURCE)
        .unwrap_or_else(|_| panic!("The provided path is incorrect. No file found at {SOURCE}."));
    let files: HashMap<String, &str> = [(SOURCE, content)]
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value))
        .collect();
    let mut location = Location::from(SOURCE);
    // let mut tokens = vec![];
    let mut errors = vec![];
    for line in content.lines() {
        let parsed = parse::parse(line.trim_end(), &mut location);
        // tokens.extend(parsed.result);
        errors.extend(parsed.errors);
        location.new_line();
    }
    println!("{SOURCE} was parsed.");
    display_errors(errors, files);
}
