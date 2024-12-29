pub mod binary;
pub mod blocks;
mod conversions;
pub mod literal;
pub mod node;
mod traits;
pub mod unary;

use core::fmt;

use binary::BinaryOperator;
use literal::Variable;
use node::Ast;
use traits::{Associativity, Operator};
use unary::UnaryOperator;

use crate::EMPTY;

#[derive(Debug, PartialEq)]
pub struct CompoundLiteral {
    args: Vec<Ast>,
    full: bool,
    op: CompoundLiteralOperator,
    type_: String,
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for CompoundLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}){{{}}}", self.type_, repr_vec_node(&self.args))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CompoundLiteralOperator;

impl Operator for CompoundLiteralOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> u32 {
        1
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub args: Vec<Ast>,
    pub full: bool,
    pub op: FunctionOperator,
    pub variable: Variable,
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}\u{b0}({}{}))",
            self.variable,
            repr_vec_node(&self.args),
            if self.full { "" } else { ".." },
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionOperator;

impl Operator for FunctionOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> u32 {
        1
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct ListInitialiser {
    pub elts: Vec<Ast>,
    pub full: bool,
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for ListInitialiser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.elts
                .iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

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
            repr_option_node(self.failure.as_ref()),
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

#[expect(clippy::borrowed_box)]
fn repr_option_node(opt: Option<&Box<Ast>>) -> String {
    opt.map_or_else(|| EMPTY.to_owned(), Box::<Ast>::to_string)
}

fn repr_vec_node(vec: &[Ast]) -> String {
    vec.iter()
        .map(|node| format!("{node}"))
        .collect::<Vec<_>>()
        .join(", ")
}
