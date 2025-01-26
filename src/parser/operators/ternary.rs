//! Defines the unary operator nodes.

use core::fmt;

use super::operator::{Associativity, Operator};
use crate::parser::display::repr_option;
use crate::parser::tree::api::Ast;

/// Ternary node of an [`Ast`]
///
/// The structure is `<condition> ? <success> : <failure>.`
#[derive(Debug, Default)]
pub struct Ternary {
    /// Condition [`Ast`] (before `?`)
    pub condition: Box<Ast>,
    /// Failure [`Ast`] (after `:`)
    pub failure: Option<Box<Ast>>,
    /// Ternary operator
    ///
    /// This is a constant type, but is used to access the methods of the
    /// [`Operator`] trait.
    pub op: TernaryOperator,
    /// Success [`Ast`] (between `?` and ':')
    pub success: Box<Ast>,
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for Ternary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({} ? {} : {})",
            self.condition,
            self.success,
            repr_option(&self.failure),
        )
    }
}

/// Ternary operator
///
/// This is a constant type, but is used to access the methods of the
/// [`Operator`] trait.
#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct TernaryOperator;

impl Operator for TernaryOperator {
    fn associativity(&self) -> Associativity {
        Associativity::RightToLeft
    }

    fn precedence(&self) -> u32 {
        13
    }
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for TernaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "?:".fmt(f)
    }
}
