//! Module implementation for variable declarations (LHS), i.e.,
//! that contain attributes.

use core::{fmt, mem};

use super::Variable;
use crate::parser::keyword::attributes::{AttributeKeyword, UserDefinedTypes};
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::repr_option_vec;
use crate::parser::types::Ast;
use crate::parser::types::literal::Attribute;

/// Variable declarations
///
/// # Example
///
/// ```c
/// const * int * volatile x = 3, y = 2;
/// ```
#[derive(Debug, PartialEq)]
pub struct AttributeVariable {
    /// attributes of the variable
    pub attrs: Vec<Attribute>,
    /// name of the variable
    pub declarations: Vec<Option<Declaration>>,
}

impl AttributeVariable {
    /// Checks if a variable is in reality a type definition.
    ///
    /// `struct Name` is parsed as a variable attributes `struct` and `Name` and
    /// is waiting for the variable name. But if the next token is block, like
    /// in `struct Name {}`, it is meant as a control flow to define the type.
    pub fn get_typedef(&mut self) -> Option<(&UserDefinedTypes, String)> {
        if self.attrs.len() == 1
            && let Some(Attribute::Keyword(AttributeKeyword::UserDefinedTypes(user_type))) =
                self.attrs.last()
            && self.declarations.len() == 1
            && let Some(Some(decl)) = self.declarations.last_mut()
        {
            Some((user_type, mem::take(&mut decl.name)))
        } else {
            None
        }
    }

    /// Transforms a variable into a list of [`Attribute`]
    pub fn into_attrs(self) -> Result<Vec<Attribute>, String> {
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

    /// Pushes a comma into an [`AttributeVariable`]
    pub fn push_comma(&mut self) {
        self.declarations.push(None);
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
        println!("\tPushing {ast} as leaf in decl {self}");
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
        self.declarations
            .last_mut()
            .ok_or("Can't push op in empty declarations: missing variable name.")
            .and_then(|last| last.as_mut().ok_or("Missing variable name."))
            .map_err(str::to_owned)
            .and_then(|last| last.push_op(op))
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for AttributeVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}:{})",
            self.attrs
                .iter()
                .map(Attribute::to_string)
                .collect::<Vec<_>>()
                .join(" "),
            repr_option_vec(&self.declarations),
        )
    }
}

/// Declaration of one variable
#[derive(Debug, PartialEq, Default)]
pub struct Declaration {
    /// Name of the variable.
    name: String,
    /// Expression to define the value of the variable.
    value: Option<Ast>,
}

impl Declaration {
    /// Tries to push an operator in a [`Declaration`]
    ///
    /// See [`Ast::push_op`] for more information.
    fn push_op<T: OperatorConversions + fmt::Display + Copy>(
        &mut self,
        op: T,
    ) -> Result<(), String> {
        if let Some(node) = &mut self.value {
            node.push_op(op)
        } else if op.is_eq() {
            self.value = Some(Ast::Empty);
            Ok(())
        } else {
            Err(format!("Expected assign, found illegal operator {op}."))
        }
    }
}

impl From<String> for Declaration {
    fn from(name: String) -> Self {
        Self { name, value: None }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(value) => write!(f, "({} = {})", self.name, value),
            None => self.name.fmt(f),
        }
    }
}
