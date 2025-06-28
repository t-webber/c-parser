//! Defines the unary operator nodes.

use super::operator::{Associativity, Operator};
use crate::parser::display::repr_option;
use crate::parser::tree::api::Ast;
use crate::utils::display;

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

display!(
    Ternary,
    self,
    f,
    write!(f, "({} ? {} : {})", self.condition, self.success, repr_option(&self.failure),)
);

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

display!(TernaryOperator, self, f, "?:".fmt(f));
