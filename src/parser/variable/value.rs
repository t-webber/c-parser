//! Module to contain the underlying value of a [`Variable`]

use core::mem::{self, take};

use super::after_keyword_err;
use super::declaration::{AttributeVariable, Declaration};
use super::name::VariableName;
use super::traits::{PureType, VariableConversion};
use crate::errors::api::{ErrorLocation, Located};
use crate::parser::keyword::attributes::UserDefinedTypes;
use crate::parser::literal::Attribute;
use crate::parser::modifiers::functions::{CanMakeFnRes, MakeFunction};
use crate::parser::modifiers::push::Push as _;
use crate::parser::tree::Ast;
use crate::parser::tree::api::{CanPush, PushAttribute};
use crate::parser::variable::Variable;
use crate::utils::display;

/// Different variable cases
#[derive(Debug)]
pub enum VariableValue {
    /// A variable declaration, with attributes and/or expression
    AttributeVariable(AttributeVariable),
    /// A lone variable name, without attributes or value
    VariableName(ErrorLocation, VariableName),
}

impl VariableValue {
    /// Merges a [`Variable`] with another [`Variable`] and returns the result.
    pub fn extend(&mut self, other: Variable) -> Result<(), String> {
        match other.value {
            Self::VariableName(loc_o, VariableName::UserDefined(name_o)) => match self {
                Self::AttributeVariable(var) => var.push_name(loc_o.wrap(name_o)),

                Self::VariableName(loc_s, name_s) => {
                    let attr = take(name_s).into_attr();
                    *self = Self::AttributeVariable(AttributeVariable {
                        declarations: vec![Some(Declaration::from(loc_o.wrap(name_o)))],
                        attrs: vec![take(loc_s).wrap(attr)],
                    });
                    Ok(())
                }
            },
            Self::VariableName(loc_o, VariableName::Keyword(keyword)) => {
                match self {
                    Self::AttributeVariable(attr) =>
                        attr.push_block_as_leaf(Ast::Variable(Variable {
                            full: other.full,
                            value: Self::VariableName(loc_o, VariableName::Keyword(keyword)),
                        })),
                    Self::VariableName(_, variable_name) => Err(format!(
                        "Invalid token. Expected user-defined variable after name {variable_name}, found keyword {keyword}"
                    )), // TODO: check this
                }
            }

            Self::AttributeVariable(mut var) => {
                debug_assert!(var.declarations.is_empty(), "Found declaration after declaration.");
                debug_assert!(var.attrs.len() == 1, "Created declaration for non atomic token");
                let attr = var.attrs.pop().expect("len = 1");
                self.push_attr(attr)
            }
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
            Self::VariableName(..) => true,
        }
    }

    /// Returns the variable name if the variable is a user defined variable
    pub fn into_user_defined_name(self) -> Result<Located<String>, &'static str> {
        match self {
            Self::AttributeVariable(_) => Err("Expected variable name, found illegal attributes."),
            Self::VariableName(_, VariableName::Keyword(_)) =>
                Err("Illegal type name: this is a protected keyword."),
            Self::VariableName(loc, VariableName::UserDefined(name)) => Ok(loc.wrap(name)),
        }
    }

    /// Adds an attribute to the variable
    pub fn push_attr(&mut self, attr: Located<Attribute>) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::lgp!("Pushing attribute {attr} in {self}");
        match self {
            Self::AttributeVariable(var) => var.push_attr(attr)?,
            Self::VariableName(loc, var) => match var {
                VariableName::Keyword(keyword) =>
                    return Err(after_keyword_err("attribute", attr, keyword)),
                VariableName::UserDefined(name) => {
                    *self = Self::AttributeVariable(AttributeVariable {
                        attrs: vec![attr],
                        declarations: vec![Some(Declaration::from(loc.clone().wrap(take(name))))],
                    });
                }
            },
        }
        Ok(())
    }

    /// Takes the value of `self` and puts a placeholder in its place.
    pub fn take(&mut self) -> Self {
        mem::replace(self, Self::AttributeVariable(AttributeVariable::default()))
    }

    /// Tries transforming the [`Self`] into a user defined variable name.
    pub fn take_user_defined(&mut self) -> Option<Located<String>> {
        match self {
            Self::VariableName(loc, VariableName::UserDefined(name)) =>
                Some(take(loc).wrap(take(name))),
            Self::AttributeVariable(_) | Self::VariableName(..) => None,
        }
    }
}

impl MakeFunction for VariableValue {
    fn can_make_function(&self) -> CanMakeFnRes {
        match self {
            Self::AttributeVariable(var) => var.can_make_function(),
            Self::VariableName(..) => CanMakeFnRes::None,
        }
    }

    fn make_function(&mut self, depth: u32, arguments: Vec<Ast>) {
        match self {
            Self::AttributeVariable(var) => var.make_function(depth, arguments),
            Self::VariableName(..) => unreachable!(),
        }
    }
}

impl CanPush for VariableValue {
    fn can_push_leaf(&self) -> bool {
        match self {
            Self::AttributeVariable(var) => var.can_push_leaf(),
            Self::VariableName(..) => false,
        }
    }
}

impl PureType for VariableValue {
    fn is_pure_type(&self) -> bool {
        match self {
            Self::AttributeVariable(var) => var.is_pure_type(),
            Self::VariableName(_, VariableName::UserDefined(_)) => true,
            Self::VariableName(_, VariableName::Keyword(_)) => false,
        }
    }

    fn take_pure_type(&mut self) -> Option<Vec<Located<Attribute>>> {
        match self {
            Self::AttributeVariable(var) => var.take_pure_type(),
            Self::VariableName(loc, VariableName::UserDefined(user_defined)) =>
                Some(vec![loc.clone().wrap(Attribute::User(take(user_defined)))]),
            Self::VariableName(_, VariableName::Keyword(_)) => None,
        }
    }
}

impl PushAttribute for VariableValue {
    fn add_attribute_to_left_variable(
        &mut self,
        previous_attrs: Vec<Located<Attribute>>,
    ) -> Result<(), String> {
        match self {
            Self::AttributeVariable(AttributeVariable { attrs, .. }) => {
                let mut previous = previous_attrs;
                previous.reserve(attrs.len());
                let old_attrs = mem::replace(attrs, previous);
                attrs.extend(old_attrs);
            }
            Self::VariableName(_, VariableName::Keyword(keyword)) => {
                return Err(format!(
                    "Found attributes for function keyword {keyword}. Please use another variable name."
                ));
            }
            Self::VariableName(loc, VariableName::UserDefined(name)) => {
                *self = Self::AttributeVariable(AttributeVariable {
                    attrs: previous_attrs,
                    declarations: vec![Some(Declaration::from(loc.clone().wrap(take(name))))],
                });
            }
        }
        Ok(())
    }
}

impl VariableConversion for VariableValue {
    fn as_partial_typedef(
        &mut self,
    ) -> Option<(Located<UserDefinedTypes>, Option<Located<String>>)> {
        match self {
            Self::AttributeVariable(var) => var.as_partial_typedef(),
            Self::VariableName(..) => None,
        }
    }

    fn into_attrs(self) -> Result<Vec<Located<Attribute>>, String> {
        match self {
            Self::AttributeVariable(var) => var.into_attrs(),
            Self::VariableName(loc, name) => Ok(vec![loc.wrap(name.into_attr())]),
        }
    }

    fn push_comma(&mut self) -> bool {
        match self {
            Self::AttributeVariable(var) => var.push_comma(),
            Self::VariableName(..) => false,
        }
    }
}

display!(
    VariableValue,
    self,
    f,
    match self {
        Self::AttributeVariable(val) => val.fmt(f),
        Self::VariableName(_, val) => val.fmt(f),
    }
);
