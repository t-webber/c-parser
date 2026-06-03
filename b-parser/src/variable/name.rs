//! Module implementation for variable considered as names, i.e., that don't
//! contain attributes.

use core::mem;

use crate::keyword::functions::FunctionKeyword;
use crate::literal::Attribute;
use utils::display;

/// Variable name
#[derive(Debug, PartialEq, Eq)]
pub enum VariableName {
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
        }
    }

    /// Takes the value of `self` and puts a placeholder in its place.
    pub const fn take(&mut self) -> Self {
        mem::replace(self, Self::UserDefined(String::new()))
    }
}

display!(
    VariableName,
    self,
    f,
    match self {
        Self::UserDefined(val) => val.fmt(f),
        Self::Keyword(val) => val.fmt(f),
    }
);
