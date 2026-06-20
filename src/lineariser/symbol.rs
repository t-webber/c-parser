//! Defines the [`Symbol`] item, in charge of representing global symbols and
//! forwarding them to the next compilation steps.

use crate::EMPTY;
use crate::lineariser::basic_block::BasicBlocks;
use crate::parser::api::{Attribute, Literal};
use crate::utils::{display, repr_vec_comma_space, repr_vec_space};

/// Short hand to represent the `type` type, i.e., a list of attributes.
pub type Type = Vec<Attribute>;

/// Temporal value to hold the part kept in the [`LState`](super::state::LState)
/// of a constant literal value.
///
/// Is converted to [`Symbol`] when pushed into the [`Ssa`](super::ssa::Ssa).
#[derive(Debug)]
pub struct LiteralBuilder {
    /// Unique index to denote this variable.
    pub id: usize,
    /// Type of the symbol.
    pub ty: Type,
}

impl LiteralBuilder {
    /// Adds the missing data to create an ssa symbol.
    pub const fn with_value(self, value: Literal) -> Symbol {
        Symbol::Element {
            name: None,
            value: ElementBuilder { init_value: Some(value), metadata: self },
        }
    }
}

/// Temporal value to hold the part kept in the [`LState`](super::state::LState)
/// of a declared variable, called `element` here.
///
/// Is converted to [`Symbol`] when pushed into the [`Ssa`](super::ssa::Ssa).
#[derive(Debug)]
pub struct ElementBuilder {
    /// Initialisation value, if any.
    pub init_value: Option<Literal>,
    /// Type and id of the element
    pub metadata: LiteralBuilder,
}

impl ElementBuilder {
    /// Adds the missing data to create an ssa symbol.
    pub const fn with_name(self, name: String) -> Symbol {
        Symbol::Element { name: Some(name), value: self }
    }
}

display!(
    ElementBuilder,
    self,
    f,
    write!(
        f,
        "{} x{} = {}",
        repr_vec_space(&self.metadata.ty),
        self.metadata.id,
        self.init_value
            .as_ref()
            .map_or_else(|| EMPTY.to_owned(), |val| format!("{val}"))
    )
);

/// Temporal value to hold the part kept in the [`LState`](super::state::LState)
/// of a function declaration.
///
/// Is converted to [`Symbol`] when pushed into the [`Ssa`](super::ssa::Ssa).
#[derive(Debug)]
pub struct FunctionBuilder {
    /// Type of the input arguments.
    pub args: Vec<Type>,
    /// Body of the function.
    pub body: Option<BasicBlocks>,
    /// Unique index to denote this variable.
    pub id: usize,
    /// Return type.
    pub ret: Type,
}

impl FunctionBuilder {
    /// Adds the missing data to create an ssa symbol.
    pub const fn with_name(self, name: String) -> Symbol {
        Symbol::Function { name, value: self }
    }
}

display!(
    FunctionBuilder,
    self,
    f,
    write!(
        f,
        "f{}({}) -> {}{}",
        self.id,
        repr_vec_comma_space(self.args.as_slice()),
        repr_vec_space(&self.ret),
        self.body
            .as_ref()
            .map_or_else(|| " ;".to_owned(), ToString::to_string)
    )
);

/// A symbol that can be defined or declared.
#[derive(Debug)]
pub enum Symbol {
    //TODO: this shouldn't be an enum, a function is a variable.
    /// Simple element that can be assigned.
    Element {
        /// Name of the symbol.
        ///
        /// There is no name if it is a literal constant.
        name: Option<String>,
        /// Value and parameters of the element.
        value: ElementBuilder,
    },
    /// Function that can be called
    Function {
        /// Name of the function.
        name: String,
        /// Value and parameters of the function.
        value: FunctionBuilder,
    },
}

impl Symbol {
    /// Returns the unique identifier of this symbol.
    pub const fn id(&self) -> usize {
        match self {
            Self::Element { value, .. } => value.metadata.id,
            Self::Function { value, .. } => value.id,
        }
    }
}

display!(Symbol, self, f, {
    match self {
        Self::Element { name, value } =>
            write!(f, "[{}] {value}", name.as_ref().map(String::as_str).unwrap_or_default()),
        Self::Function { name, value } => write!(f, "[{name}] {value}"),
    }
});
