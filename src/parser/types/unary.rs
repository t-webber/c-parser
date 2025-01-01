//! Defines the unary operator nodes.

use core::fmt;

use super::binary::BinaryOperator;
use super::{Associativity, Ast, Operator};

/// Unary operator node
#[derive(Debug, PartialEq)]
pub struct Unary {
    /// Argument
    pub arg: Box<Ast>,
    /// Operator
    pub op: UnaryOperator,
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.op.associativity() == Associativity::LeftToRight {
            write!(f, "({}{})", self.arg, self.op)
        } else {
            write!(f, "({}{})", self.op, self.arg)
        }
    }
}

/// Unary operator
#[derive(Debug, PartialEq, Eq)]
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

impl Operator for UnaryOperator {
    fn associativity(&self) -> Associativity {
        match self {
            Self::PostfixIncrement | Self::PostfixDecrement => Associativity::LeftToRight,
            Self::PrefixIncrement
            | Self::PrefixDecrement
            | Self::Plus
            | Self::Minus
            | Self::BitwiseNot
            | Self::LogicalNot
            | Self::Indirection
            | Self::AddressOf => Associativity::RightToLeft,
        }
    }

    fn precedence(&self) -> u32 {
        match self {
            Self::PostfixIncrement | Self::PostfixDecrement => 1,
            Self::PrefixIncrement
            | Self::PrefixDecrement
            | Self::Plus
            | Self::Minus
            | Self::BitwiseNot
            | Self::LogicalNot
            | Self::Indirection
            | Self::AddressOf => 2,
        }
    }
}

impl PartialEq<BinaryOperator> for UnaryOperator {
    fn eq(&self, _: &BinaryOperator) -> bool {
        false
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::PostfixIncrement | Self::PrefixIncrement => "++",
            Self::PostfixDecrement | Self::PrefixDecrement => "--",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::BitwiseNot => "~",
            Self::LogicalNot => "!",
            Self::Indirection => "*",
            Self::AddressOf => "&",
        })
    }
}
