use core::{fmt, mem};

use crate::parser::keyword::types::attributes::AttributeKeyword;
use crate::parser::keyword::types::functions::FunctionKeyword;
use crate::{EMPTY, Number};

#[derive(Debug, PartialEq, Eq)]
pub enum Attribute {
    Indirection,
    Keyword(AttributeKeyword),
    User(String),
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Indirection => '*'.fmt(f),
            Self::Keyword(keyword) => keyword.fmt(f),
            Self::User(val) => write!(f, "'{val}'"),
        }
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
            Self::Char(val) => write!(f, "'{val}'"),
            Self::Str(val) => write!(f, "\"{val}\""),
            Self::Number(val) => val.fmt(f),
            Self::ConstantBool(val) => val.fmt(f),
            Self::Variable(val) => val.fmt(f),
        }
    }
}

#[derive(Debug, PartialEq, Default, Eq)]
pub struct Variable {
    pub attrs: Vec<Attribute>,
    pub name: VariableName,
}

impl Variable {
    pub const fn from_keyword(keyword: FunctionKeyword) -> Self {
        Self {
            name: VariableName::Keyword(keyword),
            attrs: vec![],
        }
    }

    pub fn push_attr(&mut self, attr: Attribute) {
        self.attrs.push(attr);
    }

    pub fn push_indirection(&mut self) -> Result<(), String> {
        match mem::take(&mut self.name) {
            VariableName::Empty => (),
            VariableName::Keyword(keyword) => {
                return Err(format!("Found '*' after function keyword {keyword}."));
            }
            VariableName::UserDefined(name) => self.attrs.push(Attribute::User(name)),
        }
        assert!(self.name == VariableName::Empty, "???");
        self.attrs.push(Attribute::Indirection);
        Ok(())
    }

    pub fn push_keyword(&mut self, keyword: AttributeKeyword) {
        self.attrs.push(Attribute::Keyword(keyword));
    }

    pub fn push_name(&mut self, name: VariableName) -> Result<(), String> {
        match mem::take(&mut self.name) {
            VariableName::Empty => {
                self.name = name;
                Ok(())
            }
            VariableName::Keyword(keyword) => Err(format!(
                "Found 2 successive literals, found identifier {name} after function keuword {keyword}."
            )),
            VariableName::UserDefined(old) => {
                self.attrs.push(Attribute::User(old));
                self.name = name;
                Ok(())
            }
        }
    }
}

impl From<String> for Variable {
    fn from(name: String) -> Self {
        Self {
            name: VariableName::UserDefined(name),
            attrs: vec![],
        }
    }
}

impl From<AttributeKeyword> for Variable {
    fn from(attr: AttributeKeyword) -> Self {
        Self {
            name: VariableName::Empty,
            attrs: vec![Attribute::Keyword(attr)],
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.attrs.is_empty() {
            self.name.fmt(f)
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
