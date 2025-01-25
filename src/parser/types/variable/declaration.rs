//! Module implementation for variable declarations (LHS), i.e.,
//! that contain attributes.

use core::{fmt, mem};

use super::name::VariableName;
use super::traits::VariableConversion;
use super::{Variable, traits};
use crate::parser::keyword::attributes::{AttributeKeyword, UserDefinedTypes};
use crate::parser::modifiers::ast::can_push::CanPush;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::repr_option_vec;
use crate::parser::types::Ast;
use crate::parser::types::literal::{Attribute, repr_vec_attr};

/// Variable declarations
///
/// # Example
///
/// ```c
/// const * int * volatile x = 3, y = 2;
/// ```
#[derive(Debug)]
pub struct AttributeVariable {
    /// attributes of the variable
    pub attrs: Vec<Attribute>,
    /// name of the variable
    pub declarations: Vec<Option<Declaration>>,
}

impl AttributeVariable {
    /// Transforms a [`VariableName`] and an equal `=` sign into an
    /// [`AttributeVariable`].
    pub fn from_name_eq(varname: VariableName) -> Result<Self, &'static str> {
        match varname {
            VariableName::UserDefined(name) => Ok(Self {
                attrs: vec![],
                declarations: vec![Some(Declaration {
                    name,
                    value: Some(Ast::Empty),
                })],
            }),
            VariableName::Keyword(_) => Err("Can't assign to function keyword."),
            VariableName::Empty => panic!("never constructed"),
        }
    }

    /// Adds an attribute to the variable
    pub fn push_attr(&mut self, attr: Attribute) {
        if self.declarations.len() <= 1 {
            if let Some(Some(last)) = self.declarations.pop() {
                if last.value.is_none() {
                    self.attrs.push(Attribute::User(last.name));
                } else {
                    panic!("Trying to push attribute after variable initialisation expression.")
                }
            }
            self.attrs.push(attr);
        } else {
            panic!("tried to push attribute on multiple variables")
        }
    }

    /// Pushes a name into an [`AttributeVariable`]
    pub fn push_name(&mut self, name: String) -> Result<(), String> {
        if self.declarations.is_empty() {
            self.declarations.push(Some(Declaration::from(name)));
            Ok(())
        } else if self.declarations.len() == 1 {
            let last = self.declarations.last_mut().expect("len = 1");
            if let Some(decl) = last {
                if let Some(value) = &mut decl.value {
                    value.push_block_as_leaf(Ast::Variable(Variable::from(name)))
                } else {
                    self.attrs.push(Attribute::User(mem::take(&mut decl.name)));
                    decl.name = name;
                    Ok(())
                }
            } else {
                self.declarations.push(Some(Declaration::from(name)));
                Ok(())
            }
        } else {
            let last = self.declarations.last_mut().expect("len > 1");
            if last.is_none() {
                *last = Some(Declaration::from(name));
                Ok(())
            } else {
                Err(
                    "Successive literals in variable declaration. Found attribute after comma"
                        .to_owned(),
                )
            }
        }
    }
}

impl CanPush for AttributeVariable {
    fn can_push_leaf(&self) -> bool {
        self.declarations.last().is_some_and(|opt| {
            opt.as_ref()
                .is_some_and(|decl| decl.value.as_ref().is_some_and(Ast::can_push_leaf))
        })
    }
}

impl From<AttributeKeyword> for AttributeVariable {
    fn from(value: AttributeKeyword) -> Self {
        Self {
            attrs: vec![Attribute::Keyword(value)],
            declarations: vec![],
        }
    }
}

impl Push for AttributeVariable {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf(&ast, self, "attr var");
        self.declarations
            .last_mut()
            .ok_or("Found non empty declarations")
            .and_then(|last| last.as_mut().ok_or("Missing name for last declaration"))
            .and_then(|last| {
                last.value.as_mut().ok_or(
                    "Found successive literals in variable declaration. Did you forget an assign?",
                )
            })
            .map_err(str::to_owned)
            .and_then(|last| last.push_block_as_leaf(ast))
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_op(&op, self, "attr var");
        match self.declarations.last_mut() {
            Some(Some(last)) => {
                if let Some(value) = &mut last.value {
                    value.push_op(op)
                } else if op.is_star() {
                    if self.declarations.len() == 1 {
                        self.attrs.push(Attribute::User(
                            self.declarations
                                .pop()
                                .expect("is some")
                                .expect("is some")
                                .name,
                        ));
                        self.push_attr(Attribute::Indirection);
                        Ok(())
                    } else {
                        Err("Can't push * in empty declaration: missing `=`.".to_owned())
                    }
                } else if op.is_eq() {
                    last.value = Some(Ast::Empty);
                    Ok(())
                } else {
                    Err("Can't push operator in empty declaration: missing `=`.".to_owned())
                }
            }
            Some(None) => {
                Err("Can't push operator in empty declaration: missing variable name.".to_owned())
            }
            None if op.is_star() => {
                self.attrs.push(Attribute::Indirection);
                Ok(())
            }
            None => Err("Can't push operator to variable: missing declaration.".to_owned()),
        }
    }
}

impl VariableConversion for AttributeVariable {
    fn as_partial_typedef(&mut self) -> Option<(&UserDefinedTypes, Option<String>)> {
        if self.attrs.len() == 1
            && let Some(Attribute::Keyword(AttributeKeyword::UserDefinedTypes(user_type))) =
                self.attrs.last()
        {
            if self.declarations.is_empty() {
                Some((user_type, None))
            } else if self.declarations.len() == 1
                && let Some(Some(decl)) = self.declarations.last_mut()
                && decl.value.is_none()
            {
                Some((user_type, Some(mem::take(&mut decl.name))))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn has_eq(&self) -> bool {
        self.declarations
            .iter()
            .any(|opt| opt.as_ref().is_some_and(|decl| decl.value.is_some()))
    }

    fn into_attrs(self) -> Result<Vec<Attribute>, String> {
        let mut mutable = self;
        if mutable.declarations.len() == 1
            && let Some(Some(last)) = mutable.declarations.pop()
            && last.value.is_none()
        {
            mutable.attrs.push(Attribute::User(last.name));
        } else if !mutable.declarations.is_empty() {
            return Err(
                "Trying to convert declarations to attributes, but this is illegal.".to_owned(),
            );
        }
        Ok(mutable.attrs)
    }

    fn into_partial_typedef(mut self) -> Option<(UserDefinedTypes, Option<String>)> {
        if self.attrs.len() == 1 {
            if let Some(Attribute::Keyword(AttributeKeyword::UserDefinedTypes(user_type))) =
                self.attrs.pop()
            {
                if self.declarations.len() == 1
                    && let Some(Some(last)) = self.declarations.pop()
                    && last.value.is_none()
                {
                    Some((user_type, Some(last.name)))
                } else {
                    Some((user_type, None))
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn push_comma(&mut self) -> bool {
        self.declarations.push(None);
        true
    }
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for AttributeVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}:{})",
            repr_vec_attr(&self.attrs),
            repr_option_vec(&self.declarations),
        )
    }
}

impl traits::PureType for AttributeVariable {
    fn is_pure_type(&self) -> bool {
        self.declarations
            .last()
            .is_none_or(|opt| opt.as_ref().is_none_or(|decl| decl.value.is_none()))
    }

    fn take_pure_type(&mut self) -> Option<Vec<Attribute>> {
        self.is_pure_type().then(|| {
            if let Some(Some(Declaration { name, value })) = self.declarations.last_mut() {
                debug_assert!(value.is_none(), "");
                self.attrs.push(Attribute::User(mem::take(name)));
            }
            mem::take(&mut self.attrs)
        })
    }
}

/// Declaration of one variable
#[derive(Debug, Default)]
pub struct Declaration {
    /// Name of the variable.
    name: String,
    /// Expression to define the value of the variable.
    value: Option<Ast>,
}

impl From<String> for Declaration {
    fn from(name: String) -> Self {
        Self { name, value: None }
    }
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(value) => write!(f, "({} = {})", self.name, value),
            None => self.name.fmt(f),
        }
    }
}
