pub mod api {
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
