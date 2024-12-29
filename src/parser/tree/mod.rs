pub mod binary;
pub mod blocks;
mod conversions;
pub mod node;
mod traits;
pub mod unary;

use core::fmt;
use core::mem;

use node::Ast;
use traits::{Associativity, Operator};

use super::keyword::types::attributes::AttributeKeyword;
use super::keyword::types::functions::FunctionKeyword;
use crate::lexer::api::Number;
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

#[derive(Debug, PartialEq)]
pub enum Literal {
    Char(char),
    ConstantBool(bool),
    Empty,
    Nullptr,
    Number(Number),
    Str(String),
    Variable(Variable),
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => EMPTY.fmt(f),
            Self::Nullptr => "NULL".fmt(f),
            Self::Char(val) => val.fmt(f),
            Self::Str(val) => val.fmt(f),
            Self::Number(val) => val.fmt(f),
            Self::ConstantBool(val) => val.fmt(f),
            Self::Variable(val) => val.fmt(f),
        }
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

#[expect(clippy::min_ident_chars)]
impl fmt::Display for TernaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "?:".fmt(f)
    }
}

#[derive(Debug, PartialEq, Default, Eq)]
pub struct Variable {
    attrs: Vec<AttributeKeyword>,
    name: VariableName,
    user_type: Option<String>,
}

impl Variable {
    pub const fn from_keyword(keyword: FunctionKeyword) -> Self {
        Self {
            name: VariableName::Keyword(keyword),
            user_type: None,
            attrs: vec![],
        }
    }

    pub fn push_attr(&mut self, attr: AttributeKeyword) {
        self.attrs.push(attr);
    }

    pub fn push_str(&mut self, value: String) -> Result<(), String> {
        match mem::take(&mut self.name) {
            VariableName::Empty => {self.name = VariableName::UserDefined(value); Ok(())}, 
            VariableName::Keyword(keyword) => Err(format!("Found 2 successive literals, found identifier {value} after function keyword {keyword}.")),
            VariableName::UserDefined(old) => {
                self.user_type = Some(old);
                self.name = VariableName::UserDefined(value);
                Ok(())
            }
        }   
    }
}

impl From<String> for Variable {
    fn from(name: String) -> Self {
        Self {
            name: VariableName::UserDefined(name),
            user_type: None, 
            attrs: vec![],
        }
    }
}

impl From<AttributeKeyword> for Variable {
    fn from(attr: AttributeKeyword) -> Self {
        Self {
            name: VariableName::Empty,
            user_type: None, 
            attrs: vec![attr],
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.attrs.is_empty() {
            self.name.fmt(f)
        } else if let Some(usr) = &self.user_type {
            write!(
                f,
                "({} '{usr}' {})",
                self.attrs
                    .iter()
                    .map(|attr| format!("{attr}"))
                    .collect::<Vec<_>>()
                    .join(" "),
                self.name
            )
        } else {
            write!(
                f,
                "({} {})",
                self.attrs
                    .iter()
                    .map(|attr| format!("{attr}"))
                    .collect::<Vec<_>>()
                    .join(" "),
                self.name
            )
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum VariableName {
    #[default]
    Empty,
    Keyword(FunctionKeyword),
    UserDefined(String),
}

impl From<&str> for VariableName {
    fn from(name: &str) -> Self {
        Self::UserDefined(name.to_owned())
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for VariableName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => EMPTY.fmt(f),
            Self::UserDefined(val) => val.fmt(f),
            Self::Keyword(val) => val.fmt(f),
        }
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
