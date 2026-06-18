//! Implementation of variables, i.e. identifiers.
//!
//! Note that labels (as in `goto: label`) are considered variables before being
//! pushed to the control flow.
//!
//! Else, variables can either be declarations (if attributes are applied to the
//! variable) or names (else). In the RHS, variables must be names.

#![expect(clippy::inline_modules, reason = "clearer api")]
pub mod api {
    //! Api module to choose what functions to export.

    #![allow(clippy::pub_use, reason = "expose simple API")]

    pub use super::Variable;
    pub use super::declaration::{AttributeVariable, Declaration, DeclarationValue};
    pub use super::traits::{PureType, VariableConversion};
    pub use super::value::VariableValue;
}

mod declaration;
mod name;
mod traits;
mod value;

use core::fmt;
use core::mem::take;

use declaration::AttributeVariable;
use name::VariableName;
use traits::{PureType, VariableConversion};
use value::VariableValue;

use super::keyword::attributes::{AttributeKeyword, UserDefinedTypes};
use super::keyword::functions::FunctionKeyword;
use super::literal::Attribute;
use super::modifiers::push::Push as _;
use super::tree::api::{Ast, CanPush, PushAttribute};
use crate::errors::api::Located;
use crate::parser::keyword::control_flow::types::colon_ast::ColonAstCtrl;
use crate::parser::modifiers::functions::{CanMakeFnRes, MakeFunction};
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
    /// Returns the variable as an attribute value if it is not a lone variable
    /// name.
    pub const fn as_attribute_variable_mut(&mut self) -> Option<&mut AttributeVariable> {
        if let VariableValue::AttributeVariable(attribute_variable) = &mut self.value {
            Some(attribute_variable)
        } else {
            None
        }
    }

    /// Merges a [`Variable`] with another [`Variable`] and returns the result.
    pub fn extend(&mut self, other: Self) -> Result<(), String> {
        if self.full {
            Err("Can't extend full variable".to_owned())
        } else {
            let other_full = other.full;
            self.value.extend(other)?;
            if other_full {
                self.full = true;
            }
            Ok(())
        }
    }

    /// Makes the variable full
    pub const fn fill(&mut self) {
        self.full = true;
    }

    /// Checks if the variable contains attributes
    pub const fn has_empty_attrs(&self) -> bool {
        self.value.has_empty_attrs()
    }

    /// Takes the attributes from inside self it is a type;
    pub fn into_type(self) -> Option<Vec<Attribute>> {
        match self.value {
            VariableValue::AttributeVariable(attr) => attr.into_type(),
            VariableValue::VariableName(..) => None,
        }
    }

    /// Returns the variable name if the variable is a user defined variable
    pub fn into_user_defined_name(self) -> Result<String, &'static str> {
        self.value.into_user_defined_name()
    }

    /// Returns the value of the variable.
    pub fn into_value(self) -> VariableValue {
        self.value
    }

    /// Checks if the variable is a user defined variable
    pub const fn is_declaration(&self) -> bool {
        matches!(self.value, VariableValue::AttributeVariable(_))
    }

    /// Checks if the variable is full
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

    /// Pushes an ast as leaf in the current variable.
    pub fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf(&ast, self, "var");
        if self.full {
            Err("Can't push ast to full variable".to_owned())
        } else if let Ast::Variable(var) = ast {
            self.extend(var)
        } else {
            match &mut self.value {
                VariableValue::AttributeVariable(decl) => decl.push_block_as_leaf(ast),
                VariableValue::VariableName(_, name) => {
                    unreachable!("tried to push block {ast} on non-declaration variable {name}")
                }
            }
        }
    }

    /// Pushes a colon `:` into a variable node.
    pub fn push_colon(&mut self) -> Result<Option<Ast>, String> {
        match &mut self.value {
            VariableValue::VariableName(_, VariableName::UserDefined(label)) =>
                Ok(Some(ColonAstCtrl::from_label_with_colon(take(label)))),
            VariableValue::VariableName(_, VariableName::Keyword(kwd)) =>
        Err(
            format!("found `:` after keyword {kwd}: colon is only valid after user-defined label")
        ),
            VariableValue::AttributeVariable(_) if self.full => Err( "Colon unexpected in this context: neither variable declaration not ternary operator."
                .into(),
        ),
            VariableValue::AttributeVariable(attr) => attr.push_colon().map(|()| None),
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

    /// Takes the value of `self` and puts a placeholder in its place.
    pub fn take(&mut self) -> Self {
        Self { full: self.full, value: self.value.take() }
    }

    /// Tries transforming the [`Self`] into a user defined variable name.
    pub fn take_user_defined(&mut self) -> Option<String> {
        self.value.take_user_defined()
    }
}

impl MakeFunction for Variable {
    fn can_make_function(&self) -> CanMakeFnRes {
        if self.full {
            CanMakeFnRes::None
        } else {
            self.value.can_make_function()
        }
    }

    fn make_function(&mut self, depth: u32, arguments: Vec<Ast>) {
        self.value.make_function(depth, arguments);
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

impl From<Located<FunctionKeyword>> for Variable {
    fn from(value: Located<FunctionKeyword>) -> Self {
        let (inner, loc) = value.into_inner();
        Self {
            full: false,
            value: VariableValue::VariableName(loc, VariableName::Keyword(inner)),
        }
    }
}

impl From<Located<String>> for Variable {
    fn from(value: Located<String>) -> Self {
        let (inner, loc) = value.into_inner();
        Self {
            full: false,
            value: VariableValue::VariableName(loc, VariableName::UserDefined(inner)),
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
