use crate::parser::api::{Modifiers, Qualifiers, Storage};
use crate::utils::{display, from};

/// Attributes that are only valid on function return types.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FunctionAttribute {
    /// Inline keyword
    Inline,
    /// Noreturn keyword
    NoReturn,
}

display!(
    FunctionAttribute,
    self,
    f,
    match self {
        FunctionAttribute::Inline => "inline",
        FunctionAttribute::NoReturn => "noreturn",
    }
    .fmt(f)
);

/// All qualifiers, modifiers, etc. that can be added around an indirection or
/// type name.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IndirectionDecorator {
    /// cf. [`Qualifiers`]
    Qualifiers(Qualifiers),
    /// Restrict keyword
    Restrict,
}

display!(
    IndirectionDecorator,
    self,
    f,
    match self {
        Self::Qualifiers(qual) => qual.fmt(f),
        Self::Restrict => "restrict".fmt(f),
    }
);

from!(Qualifiers IndirectionDecorator);

/// All qualifiers, modifiers, etc. that can be added around an indirection or
/// type name.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TypeDecorator {
    /// `atomic` keyword
    Atomic,
    /// cf. [`Modifiers`]
    Modifiers(Modifiers),
    /// cf. [`Storage`]
    Storage(Storage),
}

display!(
    TypeDecorator,
    self,
    f,
    match self {
        TypeDecorator::Modifiers(modifiers) => modifiers.fmt(f),
        TypeDecorator::Storage(storage) => storage.fmt(f),
        TypeDecorator::Atomic => "atomic".fmt(f),
    }
);

from!(Modifiers TypeDecorator);
from!(Storage TypeDecorator);
