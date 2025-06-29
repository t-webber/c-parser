//! Module implementation for variable considered as names, i.e., that don't
//! contain attributes.

use crate::EMPTY;
use crate::parser::keyword::functions::FunctionKeyword;
use crate::parser::literal::Attribute;
use crate::utils::display;

/// Variable name
#[derive(Debug, PartialEq, Eq, Default)]
pub enum VariableName {
    /// Unreachable, used only for `mem::take`
    //TODO: this is horrid
    #[default]
    Empty,
    /// Function keyword, like `sizeof` or `alignof`
    Keyword(FunctionKeyword),
    /// User defined name: any identifier
    UserDefined(String),
}

impl VariableName {
    /// Transform a [`VariableName`] into an [`Attribute`]
    ///
    /// # Panics
    ///
    /// If called on a [`FunctionKeyword`]
    pub fn into_attr(self) -> Attribute {
        match self {
            Self::UserDefined(name) => Attribute::User(name),
            Self::Keyword(_) => unreachable!("called on invalid attribute"),
            Self::Empty => unreachable!("never constructed"),
        }
    }
}

display!(
    VariableName,
    self,
    f,
    match self {
        Self::Empty => EMPTY.fmt(f),
        Self::UserDefined(val) => val.fmt(f),
        Self::Keyword(val) => val.fmt(f),
    }
);
