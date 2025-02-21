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
    /// Checks if an operator is  `[]`
    fn is_array_subscript(&self) -> bool {
        false
    }
    /// Checks if an operator is the `=` symbol
    #[coverage(off)]
    fn is_eq(&self) -> bool {
        false
    }
    /// Checks if an operator is the `*` symbol
    #[coverage(off)]
    fn is_star(&self) -> bool {
        false
    }
    /// Get precedence of an operator.
    fn precedence(&self) -> u32;
}
