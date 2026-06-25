//! Defines the unary operator nodes.

use super::operator::{Associativity, Operator};
use crate::EMPTY;
use crate::errors::api::ErrorLocation;
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
    ///
    /// The location is the location of the `:` symbol.
    pub failure: Option<(ErrorLocation, Box<Ast>)>,
    /// Success [`Ast`] (between `?` and ':')
    pub success: Box<Ast>,
}

impl Ternary {
    /// Computes the location of the ternary expression.
    pub fn location(&self) -> ErrorLocation {
        self.condition
            .location()
            .into_extended(self.failure.as_ref().map_or_else(
                || self.success.location(),
                |failure| {
                    if failure.1.is_empty() {
                        failure.0
                    } else {
                        failure.1.location()
                    }
                },
            ))
    }
}

display!(
    Ternary,
    self,
    f,
    write!(
        f,
        "({} ? {} : {})",
        self.condition,
        self.success,
        if let Some((_, fail)) = &self.failure {
            fail.to_string()
        } else {
            EMPTY.to_owned()
        }
    )
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
