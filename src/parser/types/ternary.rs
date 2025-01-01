use core::fmt;

use super::Ast;
use super::binary::BinaryOperator;
use super::operator::{Associativity, Operator};
use super::unary::UnaryOperator;
use crate::parser::repr_option;

#[derive(Debug, PartialEq, Default)]
pub struct Ternary {
    pub condition: Box<Ast>,
    pub failure: Option<Box<Ast>>,
    pub op: TernaryOperator,
    pub success: Box<Ast>,
}

#[expect(clippy::min_ident_chars)]
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

#[derive(Debug, PartialEq, Eq, Default)]
pub struct TernaryOperator;

impl Operator for TernaryOperator {
    fn associativity(&self) -> Associativity {
        Associativity::RightToLeft
    }

    fn precedence(&self) -> u32 {
        13
    }
}

impl PartialEq<BinaryOperator> for TernaryOperator {
    fn eq(&self, _: &BinaryOperator) -> bool {
        false
    }
}

impl PartialEq<UnaryOperator> for TernaryOperator {
    fn eq(&self, _: &UnaryOperator) -> bool {
        false
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for TernaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "?:".fmt(f)
    }
}
