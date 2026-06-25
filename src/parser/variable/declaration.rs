//! Module implementation for variable declarations (LHS), i.e.,
//! that contain attributes.

use core::mem::take;
use core::{fmt, mem};

use super::traits::VariableConversion;
use super::{Variable, traits};
use crate::errors::api::{ErrorLocation, Located};
use crate::lexer::api::StringId;
use crate::parser::keyword::attributes::{AttributeKeyword, UserDefinedTypes};
use crate::parser::literal::{Attribute, Literal};
use crate::parser::modifiers::functions::{CanMakeFnRes, MakeFunction};
use crate::parser::modifiers::push::Push;
use crate::parser::operators::api::OperatorConversions;
use crate::parser::tree::api::{Ast, CanPush};
use crate::utils::{StringResolver, display, repr_option, repr_option_vec, repr_vec};
use crate::{BracedBlock, EMPTY, Number};

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
    /// Displays the attribute variable declarations in an human-readable
    /// manner.
    pub fn display(&self, resolver: &StringResolver<BracedBlock>) -> String {
        format!(
            "({}:{})",
            resolver.display_type(self.attrs.as_slice(), |attr| attr.as_value()),
            self.declarations
                .iter()
                .map(|declaration| declaration.as_ref().map_or_else(
                    || EMPTY.to_owned(),
                    |decl| match &decl.value {
                        DeclarationValue::Bitfield(size) => format!(
                            "{}:{}",
                            decl.name,
                            size.as_value()
                                .as_ref()
                                .map_or_else(|| EMPTY.to_owned(), ToString::to_string)
                        ),
                        DeclarationValue::None => format!("{}", decl.name),
                        DeclarationValue::Value(ast) =>
                            format!("{}:{}", decl.name, resolver.display_node(ast)),
                    }
                ))
                .collect::<Vec<_>>()
                .join(",")
        )
    }

    /// Returns the unique variable in this attribute declaration, if there is
    /// one and only one.
    pub fn into_single_variable(mut self) -> Option<(Located<StringId>, Vec<Located<Attribute>>)> {
        if let Some(Some(decl)) = self.declarations.pop()
            && self.declarations.is_empty()
            && let (name, value) = decl.into_name_value()
            && value.is_none()
        {
            Some((name, self.attrs))
        } else {
            None
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
    pub fn push_name(&mut self, name: Located<StringId>) -> Result<(), String> {
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

impl MakeFunction for AttributeVariable {
    fn can_make_function(&self) -> CanMakeFnRes {
        match self.declarations.last()? {
            Some(declaration) => declaration.value.as_ref()?.can_make_function(),
            None => CanMakeFnRes::None,
        }
    }

    fn make_function(&mut self, depth: u32, arguments: Vec<Ast>, parens_location: ErrorLocation) {
        match self
            .declarations
            .last_mut()
            .expect("checked with can_make_function")
        {
            Some(declaration) => declaration
                .value
                .as_mut()
                .expect("checked with can_make_function")
                .make_function(depth, arguments, parens_location),
            None => unreachable!(),
        }
    }
}

impl CanPush for AttributeVariable {
    fn can_push_leaf(&self) -> bool {
        self.declarations.last().is_some_and(|opt| {
            opt.as_ref().is_some_and(|decl| match &decl.value {
                DeclarationValue::Bitfield(size) => size.as_value().is_none(),
                DeclarationValue::None => false,
                DeclarationValue::Value(ast) => ast.can_push_leaf(),
            })
        })
    }
}

impl From<Located<AttributeKeyword>> for AttributeVariable {
    fn from(value: Located<AttributeKeyword>) -> Self {
        Self { attrs: vec![value.transfer(Attribute::Keyword)], declarations: vec![] }
    }
}

impl Push for AttributeVariable {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf(&ast, self, "attr var");
        match &mut self
            .declarations
            .last_mut()
            .ok_or("Found non empty declarations")?
            .as_mut()
            .ok_or("Missing name for last declaration")?
            .value
        {
            DeclarationValue::Value(val) => val.push_block_as_leaf(ast),
            DeclarationValue::None =>
                Err("Found successive literals in variable declaration, did you forget a `=`?"
                    .to_owned()),
            DeclarationValue::Bitfield(nb) if nb.as_value().is_some() =>
                Err("Found literal after bitfield, did you forget a `;`?".to_owned()),
            DeclarationValue::Bitfield(size) => {
                if let Ast::Leaf(lit) = ast
                    && let (val, loc) = lit.into_inner()
                    && let Literal::Number(nb) = val
                {
                    *size = loc.wrap(Some(nb));
                    Ok(())
                } else {
                    Err("Expected bitfield size, but `:` is followed by a non-number token"
                        .to_owned())
                }
            }
        }
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display,
    {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_op(&op, self, "attr var");
        match self.declarations.last_mut() {
            Some(Some(last)) => match &mut last.value {
                DeclarationValue::Value(ast) => ast.push_op(op),
                DeclarationValue::Bitfield(_) =>
                    Err("Found operator after bitfield specifier but this is not allowed".into()),
                DeclarationValue::None =>
                    if let Some(loc) = op.as_star() {
                        if self.declarations.len() == 1 {
                            self.attrs.push(
                                self.declarations
                                    .pop()
                                    .expect("is some")
                                    .expect("is some")
                                    .name
                                    .transfer(Attribute::User),
                            );
                            self.push_attr(loc.wrap(Attribute::Indirection))?;
                            Ok(())
                        } else {
                            Err("Can't push * in empty declaration: missing `=`.".to_owned())
                        }
                    } else if op.is_eq() {
                        last.value = DeclarationValue::Value(Ast::Empty);
                        Ok(())
                    } else {
                        Err("Can't push operator in empty declaration: missing `=`.".to_owned())
                    },
            },
            Some(None) =>
                Err("Can't push operator in empty declaration: missing variable name.".to_owned()),
            None =>
                if let Some(loc) = op.as_star() {
                    self.attrs.push(loc.wrap(Attribute::Indirection));
                    Ok(())
                } else {
                    Err("Can't push operator to variable: missing declaration.".to_owned())
                },
        }
    }
}

impl VariableConversion for AttributeVariable {
    fn as_partial_typedef(
        &mut self,
    ) -> Option<(Located<UserDefinedTypes>, Option<Located<StringId>>)> {
        if self.attrs.len() == 1
            && let Some(last) = self.attrs.last()
            && let Attribute::Keyword(AttributeKeyword::UserDefinedTypes(user_type)) =
                last.as_value()
        {
            if self.declarations.is_empty() {
                Some((last.as_ref().transfer(|_| *user_type), None))
            } else if self.declarations.len() == 1
                && let Some(Some(decl)) = self.declarations.last_mut()
                && decl.value.is_none()
            {
                Some((last.as_ref().transfer(|_| *user_type), Some(mem::take(&mut decl.name))))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn into_attrs(self) -> Result<Vec<Located<Attribute>>, String> {
        let mut mutable = self;
        if mutable.declarations.len() == 1
            && let Some(Some(last)) = mutable.declarations.pop()
            && last.value.is_none()
        {
            mutable.attrs.push(last.name.transfer(Attribute::User));
        } else if !mutable.declarations.is_empty() {
            return Err(
                "Trying to convert declarations to attributes, but this is illegal.".to_owned()
            );
        }
        Ok(mutable.attrs)
    }

    fn push_comma(&mut self) -> bool {
        self.declarations.push(None);
        true
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

impl traits::PureType for AttributeVariable {
    fn is_pure_type(&self) -> bool {
        self.declarations
            .last()
            .is_none_or(|opt| opt.as_ref().is_none_or(|decl| decl.value.is_none()))
    }

    fn take_pure_type(&mut self) -> Option<Vec<Located<Attribute>>> {
        self.is_pure_type().then(|| {
            if let Some(Some(Declaration { name, value })) = self.declarations.last_mut() {
                debug_assert!(value.is_none(), "checked with is_pure_type");
                self.attrs.push(take(name).transfer(Attribute::User));
            }
            take(&mut self.attrs)
        })
    }
}

/// Declaration of one variable
#[derive(Debug)]
pub struct Declaration {
    /// Name of the variable.
    name: Located<StringId>,
    /// Expression to define the value of the variable.
    value: DeclarationValue,
}

impl Declaration {
    /// Returns name and value of the declaration.
    pub fn into_name_value(self) -> (Located<StringId>, DeclarationValue) {
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

impl From<Located<StringId>> for Declaration {
    fn from(name: Located<StringId>) -> Self {
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
    const fn as_mut(&mut self) -> Option<&mut Ast> {
        if let Self::Value(ast) = self {
            Some(ast)
        } else {
            None
        }
    }

    /// Returns a reference to the declaration value, if it exists.
    const fn as_ref(&self) -> Option<&Ast> {
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
