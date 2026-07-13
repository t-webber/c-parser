use core::mem::take;

use crate::Res;
use crate::errors::api::{ErrorLocation, Located};
use crate::lineariser::types::decorators::{
    FunctionAttribute, IndirectionDecorator, TypeDecorator
};
use crate::lineariser::types::name::TypeName;
use crate::lineariser::types::{ReturnType, Type};
use crate::parser::api::{
    Attribute, AttributeKeyword, BasicDataType, SpecialAttributes, UserDefinedTypes
};
use crate::utils::{display, repr_vec};

/// Automaton to parse the list of attributes into a structure type.
#[derive(Debug)]
pub enum TypeParsingState {
    /// The base type name was found.
    Base(ReturnType),
    /// The base type name was not yet found.
    NoBase(
        Vec<TypeDecorator>,
        Vec<Vec<IndirectionDecorator>>,
        Option<UserDefinedTypes>,
        Vec<Located<FunctionAttribute>>,
    ),
}

display!(
    TypeParsingState,
    self,
    f,
    match self {
        Self::Base(ty) => ty.fmt(f),
        Self::NoBase(base_dec, ind_dec, usr_def, fn_attr) => {
            repr_vec(fn_attr, " ").fmt(f)?;
            repr_vec(base_dec, " ").fmt(f)?;
            if let Some(usr_def_ty) = usr_def {
                usr_def_ty.fmt(f)?;
            }
            for inds in ind_dec {
                repr_vec(inds, " ").fmt(f)?;
                " * ".fmt(f)?;
            }
            Ok(())
        }
    }
);

impl Default for TypeParsingState {
    fn default() -> Self {
        Self::NoBase(vec![], vec![vec![]], None, vec![])
    }
}

impl TypeParsingState {
    /// Adds an attribute to the current type parsing state.
    #[expect(clippy::min_ident_chars, reason = "scoped shorthand")]
    pub fn add_attribute(&mut self, attr: &Located<Attribute>) -> Res<()> {
        #[cfg(feature = "debug")]
        crate::lgp!("Add attr {attr} to {self:?}");
        use AttributeKeyword as K;
        use SpecialAttributes as S;
        let loc = attr.as_location();
        match attr.as_value() {
            Attribute::Indirection => self.add_indirection(),
            Attribute::User(name) =>
                return self.add_type(loc.wrap(TypeName::TypeDef(name.to_owned()))),
            Attribute::Keyword(kwd) => match kwd {
                K::Modifiers(dec) => self.add_ty_dec(*dec),
                K::BasicDataType(base) =>
                    return self.add_type(loc.wrap(TypeName::BasicDataType(*base))),
                K::Qualifiers(dec) => self.add_indirection_dec(*dec),
                K::Storage(dec) => self.add_ty_dec(*dec),
                K::UserDefinedTypes(usr_def) => return self.add_usr_def(loc.wrap(*usr_def)),
                K::SpecialAttributes(special) => match special {
                    S::UAtomic => self.add_ty_dec(TypeDecorator::Atomic),
                    S::Alignas =>
                        return Res::ok(())
                            .add_err(loc.fail("alignas keyword not yet supported".to_owned())),
                    S::Inline => self.add_fn_attr(loc.wrap(FunctionAttribute::Inline)),
                    S::Restrict => self.add_indirection_dec(IndirectionDecorator::Restrict),
                    S::UGeneric =>
                        return Res::ok(())
                            .add_err(loc.fail("generic keyword not yet supported".to_owned())),
                    S::UNoreturn => self.add_fn_attr(loc.wrap(FunctionAttribute::NoReturn)),
                },
            },
        }
        Res::ok(())
    }

    /// Adds a function-only attribute to the current type parsing state.
    fn add_fn_attr(&mut self, attr: Located<FunctionAttribute>) {
        match self {
            Self::NoBase(.., attrs) => attrs.push(attr),
            Self::Base(ty) => ty.attrs.push(attr),
        }
    }

    /// Adds an indirection
    fn add_indirection(&mut self) {
        match self {
            Self::Base(ty) => {
                ty.ty.indirections.push(vec![]);
            }
            Self::NoBase(_, inds, ..) => inds.push(vec![]),
        }
    }

    /// Adds an indirection decorator to the current type parsing state.
    fn add_indirection_dec(&mut self, dec: impl Into<IndirectionDecorator>) {
        match self {
            Self::NoBase(_, decs, ..) => decs.last_mut().expect(">=1").push(dec.into()),
            Self::Base(ty) => ty
                .ty
                .indirections
                .last_mut()
                .expect("never empty")
                .push(dec.into()),
        }
    }

    /// Adds a base type decorator to the current type parsing state.
    fn add_ty_dec(&mut self, dec: impl Into<TypeDecorator>) {
        match self {
            Self::NoBase(decs, ..) => decs.push(dec.into()),
            Self::Base(ty) => ty.ty.base_decorations.push(dec.into()),
        }
    }

    /// Adds the base type to the parsing state.
    fn add_type(&mut self, base: Located<TypeName>) -> Res<()> {
        let loc = base.as_location();
        match take(self) {
            Self::NoBase(base_decorations, indirections, usr_def, attrs) =>
                base.drop_location().with(usr_def, loc).map(|new_base| {
                    *self = Self::Base(ReturnType {
                        attrs,
                        ty: Type { base: new_base, base_decorations, indirections },
                    });
                }),
            Self::Base(old) => {
                let res = Res::ok(()).add_err(
                    loc.fail(format!("Found another type name, previous was {}", old.ty.base)),
                );
                *self = Self::Base(old);
                res
            }
        }
    }

    /// Adds a user defined type attribute, like `struct`.
    fn add_usr_def(&mut self, usr_def: Located<UserDefinedTypes>) -> Res<()> {
        match self {
            Self::NoBase(.., old @ None, _) => {
                *old = Some(usr_def.drop_location());
                Res::ok(())
            }
            Self::NoBase(.., Some(old), _) => Res::ok(()).add_err(usr_def.as_location().fail(
                format!("Found `{usr_def}` after `{old}` supposed to be applied on the same type"),
            )),
            Self::Base(_) => Res::ok(()).add_err(
                usr_def
                    .as_location()
                    .fail(format!("Found {usr_def} after type name")),
            ),
        }
    }

    /// Returns the type represented by the current parsing state.
    pub fn into_type(mut self, loc: ErrorLocation) -> Res<ReturnType> {
        match self {
            Self::NoBase(..) => {
                self.add_type(loc.wrap(TypeName::BasicDataType(BasicDataType::Int)));
                self.into_type(loc)
                    .add_err(loc.fail("Missing variable name or type name".to_owned()))
            }
            Self::Base(this) => Res::ok(this),
        }
    }
}
