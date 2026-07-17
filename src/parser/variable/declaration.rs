//! Module implementation for variable declarations (LHS), i.e.,
//! that contain attributes.

use crate::Number;
use crate::errors::api::{ErrorLocation, Located};
use crate::parser::tree::api::Ast;
use crate::utils::{display, repr_option};

/// Declaration of one variable
#[derive(Debug)]
pub struct Declaration {
    /// Name of the variable.
    pub name: Located<String>,
    /// Expression to define the value of the variable.
    pub value: DeclarationValue,
}

impl Declaration {
    /// Returns name and value of the declaration.
    pub fn into_name_value(self) -> (Located<String>, DeclarationValue) {
        (self.name, self.value)
    }

    /// Returns the whole location of the declaration, from variable name to
    /// value.
    pub fn location(&self) -> ErrorLocation {
        match &self.value {
            DeclarationValue::Bitfield(size) => size.as_location(),
            DeclarationValue::None => self.name.as_location(),
            DeclarationValue::Value(ast) => ast.location().into_extended(self.name.as_location()),
        }
    }
}

impl From<Located<String>> for Declaration {
    fn from(name: Located<String>) -> Self {
        Self { name, value: DeclarationValue::None }
    }
}

display!(
    Declaration,
    self,
    f,
    match &self.value {
        DeclarationValue::Value(value) => write!(f, "({} = {})", self.name, value),
        DeclarationValue::None => self.name.fmt(f),
        DeclarationValue::Bitfield(size) =>
            write!(f, "({}:{})", self.name, repr_option(size.as_value())),
    }
);

/// Value of the declaration
///
/// This is meant to represent anything following the variable name that is only
/// associated with this variable declaration, and not to other variables
/// declared with the same statement.
#[derive(Debug, Default)]
pub enum DeclarationValue {
    /// A `:` sign was found after the name, meaning a bitfield specifier.
    Bitfield(Located<Option<Number>>),
    /// No value yet for this declaration, waiting for either `=` or `:`.
    #[default]
    None,
    /// An `=` sign was found, and the value after it is store here.
    Value(Ast),
}

impl DeclarationValue {
    /// Returns a mutable reference to the declaration value, if it exists.
    pub const fn as_mut(&mut self) -> Option<&mut Ast> {
        if let Self::Value(ast) = self {
            Some(ast)
        } else {
            None
        }
    }

    /// Returns a reference to the declaration value, if it exists.
    pub const fn as_ref(&self) -> Option<&Ast> {
        if let Self::Value(ast) = self {
            Some(ast)
        } else {
            None
        }
    }

    /// Returns `true` iff the declaration doesn't yet have a value.
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}
