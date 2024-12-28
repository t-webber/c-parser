pub mod binary;
pub mod blocks;
mod conversions;
pub mod node;
mod traits;
pub mod unary;
use core::fmt;

use node::Ast;
use traits::{Associativity, Operator};

use crate::lexer::api::Number;

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
    args: Vec<Ast>,
    full: bool,
    name: String,
    op: FunctionOperator,
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}\u{b0}({}{}))",
            self.name,
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
    elts: Vec<Ast>,
    full: bool,
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
pub enum Literal {
    Char(char),
    ConstantBool(bool),
    #[default]
    Empty,
    Nullptr,
    Number(Number),
    Str(String),
    Variable(String),
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => "\u{2205} ".fmt(f),
            Self::Nullptr => "NULL".fmt(f),
            Self::Char(val) => val.fmt(f),
            Self::Number(val) => val.fmt(f),
            Self::ConstantBool(val) => val.fmt(f),
            Self::Variable(val) | Self::Str(val) => val.fmt(f),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Ternary {
    pub(super) condition: Box<Ast>,
    pub(super) failure: Option<Box<Ast>>,
    pub(super) op: TernaryOperator,
    pub(super) success: Box<Ast>,
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

#[expect(clippy::min_ident_chars)]
impl fmt::Display for TernaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "?:".fmt(f)
    }
}

#[expect(clippy::borrowed_box)]
fn repr_option_node(opt: Option<&Box<Ast>>) -> String {
    opt.map_or_else(|| "\u{2205} ".to_owned(), Box::<Ast>::to_string)
}

fn repr_vec_node(vec: &[Ast]) -> String {
    vec.iter()
        .map(|node| format!("{node}"))
        .collect::<Vec<_>>()
        .join(", ")
}
