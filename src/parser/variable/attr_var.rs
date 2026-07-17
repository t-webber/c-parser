//! Module implementation for variable declarations (LHS), i.e.,
//! that contain attributes.

use core::mem::take;

use super::Variable;
use crate::errors::api::{ErrorLocation, Located};
use crate::parser::api::{Declaration, DeclarationValue};
use crate::parser::keyword::attributes::AttributeKeyword;
use crate::parser::literal::Attribute;
use crate::parser::modifiers::push::Push as _;
use crate::parser::tree::api::Ast;
use crate::utils::{display, repr_option_vec, repr_vec};

/// Variable declarations
///
/// # Example
///
/// ```c
/// const * int * volatile x = 3, y = 2;
/// ```
#[derive(Debug, Default)]
pub struct AttributeVariable {
    /// attributes of the variable
    pub attrs: Vec<Located<Attribute>>,
    /// name of the variable
    pub declarations: Vec<Option<Declaration>>,
}

impl AttributeVariable {
    /// Returns the unique variable in this attribute declaration, if there is
    /// one and only one.
    pub fn into_single_variable(mut self) -> (Option<Located<String>>, Vec<Located<Attribute>>) {
        if let Some(Some(decl)) = self.declarations.pop()
            && self.declarations.is_empty()
            && let (name, value) = decl.into_name_value()
            && value.is_none()
        {
            (Some(name), self.attrs)
        } else {
            (None, self.attrs)
        }
    }

    /// Takes the attributes from inside self it is a type;
    pub fn into_type(self) -> Option<Vec<Located<Attribute>>> {
        self.declarations.is_empty().then_some(self.attrs)
    }

    /// Builds and returns the location of the attribute variable.
    pub fn location(&self) -> ErrorLocation {
        let start = self.attrs.first().map_or_else(
            || {
                self.declarations
                    .iter()
                    .find_map(|decl| decl.as_ref())
                    .map(Declaration::location)
            },
            |attr| Some(attr.as_location()),
        );
        let end = self
            .declarations
            .iter()
            .rev()
            .find_map(|decl| decl.as_ref())
            .map_or_else(
                || self.attrs.last().map(Located::as_location),
                |decl| Some(decl.location()),
            );
        match (start, end) {
            (None, None) => unreachable!("variable declaration is created with at least 1 varname"),
            (None, Some(loc)) | (Some(loc), None) => loc,
            (Some(first), Some(last)) => first.into_extended(last),
        }
    }

    /// Adds an attribute to the variable
    pub fn push_attr(&mut self, attr: Located<Attribute>) -> Result<(), String> {
        if self.declarations.len() <= 1 {
            if let Some(Some(last)) = self.declarations.pop() {
                if last.value.is_none() {
                    self.attrs.push(last.name.transfer(Attribute::User));
                } else {
                    return Err("Unexpected attribute: not in variable type".to_owned());
                }
            }
            self.attrs.push(attr);
        } else {
            return Err("Isolated attribute: not in variable type".to_owned());
        }
        Ok(())
    }

    /// Pushes a colon `:` into a variable node.
    pub fn push_colon(&mut self, colon_location: ErrorLocation) -> Result<(), String> {
        match self
            .declarations
            .last_mut()
            .and_then(|opt| opt.as_mut())
            .map(|decl| &mut decl.value)
        {
            None => Err("Expected variable name, found `:`".into()),
            Some(value @ DeclarationValue::None) => {
                *value = DeclarationValue::Bitfield(colon_location.wrap(None));
                Ok(())
            }
            Some(DeclarationValue::Value(ast)) => ast.handle_colon(colon_location),
            Some(DeclarationValue::Bitfield(_)) =>
                Err("found 2 successive colons in struct declaration".into()),
        }
    }

    /// Pushes a name into an [`AttributeVariable`]
    pub fn push_name(&mut self, name: Located<String>) -> Result<(), String> {
        if self.declarations.is_empty() {
            self.declarations.push(Some(Declaration::from(name)));
            Ok(())
        } else {
            let last = self.declarations.last_mut().expect("len = 1");
            if let Some(decl) = last {
                match &mut decl.value {
                    DeclarationValue::None => {
                        self.attrs
                            .push(take(&mut decl.name).transfer(Attribute::User));
                        decl.name = name;
                        Ok(())
                    }
                    DeclarationValue::Value(ast) =>
                        ast.push_block_as_leaf(Ast::Variable(Variable::from(name))),
                    DeclarationValue::Bitfield(_) =>
                        Err("Found unexpected identifier after bitfield specifier".into()),
                }
            } else {
                *last = Some(Declaration::from(name));
                Ok(())
            }
        }
    }
}

impl From<Located<AttributeKeyword>> for AttributeVariable {
    fn from(value: Located<AttributeKeyword>) -> Self {
        Self { attrs: vec![value.transfer(Attribute::Keyword)], declarations: vec![] }
    }
}

display!(
    AttributeVariable,
    self,
    f,
    write!(
        f,
        "({}:{})",
        repr_vec(&self.attrs, " "),
        repr_option_vec(&self.declarations, ", ")
    )
);
