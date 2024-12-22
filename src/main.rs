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
    clippy::unwrap_in_result
)]
//
#![feature(is_ascii_octdigit, f128, concat_idents, pattern)]

mod errors;
mod lexer;
mod parser;
#[cfg(test)]
mod test;
use errors::{compile::Res, display::display_errors, location::Location};
use lexer::lex_file;
use parser::parse_tokens;
use std::{env, fs};

const DIR: &str = "./data/";

#[expect(clippy::panic, clippy::print_stdout, clippy::use_debug)]
fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| "test".to_owned());
    let path = format!("{DIR}{filename}.c");
    let content: &str = &fs::read_to_string(&path).unwrap_or_else(|_| {
        panic!(
            "The provided path is incorrect. No file found at {}.",
            &path
        )
    });
    let files: &[(String, &str)] = &[(path.clone(), content)];
    let mut location = Location::from(path);
    let Res {
        result: tokens,
        errors: lex_errors,
    } = lex_file(content, &mut location);
    if lex_errors.is_empty() {
        println!("{tokens:?}");
        let Res {
            result: ast,
            errors: pars_errors,
        } = parse_tokens(tokens);
        println!("AST = \n{ast}");
        display_errors(pars_errors, files, "parsing");
    } else {
        display_errors(lex_errors, files, "lexing");
    }
}
