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

// expands to a::b((many!(b c d)))
// but i want a::b(b::c(c(d)))

/// Automaton to parse the list of attributes into a structure type.
#[derive(Debug)]
pub enum TypeParsingState {
    /// The base type name was found.
    Base(ReturnType),
    /// The base type name was not yet found.
    NoBase(
        Vec<TypeDecorator>,
        Vec<IndirectionDecorator>,
        Option<UserDefinedTypes>,
        Vec<Located<FunctionAttribute>>,
    ),
}

impl Default for TypeParsingState {
    fn default() -> Self {
        Self::NoBase(vec![], vec![], None, vec![])
    }
}

impl TypeParsingState {
    /// Adds an attribute to the current type parsing state.
    #[expect(clippy::min_ident_chars, reason = "scoped shorthand")]
    pub fn add_attribute(&mut self, attr: &Located<Attribute>) {
        use AttributeKeyword as K;
        use SpecialAttributes as S;
        let loc = attr.as_location();
        match attr.as_value() {
            Attribute::Indirection => self.add_indirection(),
            Attribute::User(name) => self.add_type(TypeName::TypeDef(name.to_owned())),
            Attribute::Keyword(kwd) => match kwd {
                K::Modifiers(dec) => self.add_ty_dec(*dec),
                K::BasicDataType(base) => self.add_type(TypeName::BasicDataType(*base)),
                K::Qualifiers(dec) => self.add_indirection_dec(*dec),
                K::Storage(dec) => self.add_ty_dec(*dec),
                K::UserDefinedTypes(usr_def) => self.add_usr_def(*usr_def),
                K::SpecialAttributes(special) => match special {
                    S::UAtomic => self.add_ty_dec(TypeDecorator::Atomic),
                    S::Alignas => todo!(),
                    S::Inline => self.add_fn_attr(loc.wrap(FunctionAttribute::Inline)),
                    S::Restrict => self.add_indirection_dec(IndirectionDecorator::Restrict),
                    S::UGeneric => todo!(),
                    S::UNoreturn => self.add_fn_attr(loc.wrap(FunctionAttribute::NoReturn)),
                },
            },
        }
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
            Self::Base(ty) => ty.ty.indirections.push(vec![]),
            Self::NoBase(..) => todo!(),
        }
    }

    /// Adds an indirection decorator to the current type parsing state.
    fn add_indirection_dec(&mut self, dec: impl Into<IndirectionDecorator>) {
        match self {
            Self::NoBase(_, decs, ..) => decs.push(dec.into()),
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
    fn add_type(&mut self, base: TypeName) {
        *self = match take(self) {
            Self::NoBase(base_decorations, indirections, usr_def, attrs) =>
                Self::Base(ReturnType {
                    attrs,
                    ty: Type {
                        base: base.with(usr_def),
                        base_decorations,
                        indirections: vec![indirections],
                    },
                }),
            Self::Base(_) => todo!(),
        }
    }

    /// Adds a user defined type attribute, like `struct`.
    fn add_usr_def(&mut self, usr_def: UserDefinedTypes) {
        match self {
            Self::NoBase(.., old @ None, _) => *old = Some(usr_def),
            Self::Base(_) | Self::NoBase(..) => todo!(),
        }
    }

    /// Returns the type represented by the current parsing state.
    pub fn into_type(mut self, loc: ErrorLocation) -> Res<ReturnType> {
        match self {
            Self::NoBase(..) => {
                self.add_type(TypeName::BasicDataType(BasicDataType::Int));
                self.into_type(loc).add_err(loc.fail(
                    "Missing type name in type expression, C23 doesn't default to int.".to_owned(),
                ))
            }
            Self::Base(this) => Res::ok(this),
        }
    }
}
