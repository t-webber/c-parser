//! Module implementation for variable considered as names, i.e., that don't
//! contain attributes.

use core::fmt;

use super::Variable;
use crate::EMPTY;
use crate::parser::keyword::functions::FunctionKeyword;
use crate::parser::types::Ast;
use crate::parser::types::literal::Attribute;

impl TryFrom<VariableName> for Ast {
    type Error = ();

    fn try_from(value: VariableName) -> Result<Self, ()> {
        Ok(match value {
            VariableName::Empty => return Err(()),
            VariableName::Keyword(keyword) => Self::Variable(Variable::from(keyword)),
            VariableName::UserDefined(name) => Self::Variable(Variable::from(name)),
        })
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

impl VariableName {
    /// Transform a [`VariableName`] into an [`Attribute`]
    pub fn into_attr(self) -> Result<Option<Attribute>, String> {
        match self {
            Self::Empty => Ok(None),
            Self::Keyword(keyword) => Err(format!(
                "Tried to transform function keyword {keyword} to an attribute"
            )),
            Self::UserDefined(name) => Ok(Some(Attribute::User(name))),
        }
    }

    /// Checks if a variable is a user defined variable
    pub const fn is_user_variable(&self) -> bool {
        matches!(self, Self::UserDefined(_))
    }
}

impl From<&str> for VariableName {
    fn from(name: &str) -> Self {
        Self::UserDefined(name.to_owned())
    }
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for VariableName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => EMPTY.fmt(f),
            Self::UserDefined(val) => val.fmt(f),
            Self::Keyword(val) => val.fmt(f),
        }
    }
}
