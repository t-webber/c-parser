use core::fmt;
pub mod binary;
pub mod node;
pub mod unary;
use node::Node;
use unary::UnaryOperator;
mod conversions;

use crate::lexer::api::types::Number;

trait ConvertNode<T>
where
    Self: Sized,
{
    fn try_convert_from(op: T) -> Result<Self, String>;
    fn try_clone_into(&mut self, op: T) -> Result<(), String> {
        *self = Self::try_convert_from(op)?;
        Ok(())
    }
}

pub trait Operator: fmt::Debug {
    fn associativity(&self) -> Associativity;
    fn precedence(&self) -> u32;
}

enum Fix {
    Postfix,
    Prefix,
    Infix,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum Associativity {
    /// a+b+c is (a+b)+c
    ///
    /// a++-- is (a++)--
    LeftToRight,
    /// a=b=c is a=(b=c)
    ///
    /// !!a is !(!a)
    RightToLeft,
}

#[derive(Debug, PartialEq)]
pub struct CompoundLiteral {
    args: Vec<Node>,
    op: CompoundLiteralOperator,
    type_: String,
    full: bool,
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
    name: String,
    op: FunctionOperator,
    args: Vec<Node>,
    full: bool,
}

#[allow(clippy::min_ident_chars)]
impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, repr_vec_node(&self.args))
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
pub enum Literal {
    #[default]
    Empty,
    String(String),
    Variable(String),
    Char(char),
    Str(String),
    Number(Number),
}

#[allow(clippy::min_ident_chars)]
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => "\u{2205}".to_owned(),
                Self::String(val) | Self::Variable(val) | Self::Str(val) => val.to_string(),
                Self::Char(val) => val.to_string(),
                Self::Number(val) => format!("{val}"),
            }
        )
    }
}

#[allow(clippy::borrowed_box)]
fn repr_option_node(opt: Option<&Box<Node>>) -> String {
    opt.map_or_else(|| '\u{2205}'.to_string(), Box::<Node>::to_string)
}

fn repr_vec_node(vec: &[Node]) -> String {
    vec.iter()
        .map(|node| format!("{node}"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[derive(Debug, PartialEq, Default)]
pub struct Ternary {
    pub(super) op: TernaryOperator,
    pub(super) condition: Box<Node>,
    pub(super) success: Box<Node>,
    pub(super) failure: Option<Box<Node>>,
}

impl Ternary {
    fn get_last_mut(&mut self) -> &mut Box<Node> {
        match self {
            Self {
                failure: Some(val), ..
            }
            | Self { success: val, .. } => val,
        }
    }
}

#[allow(clippy::min_ident_chars)]
impl fmt::Display for Ternary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}?{}:{}",
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
