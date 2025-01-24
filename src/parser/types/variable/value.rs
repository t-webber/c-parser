//! Module to contain the underlying value of a [`Variable`](super::Variable)

use core::{fmt, mem};

use super::after_keyword_err;
use super::declaration::{AttributeVariable, Declaration};
use super::name::VariableName;
use super::traits::{PureType, VariableConversion};
use crate::parser::keyword::attributes::UserDefinedTypes;
use crate::parser::modifiers::ast::can_push::{CanPush, PushAttribute};
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
            Self::VariableName(VariableName::Keyword(keyword)) => Err(format!(
                "Invalid token. Expected user-defined variable, found keyword {keyword}"
            )),
            Self::AttributeVariable(mut var) => {
                debug_assert!(
                    var.declarations.is_empty(),
                    "Found declaration after declaration."
                );
                debug_assert!(
                    var.attrs.len() == 1,
                    "Created declaration for non atomic token"
                );
                let attr = var.attrs.pop().expect("len = 1");
                self.push_attr(attr)
            }
            Self::VariableName(VariableName::Empty) => panic!("never constructed"),
        }
    }

    /// Checks if a variable contains attributes
    ///
    /// This is used to determine whether a LHS is an expression or a
    /// declaration. It is clear that if the variable has attributes, it is a
    /// declaration. Reciprocally, if the variable has no attributes, there is
    /// no type so it is an expression.
    ///
    /// If the variable is a variable name, no type is found, and thus this
    /// method returns `true`.
    pub const fn has_empty_attrs(&self) -> bool {
        match self {
            Self::AttributeVariable(AttributeVariable { attrs, .. }) => attrs.is_empty(),
            Self::VariableName(_) => true,
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
                VariableName::Keyword(keyword) => {
                    return Err(after_keyword_err("attribute", attr, &keyword));
                }
                VariableName::UserDefined(name) => {
                    *self = Self::AttributeVariable(AttributeVariable {
                        attrs: vec![attr],
                        declarations: vec![Some(Declaration::from(name))],
                    });
                }
                VariableName::Empty => panic!("never constructed"),
            },
        }
        Ok(())
    }

    /// Tries transforming the [`Self`] into a user defined variable name.
    pub fn take_user_defined(&mut self) -> Option<String> {
        match self {
            Self::VariableName(VariableName::UserDefined(name)) => Some(mem::take(name)),
            Self::AttributeVariable(_) | Self::VariableName(_) => None,
        }
    }
}

impl CanPush for VariableValue {
    fn can_push_leaf(&self) -> bool {
        match self {
            Self::AttributeVariable(var) => var.can_push_leaf(),
            Self::VariableName(_) => false,
        }
    }
}

impl PureType for VariableValue {
    fn is_pure_type(&self) -> bool {
        match self {
            Self::AttributeVariable(var) => var.is_pure_type(),
            Self::VariableName(VariableName::UserDefined(_)) => true,
            Self::VariableName(VariableName::Keyword(_)) => false,
            Self::VariableName(VariableName::Empty) => panic!("never constructed"),
        }
    }

    fn take_pure_type(&mut self) -> Option<Vec<Attribute>> {
        match self {
            Self::AttributeVariable(var) => var.take_pure_type(),
            Self::VariableName(VariableName::UserDefined(user_defined)) => {
                Some(vec![Attribute::User(mem::take(user_defined))])
            }
            Self::VariableName(VariableName::Empty | VariableName::Keyword(_)) => None,
        }
    }
}

impl PushAttribute for VariableValue {
    fn add_attribute_to_left_variable(
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
            Self::VariableName(VariableName::UserDefined(name)) => {
                *self = Self::AttributeVariable(AttributeVariable {
                    attrs: previous_attrs,
                    declarations: vec![Some(Declaration::from(mem::take(name)))],
                });
            }
            Self::VariableName(VariableName::Empty) => {
                panic!("never constructed");
                //     *self = Self::AttributeVariable(AttributeVariable {
                //         attrs: previous_attrs,
                //         declarations: vec![],
                //     });
                // }
            }
        }
        Ok(())
    }
}

impl VariableConversion for VariableValue {
    fn get_partial_typedef(&mut self) -> Option<(&UserDefinedTypes, Option<String>)> {
        match self {
            Self::AttributeVariable(var) => var.get_partial_typedef(),
            Self::VariableName(_) => None,
        }
    }

    fn has_eq(&self) -> bool {
        match self {
            Self::AttributeVariable(var) => var.has_eq(),
            Self::VariableName(_) => false,
        }
    }

    fn into_attrs(self) -> Result<Vec<Attribute>, String> {
        match self {
            Self::AttributeVariable(var) => var.into_attrs(),
            Self::VariableName(name) => name
                .into_attr()
                .map(|name_attr| name_attr.map_or_else(Vec::new, |attr| vec![attr])),
        }
    }

    fn into_partial_typedef(self) -> Option<(UserDefinedTypes, Option<String>)> {
        match self {
            Self::AttributeVariable(var) => var.into_partial_typedef(),
            Self::VariableName(_) => None,
        }
    }

    fn push_comma(&mut self) -> bool {
        match self {
            Self::AttributeVariable(var) => var.push_comma(),
            Self::VariableName(_) => false,
        }
    }
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for VariableValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AttributeVariable(val) => val.fmt(f),
            Self::VariableName(val) => val.fmt(f),
        }
    }
}
