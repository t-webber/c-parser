//! Module implementation for variable declarations (LHS), i.e.,
//! that contain attributes.

use core::{fmt, mem};

use super::name::VariableName;
use super::traits::VariableConversion;
use super::{Variable, traits};
use crate::display::repr_option_vec;
use crate::keyword::attributes::{AttributeKeyword, UserDefinedTypes};
use crate::literal::{Attribute, Literal, repr_vec_attr};
use crate::modifiers::functions::{CanMakeFnRes, MakeFunction};
use crate::modifiers::push::Push;
use crate::operators::OperatorConversions;
use crate::tree::{Ast, CanPush};
use utils::display;
use crate::{EMPTY, Number};

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
                    value: DeclarationValue::Value(Ast::Empty),
                })],
            }),
            VariableName::Keyword(_) => Err("Can't assign to function keyword."),
        }
    }

    /// Takes the attributes from inside self it is a type;
    pub fn into_type(self) -> Option<Vec<Attribute>> {
        self.declarations.is_empty().then_some(self.attrs)
    }

    /// Adds an attribute to the variable
    pub fn push_attr(&mut self, attr: Attribute) {
        if self.declarations.len() <= 1 {
            if let Some(Some(last)) = self.declarations.pop() {
                if last.value.is_none() {
                    self.attrs.push(Attribute::User(last.name));
                } else {
                    unreachable!(
                        "Trying to push attribute after variable initialisation expression."
                    )
                }
            }
            self.attrs.push(attr);
        } else {
            unreachable!("tried to push attribute on multiple variables")
        }
    }

    /// Pushes a colon `:` into a variable node.
    pub fn push_colon(&mut self) -> Result<(), String> {
        match self
            .declarations
            .last_mut()
            .and_then(|opt| opt.as_mut())
            .map(|decl| &mut decl.value)
        {
            None => Err("Expected variable name, found `:`".into()),
            Some(value @ DeclarationValue::None) => {
                *value = DeclarationValue::Bitfield(None);
                Ok(())
            }
            Some(DeclarationValue::Value(ast)) => ast.handle_colon(),
            Some(DeclarationValue::Bitfield(_)) =>
                Err("found 2 successive colons in struct declaration".into()),
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
                match &mut decl.value {
                    DeclarationValue::None => {
                        self.attrs.push(Attribute::User(mem::take(&mut decl.name)));
                        decl.name = name;
                        Ok(())
                    }
                    DeclarationValue::Value(ast) =>
                        ast.push_block_as_leaf(Ast::Variable(Variable::from(name))),
                    DeclarationValue::Bitfield(_) =>
                        Err("Found unexpected identifier after bitfield specifier".into()),
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
                Err("Successive literals in variable declaration. Found attribute after comma"
                    .to_owned())
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

    fn make_function(&mut self, depth: u32, arguments: Vec<Ast>) {
        match self
            .declarations
            .last_mut()
            .expect("checked with can_make_function")
        {
            Some(declaration) => declaration
                .value
                .as_mut()
                .expect("checked with can_make_function")
                .make_function(depth, arguments),
            None => unreachable!(),
        }
    }
}

impl CanPush for AttributeVariable {
    fn can_push_leaf(&self) -> bool {
        self.declarations.last().is_some_and(|opt| {
            opt.as_ref().is_some_and(|decl| match &decl.value {
                DeclarationValue::Bitfield(size) => size.is_none(),
                DeclarationValue::None => false,
                DeclarationValue::Value(ast) => ast.can_push_leaf(),
            })
        })
    }
}

impl From<AttributeKeyword> for AttributeVariable {
    fn from(value: AttributeKeyword) -> Self {
        Self { attrs: vec![Attribute::Keyword(value)], declarations: vec![] }
    }
}

impl Push for AttributeVariable {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::Print::push_leaf(&ast, self, "attr var");
        match &mut self
            .declarations
            .last_mut()
            .ok_or("Found non empty declarations")?
            .as_mut()
            .ok_or("Missing name for last declaration")?
            .value
        {
            DeclarationValue::Value(val) => val.push_block_as_leaf(ast),
            val @ (DeclarationValue::None | DeclarationValue::Bitfield(Some(_))) => Err(format!(
                "Found successive literals in variable declaration. Did you forget a {}?",
                if matches!(val, DeclarationValue::None) {
                    '='
                } else {
                    ';'
                }
            )),
            DeclarationValue::Bitfield(size @ None) =>
                if let Ast::Leaf(Literal::Number(nb)) = ast {
                    *size = Some(nb);
                    Ok(())
                } else {
                    Err("Expected bitfield size, but `:` is followed by a non-number token"
                        .to_owned())
                },
        }
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        crate::errors::Print::push_op(&op, self, "attr var");
        match self.declarations.last_mut() {
            Some(Some(last)) => match &mut last.value {
                DeclarationValue::Value(ast) => ast.push_op(op),
                DeclarationValue::Bitfield(_) =>
                    Err("Found operator after bitfield specifier but this is not allowed".into()),
                DeclarationValue::None =>
                    if op.is_star() {
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
                        last.value = DeclarationValue::Value(Ast::Empty);
                        Ok(())
                    } else {
                        Err("Can't push operator in empty declaration: missing `=`.".to_owned())
                    },
            },
            Some(None) =>
                Err("Can't push operator in empty declaration: missing variable name.".to_owned()),
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
        self.declarations.iter().any(|opt| {
            opt.as_ref()
                .is_some_and(|decl| matches!(decl.value, DeclarationValue::Value(_)))
        })
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
                "Trying to convert declarations to attributes, but this is illegal.".to_owned()
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

display!(
    AttributeVariable,
    self,
    f,
    write!(f, "({}:{})", repr_vec_attr(&self.attrs), repr_option_vec(&self.declarations),)
);

impl traits::PureType for AttributeVariable {
    fn is_pure_type(&self) -> bool {
        self.declarations
            .last()
            .is_none_or(|opt| opt.as_ref().is_none_or(|decl| decl.value.is_none()))
    }

    fn take_pure_type(&mut self) -> Option<Vec<Attribute>> {
        self.is_pure_type().then(|| {
            if let Some(Some(Declaration { name, value })) = self.declarations.last_mut() {
                debug_assert!(value.is_none(), "checked with is_pure_type");
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
    value: DeclarationValue,
}

impl From<String> for Declaration {
    fn from(name: String) -> Self {
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
        DeclarationValue::Bitfield(size) => write!(
            f,
            "({}:{})",
            self.name,
            size.as_ref()
                .map_or_else(|| EMPTY.into(), ToString::to_string)
        ),
    }
);

/// Value of the declaration
///
/// This is meant to represent anything following the variable name that is only
/// associated with this variable declaration, and not to other variables
/// declared with the same statement.
#[derive(Debug, Default)]
enum DeclarationValue {
    /// A `:` sign was found after the name, meaning a bitfield specifier.
    Bitfield(Option<Number>),
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
    const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}
