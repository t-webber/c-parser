//! Implementation of variables, i.e. identifiers.
//!
//! Note that labels (as in `goto: label`) are considered variables before being
//! pushed to the control flow.
//!
//! Else, variables can either be declarations (if attributes are applied to the
//! variable) or names (else). In the RHS, variables must be names.

pub mod api {
    //! Api module to choose what functions to export.

    #![allow(clippy::pub_use, reason = "expose simple API")]

    pub use super::traits::{PureType, VariableConversion};
}
mod declaration;
mod name;
mod traits;
mod value;

use core::{fmt, mem};

use declaration::AttributeVariable;
use name::VariableName;
use traits::{PureType, VariableConversion};
use value::VariableValue;

use super::keyword::attributes::{AttributeKeyword, UserDefinedTypes};
use super::keyword::functions::FunctionKeyword;
use super::literal::Attribute;
use super::modifiers::push::Push;
use super::operators::api::OperatorConversions;
use super::tree::api::{Ast, CanPush, PushAttribute};
use crate::utils::display;

/// Different variable cases
#[derive(Debug)]
pub struct Variable {
    /// Indicated if the variable is full
    full: bool,
    /// Contains the actual value of the variable
    value: VariableValue,
}

impl Variable {
    /// Merges a [`Variable`] with another [`Variable`] and returns the result.
    pub fn extend(&mut self, other: Self) -> Result<(), String> {
        if self.full {
            Err("Can't extend full variable".to_owned())
        } else {
            self.value.extend(other.value)?;
            if other.full {
                self.full = true;
            }
            Ok(())
        }
    }

    /// Makes a variable full
    pub const fn fill(&mut self) {
        self.full = true;
    }

    /// Checks if a variable contains attributes
    pub const fn has_empty_attrs(&self) -> bool {
        self.value.has_empty_attrs()
    }

    /// Returns the variable name if the variable is a user defined variable
    pub fn into_user_defined_name(self) -> Result<String, &'static str> {
        self.value.into_user_defined_name()
    }

    /// Checks if a variable is a user defined variable
    pub const fn is_declaration(&self) -> bool {
        matches!(self.value, VariableValue::AttributeVariable(_))
    }

    /// Checks if a variable is full
    pub const fn is_full(&self) -> bool {
        self.full
    }

    /// Adds an attribute to the variable
    fn push_attr(&mut self, attr: Attribute) -> Result<(), String> {
        if self.full {
            Err("Can't push attribute to full variable".to_owned())
        } else {
            self.value.push_attr(attr)
        }
    }

    /// Adds a `*` indirection attribute to the variable
    pub fn push_indirection(&mut self) -> Result<(), String> {
        self.push_attr(Attribute::Indirection)
    }

    /// Adds a `*` indirection attribute to the variable
    pub fn push_keyword(&mut self, keyword: AttributeKeyword) -> Result<(), String> {
        self.push_attr(Attribute::Keyword(keyword))
    }

    /// Tries transforming the [`Self`] into a user defined variable name.
    pub fn take_user_defined(&mut self) -> Option<String> {
        self.value.take_user_defined()
    }
}

impl CanPush for Variable {
    fn can_push_leaf(&self) -> bool {
        self.value.can_push_leaf()
    }
}

impl From<AttributeKeyword> for Variable {
    fn from(value: AttributeKeyword) -> Self {
        Self {
            full: false,
            value: VariableValue::AttributeVariable(AttributeVariable::from(value)),
        }
    }
}

impl From<FunctionKeyword> for Variable {
    fn from(value: FunctionKeyword) -> Self {
        Self { full: true, value: VariableValue::VariableName(VariableName::Keyword(value)) }
    }
}

impl From<String> for Variable {
    fn from(value: String) -> Self {
        Self {
            full: false,
            value: VariableValue::VariableName(VariableName::UserDefined(value)),
        }
    }
}

impl PureType for Variable {
    fn is_pure_type(&self) -> bool {
        self.value.is_pure_type()
    }

    fn take_pure_type(&mut self) -> Option<Vec<Attribute>> {
        self.value.take_pure_type()
    }
}

impl Push for Variable {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf(&ast, self, "var");
        if self.full {
            Err("Can't push ast to full variable".to_owned())
        } else if let Ast::Variable(var) = ast {
            self.extend(var)
        } else {
            match &mut self.value {
                VariableValue::AttributeVariable(decl) => decl.push_block_as_leaf(ast),
                VariableValue::VariableName(name) => {
                    panic!("tried to push block {ast} on non-declaration variable {name}")
                }
            }
        }
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_op(&op, self, "var");
        match &mut self.value {
            VariableValue::AttributeVariable(var) => var.push_op(op),
            VariableValue::VariableName(name) if op.is_eq() => {
                self.value = VariableValue::AttributeVariable(AttributeVariable::from_name_eq(
                    mem::take(name),
                )?);
                Ok(())
            }
            VariableValue::VariableName(_) =>
                Err("Can't push operator in non-declaration variable".to_owned()),
        }
    }
}

impl PushAttribute for Variable {
    fn add_attribute_to_left_variable(
        &mut self,
        previous_attrs: Vec<Attribute>,
    ) -> Result<(), String> {
        if self.full {
            Err("Can't push attributes to full variable".to_owned())
        } else {
            self.value.add_attribute_to_left_variable(previous_attrs)
        }
    }
}

impl VariableConversion for Variable {
    fn as_partial_typedef(&mut self) -> Option<(&UserDefinedTypes, Option<String>)> {
        if self.full {
            None
        } else {
            self.value.as_partial_typedef()
        }
    }

    fn has_eq(&self) -> bool {
        self.value.has_eq()
    }

    fn into_attrs(self) -> Result<Vec<Attribute>, String> {
        self.value.into_attrs()
    }

    fn into_partial_typedef(self) -> Option<(UserDefinedTypes, Option<String>)> {
        if self.full {
            None
        } else {
            self.value.into_partial_typedef()
        }
    }

    fn push_comma(&mut self) -> bool {
        if self.full {
            false
        } else {
            self.value.push_comma()
        }
    }
}

display!(
    Variable,
    self,
    f,
    // write!(f, "${}{}$", self.value, repr_fullness(self.full))
    self.value.fmt(f)
);

/// Makes an error for values found after a [`FunctionKeyword`].
fn after_keyword_err<T: fmt::Display>(name: &str, value: T, keyword: &FunctionKeyword) -> String {
    format!("Found {name} {value} after function keyword {keyword}, but this is not allowed.")
}
