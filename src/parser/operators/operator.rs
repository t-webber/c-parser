//! Defines the operator trait.

use core::fmt;

use crate::errors::api::ErrorLocation;

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
#[allow(
    clippy::missing_docs_in_private_items,
    clippy::allow_attributes,
    reason = "clippy bug"
)]
pub trait Operator: fmt::Debug {
    /// Checks if an operator is the `*` symbol
    fn as_star(&self) -> Option<&ErrorLocation> {
        None
    }

    /// Get associativity of an operator.
    fn associativity(&self) -> Associativity;

    /// Checks if an operator is  `[]`
    fn is_array_subscript(&self) -> bool {
        false
    }

    /// Checks if an operator is the `=` symbol
    fn is_eq(&self) -> bool {
        false
    }

    /// Get precedence of an operator.
    fn precedence(&self) -> u32;
}
