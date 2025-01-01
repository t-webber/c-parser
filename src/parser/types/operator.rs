//! Defines the operator trait.

use core::fmt;

/// Associativity of an operator
#[derive(Debug, PartialEq, Eq)]
pub enum Associativity {
    /// Left to right
    ///
    /// # Examples
    ///
    /// - a+b+c is (a+b)+c
    /// - a++-- is (a++)--
    LeftToRight,
    /// Right to left
    ///
    /// # Examples
    ///
    /// - a=b=c is a=(b=c)
    /// - !!a is !(!a)
    RightToLeft,
}

#[cfg_attr(doc, doc = include_str!("../../../docs/operators.md"))]
pub trait Operator: fmt::Debug {
    /// Get associativity of an operator.
    fn associativity(&self) -> Associativity;
    /// Get precedence of an operator.
    fn precedence(&self) -> u32;
}
