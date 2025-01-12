//! Implementation of variables, i.e. identifiers.
//!
//! Note that labels (as in `goto: label`) are considered variables before being
//! pushed to the control flow.
//!
//! Else, variables can either be declarations (if attributes are applied to the
//! variable) or names (else). In the RHS, variables must be names.

mod declaration;
mod name;
mod value;

use core::fmt;

use declaration::AttributeVariable;
use name::VariableName;
use value::VariableValue;

use super::Ast;
use super::literal::Attribute;
use crate::parser::keyword::attributes::{AttributeKeyword, UserDefinedTypes};
use crate::parser::keyword::functions::FunctionKeyword;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;

/// Different variable cases
#[derive(Debug, PartialEq)]
pub struct Variable {
    /// Indicated if the variable is full
    full: bool,
    /// Contains the actual value of the variable
    value: VariableValue,
}

impl Variable {
    /// Finds the leaf the most left possible, checks it is a variable and
    /// pushes it some attributes.
    ///
    /// See [`Ast::add_attribute_to_left_variable`] for more information.
    pub fn add_attribute_to_left_variable(
        &mut self,
        previous_attrs: Vec<Attribute>,
    ) -> Result<(), String> {
        if self.full {
            Err("Can't push attributes to full variable".to_owned())
        } else {
            self.value.add_attribute_to_left_variable(previous_attrs)
        }
    }

    /// Merges a [`Variable`] with another [`Variable`] and returns the result.
    fn extend(&mut self, other: Self) -> Result<(), String> {
        if self.full {
            Err("Can't extend full variable".to_owned())
        } else {
            self.value.extend(other.value)?;
            Ok(())
        }
    }

    /// Makes a variable full
    pub fn fill(&mut self) {
        self.full = true;
    }

    /// Checks if a variable is in reality a type definition.
    ///
    /// `struct Name` is parsed as a variable attributes `struct` and `Name` and
    /// is waiting for the variable name. But if the next token is block, like
    /// in `struct Name {}`, it is meant as a control flow to define the type.
    pub fn get_typedef(&mut self) -> Option<(&UserDefinedTypes, String)> {
        if self.full {
            None
        } else {
            self.value.get_typedef()
        }
    }

    /// Checks if a variable contains attributes
    pub const fn has_empty_attrs(&self) -> bool {
        self.value.has_empty_attrs()
    }

    /// Transforms a variable into [`Attribute`]
    pub fn into_attrs(self) -> Result<Vec<Attribute>, String> {
        self.value.into_attrs()
    }

    /// Returns the variable name if the variable is a user defined variable
    pub fn into_user_defined_name(self) -> Result<String, &'static str> {
        self.value.into_user_defined_name()
    }

    /// Checks if a variable is a user defined variable
    pub const fn is_declaration(&self) -> bool {
        matches!(self.value, VariableValue::AttributeVariable(_))
    }

    /// Checks if a variable is a user defined variable
    pub const fn is_user_defined(&self) -> bool {
        self.value.is_user_defined()
    }

    /// Adds an attribute to the variable
    fn push_attr(&mut self, attr: Attribute) -> Result<(), String> {
        if self.full {
            Err("Can't push attribute to full variable".to_owned())
        } else {
            self.value.push_attr(attr)
        }
    }

    /// Tries to push a comma into a variable
    pub fn push_comma(&mut self) -> bool {
        if self.full {
            false
        } else {
            self.value.push_comma()
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
        Self {
            full: true,
            value: VariableValue::VariableName(VariableName::Keyword(value)),
        }
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

impl Push for Variable {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        println!("\tPushing {ast} as leaf in var {self}");
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
        if let VariableValue::AttributeVariable(decl) = &mut self.value {
            decl.push_op(op)
        } else {
            Err("Can't push op in variable without attributes".to_owned())
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "${}{}$", self.value, repr_fullness(self.full))
        self.value.fmt(f)
    }
}

/// Makes an error for values found after a [`FunctionKeyword`].
fn after_keyword_err<T: fmt::Display>(name: &str, value: T, keyword: &FunctionKeyword) -> String {
    format!("Found {name} {value} after function keyword {keyword}, but this is not allowed.")
}
