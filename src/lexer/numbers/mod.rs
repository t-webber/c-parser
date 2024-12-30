//! Module to parse numbers constants
//!
//! In a C program, number constants can be in lots of different forms like
//! `0123ull` or `0xfe2d.3ap-09L`. The goal of this module is to transform these
//! strings into their values, within the [`types::Number`] type.
//!
//! # Steps
//!
//! - Find the base (binary, hexadecimal, decimal or octal)
//! - Find the type (integer, float, double, is it unsigned ? long ? long long
//!   ?)
//! - Parse the string with the specified base and type
//! - If an overflow occurs, try to increase the type (only for integers).
//!
//! The main function of this module is [`from_literal::literal_to_number`].

pub mod api {
    //! Api module to choose what functions to export.

    #![allow(clippy::pub_use)]

    pub use super::from_literal::literal_to_number;
    pub(crate) use super::macros::safe_parse_int;
    pub use super::parse::OverParseRes;
    pub use super::types::Number;
}

mod base;
mod from_literal;
mod macros;
mod parse;
mod types;
