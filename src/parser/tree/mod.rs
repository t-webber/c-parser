pub mod binary;
mod conversions;
pub mod node;
mod traits;
pub mod unary;
use core::fmt;

use node::Node;
use traits::{Associativity, Operator};

use crate::lexer::api::number_types::Number;

#[derive(Debug, PartialEq)]
pub struct CompoundLiteral {
    args: Vec<Node>,
    full: bool,
    op: CompoundLiteralOperator,
    type_: String,
}

#[allow(clippy::min_ident_chars)]
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
    args: Vec<Node>,
    full: bool,
    name: String,
    op: FunctionOperator,
}

#[allow(clippy::min_ident_chars)]
impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\u{b0}{}\u{b0}({})",
            self.name,
            repr_vec_node(&self.args)
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
    elts: Vec<Node>,
    full: bool,
}

#[allow(clippy::min_ident_chars)]
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
    #[default]
    Empty,
    Number(Number),
    Str(String),
    Variable(String),
}

#[allow(clippy::min_ident_chars)]
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => "\u{2205} ".to_owned(),
                Self::Variable(val) | Self::Str(val) => val.to_string(),
                Self::Char(val) => val.to_string(),
                Self::Number(val) => format!("{val}"),
            }
        )
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Ternary {
    pub(super) condition: Box<Node>,
    pub(super) failure: Option<Box<Node>>,
    pub(super) op: TernaryOperator,
    pub(super) success: Box<Node>,
}

#[allow(clippy::min_ident_chars)]
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

#[allow(clippy::min_ident_chars)]
impl fmt::Display for TernaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "?:".fmt(f)
    }
}

#[allow(clippy::borrowed_box)]
fn repr_option_node(opt: Option<&Box<Node>>) -> String {
    opt.map_or_else(|| "\u{2205} ".to_owned(), Box::<Node>::to_string)
}

fn repr_vec_node(vec: &[Node]) -> String {
    vec.iter()
        .map(|node| format!("{node}"))
        .collect::<Vec<_>>()
        .join(", ")
}
