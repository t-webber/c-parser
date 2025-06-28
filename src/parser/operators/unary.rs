//! Defines the unary operator nodes.

use super::operator::{Associativity, Operator};
use crate::parser::tree::api::Ast;
use crate::utils::display;

/// Unary operator node
#[derive(Debug)]
pub struct Unary {
    /// Argument
    pub arg: Box<Ast>,
    /// Operator
    pub op: UnaryOperator,
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

    #[coverage(off)] // never used: can't push star as unary in already formed type declaration
    fn is_star(&self) -> bool {
        *self == Self::Indirection
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
