use core::fmt;

use super::binary::BinaryOperator;
use super::unary::UnaryOperator;
use super::TernaryOperator;

#[derive(Debug, PartialEq, Eq)]
pub enum Associativity {
    /// a+b+c is (a+b)+c
    ///
    /// a++-- is (a++)--
    LeftToRight,
    /// a=b=c is a=(b=c)
    ///
    /// !!a is !(!a)
    RightToLeft,
}

impl IsComma for BinaryOperator {
    fn is_comma(&self) -> bool {
        *self == Self::Comma
    }
}

pub trait IsComma {
    fn is_comma(&self) -> bool;
}

#[cfg_attr(doc, doc = include_str!("../../../docs/operators.md"))]
pub trait Operator: fmt::Debug {
    fn associativity(&self) -> Associativity;
    fn precedence(&self) -> u32;
}

impl IsComma for TernaryOperator {
    fn is_comma(&self) -> bool {
        false
    }
}

impl IsComma for UnaryOperator {
    fn is_comma(&self) -> bool {
        false
    }
}
