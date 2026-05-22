//! Module to define and handle operators

#[expect(clippy::inline_modules, reason = "clearer api")]
pub mod api {
    //! Api module to choose what functions to export.

    #![allow(clippy::pub_use, reason = "expose simple API")]

    pub use super::binary::{Binary, BinaryOperator};
    pub use super::conversions::OperatorConversions;
    pub use super::operator::{Associativity, Operator};
    pub use super::ternary::{Ternary, TernaryOperator};
    pub use super::unary::{Unary, UnaryOperator};
}

mod binary;
mod conversions;
mod operator;
mod ternary;
mod unary;
