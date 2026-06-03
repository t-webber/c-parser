//! Module to define and handle operators

pub use binary::{Binary, BinaryOperator};
pub use conversions::OperatorConversions;
pub use operator::{Associativity, Operator};
pub use ternary::{Ternary, TernaryOperator};
pub use unary::{Unary, UnaryOperator};

mod binary;
mod conversions;
mod operator;
mod ternary;
mod unary;
