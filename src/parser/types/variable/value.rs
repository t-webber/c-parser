//! Module to contain the underlying value of a [`Variable`](super::Variable)

use core::{fmt, mem};

use super::after_keyword_err;
use super::declaration::{AttributeVariable, Declaration};
use super::name::VariableName;
use crate::parser::keyword::attributes::UserDefinedTypes;
use crate::parser::types::literal::Attribute;

/// Different variable cases
#[derive(Debug, PartialEq)]
pub enum VariableValue {
    /// A variable declaration, with attributes and/or expression
    AttributeVariable(AttributeVariable),
    /// A lone variable name, without attributes or value
    VariableName(VariableName),
}

impl VariableValue {
    /// Finds the leaf the most left possible, checks it is a variable and
    /// pushes it some attributes.
    ///
    /// See [`super::Ast::add_attribute_to_left_variable`] for more information.
    pub fn add_attribute_to_left_variable(
        &mut self,
        previous_attrs: Vec<Attribute>,
    ) -> Result<(), String> {
        match self {
            Self::AttributeVariable(AttributeVariable { attrs, .. }) => {
                let mut previous = previous_attrs;
                previous.reserve(attrs.len());
                let old_attrs = mem::replace(attrs, previous);
                attrs.extend(old_attrs);
            }
            Self::VariableName(VariableName::Keyword(keyword)) => {
                return Err(format!(
                    "Found attributes for function keyword {keyword}. Please use another variable name."
                ));
            }
            Self::VariableName(VariableName::Empty) => {
                *self = Self::AttributeVariable(AttributeVariable {
                    attrs: previous_attrs,
                    declarations: vec![],
                });
            }
            Self::VariableName(VariableName::UserDefined(name)) => {
                *self = Self::AttributeVariable(AttributeVariable {
                    attrs: previous_attrs,
                    declarations: vec![Some(Declaration::from(mem::take(name)))],
                });
            }
        }
        Ok(())
    }

    /// Merges a [`Variable`](super::Variable) with another
    /// [`Variable`](super::Variable) and returns the result.
    pub fn extend(&mut self, other: Self) -> Result<(), String> {
        match other {
            Self::VariableName(VariableName::UserDefined(name_o)) => match self {
                Self::AttributeVariable(var) => var.push_name(name_o),

                Self::VariableName(name_s) => {
                    let attrs = mem::take(name_s)
                        .into_attr()?
                        .map(|x| vec![x])
                        .unwrap_or_default();
                    *self = Self::AttributeVariable(AttributeVariable {
                        declarations: vec![Some(Declaration::from(name_o))],
                        attrs,
                    });
                    Ok(())
                }
            },
            Self::VariableName(VariableName::Empty) => Ok(()),
            Self::VariableName(VariableName::Keyword(keyword)) => Err(format!(
                "Invalid token. Expected user-defined variable, found keyword {keyword}"
            )),
            Self::AttributeVariable(mut decl) => {
                debug_assert!(
                    decl.declarations.is_empty(),
                    "Found declaration after declaration."
                );
                debug_assert!(
                    decl.attrs.len() == 1,
                    "Created declaration for non atomic token"
                );
                let attr = decl.attrs.pop().expect("len = 1");
                self.push_attr(attr)
            }
        }
    }

    /// Checks if a variable is in reality a type definition.
    ///
    /// `struct Name` is parsed as a variable attributes `struct` and `Name` and
    /// is waiting for the variable name. But if the next token is block, like
    /// in `struct Name {}`, it is meant as a control flow to define the type.
    pub fn get_partial_typedef(&mut self) -> Option<(&UserDefinedTypes, Option<String>)> {
        if let Self::AttributeVariable(var) = self {
            var.get_partial_typedef()
        } else {
            None
        }
    }

    /// Checks if a variable contains attributes
    pub const fn has_empty_attrs(&self) -> bool {
        match self {
            Self::AttributeVariable(AttributeVariable { attrs, .. }) => attrs.is_empty(),
            Self::VariableName(_) => false,
        }
    }

    /// Transforms a variable into a list of [`Attribute`]
    pub fn into_attrs(self) -> Result<Vec<Attribute>, String> {
        match self {
            Self::AttributeVariable(var) => var.into_attrs(),
            Self::VariableName(name) => name
                .into_attr()
                .map(|name_attr| name_attr.map_or_else(Vec::new, |attr| vec![attr])),
        }
    }

    /// Transforms a variable into a partial typedef
    pub fn into_partial_typedef(self) -> Option<(UserDefinedTypes, Option<String>)> {
        match self {
            Self::AttributeVariable(var) => var.into_partial_typedef(),
            Self::VariableName(_) => None,
        }
    }

    /// Returns the variable name if the variable is a user defined variable
    pub fn into_user_defined_name(self) -> Result<String, &'static str> {
        match self {
            Self::AttributeVariable(_) => Err("Expected variable name, found illegal attributes."),
            Self::VariableName(VariableName::Empty) => {
                panic!("Tried to use illegal variable empty constructor.")
            }
            Self::VariableName(VariableName::Keyword(_)) => {
                Err("Illegal type name: this is a protected keyword.")
            }
            Self::VariableName(VariableName::UserDefined(name)) => Ok(name),
        }
    }

    /// Checks if a variable is a user defined variable
    pub const fn is_user_defined(&self) -> bool {
        match self {
            Self::AttributeVariable(_) => true,
            Self::VariableName(name) => name.is_user_variable(),
        }
    }

    /// Adds an attribute to the variable
    pub fn push_attr(&mut self, attr: Attribute) -> Result<(), String> {
        match self {
            Self::AttributeVariable(var) => var.push_attr(attr),
            Self::VariableName(var) => match mem::take(var) {
                VariableName::Empty => {
                    *self = Self::AttributeVariable(AttributeVariable {
                        attrs: vec![attr],
                        declarations: vec![],
                    });
                }
                VariableName::Keyword(keyword) => {
                    return Err(after_keyword_err("attribute", attr, &keyword));
                }
                VariableName::UserDefined(name) => {
                    *self = Self::AttributeVariable(AttributeVariable {
                        attrs: vec![attr],
                        declarations: vec![Some(Declaration::from(name))],
                    });
                }
            },
        }
        Ok(())
    }

    /// Tries to push a comma into a variable
    pub fn push_comma(&mut self) -> bool {
        if let Self::AttributeVariable(decl) = self {
            decl.push_comma();
            true
        } else {
            false
        }
    }

    /// Tries transforming the [`Self`] into a user defined variable name.
    pub fn take_user_defined(&mut self) -> Option<String> {
        match self {
            Self::VariableName(VariableName::UserDefined(name)) => Some(mem::take(name)),
            Self::AttributeVariable(_) | Self::VariableName(_) => None,
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for VariableValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AttributeVariable(val) => val.fmt(f),
            Self::VariableName(val) => val.fmt(f),
        }
    }
}
