//! Defines the unary operator nodes.

use super::operator::{Associativity, Operator};
use crate::errors::api::{ErrorLocation, Located};
use crate::parser::tree::api::Ast;
use crate::utils::display;

/// Unary operator node
#[derive(Debug)]
pub struct Unary {
    /// Argument
    pub arg: Box<Ast>,
    /// Operator
    pub op: Located<UnaryOperator>,
}

display!(Unary, self, f, {
    if self.op.associativity() == Associativity::LeftToRight {
        write!(f, "({}{})", self.arg, self.op)
    } else {
        write!(f, "({}{})", self.op, self.arg)
    }
});

/// Unary operator
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnaryOperator {
    /// Address-of (`&`)
    AddressOf,
    /// `~`
    BitwiseNot,
    /// Dereference (`*`)
    Indirection,
    /// `!`
    LogicalNot,
    /// `-`
    Minus,
    /// `+`
    Plus,
    /// `--` (in `x--`)
    PostfixDecrement,
    /// `++` (in `x++`)
    PostfixIncrement,
    /// `--` (in `--x`)
    PrefixDecrement,
    /// `++` (in `++x`)
    PrefixIncrement,
}

impl Operator for Located<UnaryOperator> {
    fn as_star(&self) -> Option<&ErrorLocation> {
        (*self.as_value() == UnaryOperator::Indirection).then(|| self.as_location())
    }

    fn associativity(&self) -> Associativity {
        match self.as_value() {
            UnaryOperator::PostfixIncrement | UnaryOperator::PostfixDecrement =>
                Associativity::LeftToRight,
            UnaryOperator::PrefixIncrement
            | UnaryOperator::PrefixDecrement
            | UnaryOperator::Plus
            | UnaryOperator::Minus
            | UnaryOperator::BitwiseNot
            | UnaryOperator::LogicalNot
            | UnaryOperator::Indirection
            | UnaryOperator::AddressOf => Associativity::RightToLeft,
        }
    }

    fn precedence(&self) -> u32 {
        match self.as_value() {
            UnaryOperator::PostfixIncrement | UnaryOperator::PostfixDecrement => 1,
            UnaryOperator::PrefixIncrement
            | UnaryOperator::PrefixDecrement
            | UnaryOperator::Plus
            | UnaryOperator::Minus
            | UnaryOperator::BitwiseNot
            | UnaryOperator::LogicalNot
            | UnaryOperator::Indirection
            | UnaryOperator::AddressOf => 2,
        }
    }
}

display!(UnaryOperator, self, f, {
    write!(
        f,
        "{}",
        match self {
            Self::PostfixIncrement | Self::PrefixIncrement => "++",
            Self::PostfixDecrement | Self::PrefixDecrement => "--",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::BitwiseNot => "~",
            Self::LogicalNot => "!",
            Self::Indirection => "*",
            Self::AddressOf => "&",
        }
    )
});
