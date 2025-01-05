//! Defines the unary operator nodes.

use core::{fmt, mem};

use crate::parser::keyword::attributes::{AttributeKeyword, UserDefinedTypes};
use crate::parser::keyword::functions::FunctionKeyword;
use crate::{EMPTY, Number};

/// Attribute of a variable
#[derive(Debug, PartialEq, Eq)]
pub enum Attribute {
    /// Represents the `*` attribute
    Indirection,
    /// Keyword attribute, like `const` or `int`
    Keyword(AttributeKeyword),
    /// User-defined attribute, like a user defined type
    User(String),
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Indirection => '*'.fmt(f),
            Self::Keyword(keyword) => keyword.fmt(f),
            Self::User(val) => val.fmt(f),
        }
    }
}

/// Literal
#[derive(Debug, PartialEq)]
pub enum Literal {
    /// Char
    Char(char),
    /// Boolean constant: `true` or `false`
    ConstantBool(bool),
    /// `NULL` constant
    Nullptr,
    /// Number constant
    Number(Number),
    /// String constant
    Str(String),
    /// Variable
    Variable(Variable),
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nullptr => "NULL".fmt(f),
            Self::Char(val) => write!(f, "'{val}'"),
            Self::Str(val) => write!(f, "\"{val}\""),
            Self::Number(val) => val.fmt(f),
            Self::ConstantBool(val) => val.fmt(f),
            Self::Variable(val) => val.fmt(f),
        }
    }
}

/// Variable
#[derive(Debug, PartialEq, Default, Eq)]
pub struct Variable {
    /// attributes of the variable
    pub attrs: Vec<Attribute>,
    /// name of the variable
    pub name: VariableName,
}

impl Variable {
    /// Checks if a variable is in reality a type definition.
    ///
    /// `struct Name` is parsed as a variable attributes `struct` and `Name` and
    /// is waiting for the variable name. But if the next token is block, like
    /// in `struct Name {}`, it is meant as a control flow to define the type.
    pub fn get_typedef(&mut self) -> Result<Option<(&UserDefinedTypes, String)>, String> {
        if self.attrs.len() == 1
            && let Some(Attribute::Keyword(AttributeKeyword::UserDefinedTypes(user_type))) =
                self.attrs.last()
        {
            if let VariableName::UserDefined(name) = &mut self.name {
                Ok(Some((user_type, mem::take(name))))
            } else {
                Err(format!("Missing type name after {user_type:?}."))
            }
        } else {
            Ok(None)
        }
    }

    /// Adds an attribute to the variable
    fn push_attr(&mut self, attr: Attribute) -> Result<(), String> {
        match mem::take(&mut self.name) {
            VariableName::Empty => (),
            VariableName::Keyword(keyword) => {
                return Err(format!("Found {attr} after function keyword {keyword}."));
            }
            VariableName::UserDefined(name) => self.attrs.push(Attribute::User(name)),
        }
        self.attrs.push(attr);
        Ok(())
    }

    /// Adds a `*` indirection attribute to the variable
    pub fn push_indirection(&mut self) -> Result<(), String> {
        self.push_attr(Attribute::Indirection)
    }

    /// Adds a `*` indirection attribute to the variable
    pub fn push_keyword(&mut self, keyword: AttributeKeyword) -> Result<(), String> {
        self.push_attr(Attribute::Keyword(keyword))
    }

    /// Adds a non-keyword identifier to the variable
    ///
    /// An identifier can be meant as a user-defined type or as a variable name.
    pub fn push_name(&mut self, name: VariableName) -> Result<(), String> {
        match mem::take(&mut self.name) {
            VariableName::Empty => {
                self.name = name;
                Ok(())
            }
            VariableName::Keyword(keyword) => Err(format!(
                "Found 2 successive literals, found identifier {name} after function keyword {keyword}."
            )),
            VariableName::UserDefined(old) => {
                self.attrs.push(Attribute::User(old));
                self.name = name;
                Ok(())
            }
        }
    }
}

impl From<FunctionKeyword> for Variable {
    fn from(value: FunctionKeyword) -> Self {
        Self {
            name: VariableName::Keyword(value),
            attrs: vec![],
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

/// Variable name
#[derive(Debug, PartialEq, Eq, Default)]
pub enum VariableName {
    /// No variable name yet
    #[default]
    Empty,
    /// Function keyword, like `sizeof` or `alignof`
    Keyword(FunctionKeyword),
    /// User defined name: any identifier
    UserDefined(String),
}

impl From<&str> for VariableName {
    fn from(name: &str) -> Self {
        Self::UserDefined(name.to_owned())
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for VariableName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => EMPTY.fmt(f),
            Self::UserDefined(val) => val.fmt(f),
            Self::Keyword(val) => val.fmt(f),
        }
    }
}
