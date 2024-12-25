use core::fmt;

use super::binary::BinaryOperator;
use super::unary::UnaryOperator;
use super::TernaryOperator;

trait ConvertNode<T>
where
    Self: Sized,
{
    fn try_convert_from(op: T) -> Result<Self, String>;
    fn try_clone_into(&mut self, op: T) -> Result<(), String> {
        *self = Self::try_convert_from(op)?;
        Ok(())
    }
}

#[cfg_attr(doc, doc = include_str!("../../../docs/operators.md"))]
pub trait Operator: fmt::Debug {
    fn associativity(&self) -> Associativity;
    fn precedence(&self) -> u32;
}

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

pub trait IsComma {
    fn is_comma(&self) -> bool;
}

impl IsComma for UnaryOperator {
    fn is_comma(&self) -> bool {
        false
    }
}

impl IsComma for BinaryOperator {
    fn is_comma(&self) -> bool {
        *self == Self::Comma
    }
}

impl IsComma for TernaryOperator {
    fn is_comma(&self) -> bool {
        false
    }
}
