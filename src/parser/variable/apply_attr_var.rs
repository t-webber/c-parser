//! Traits applied to attribute variables for building it.

use core::fmt;
use core::mem::take;

use crate::Ast;
use crate::errors::api::{ErrorLocation, Located};
use crate::parser::api::{
    Attribute, AttributeKeyword, AttributeVariable, Declaration, DeclarationValue, Literal, UserDefinedTypes
};
use crate::parser::modifiers::functions::{CanMakeFnRes, MakeFunction};
use crate::parser::modifiers::push::Push;
use crate::parser::operators::api::OperatorConversions;
use crate::parser::tree::api::CanPush;
use crate::parser::variable::api::{PureType, VariableConversion};

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
    ) -> Option<(Located<UserDefinedTypes>, Option<Located<String>>)> {
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
                Some((last.as_ref().transfer(|_| *user_type), Some(take(&mut decl.name))))
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

impl PureType for AttributeVariable {
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
