//! Defines the [`Symbol`] item, in charge of representing global symbols and
//! forwarding them to the next compilation steps.

use crate::EMPTY;
use crate::lineariser::basic_block::BasicBlocks;
use crate::parser::api::{Attribute, Literal};
use crate::utils::{display, repr_vec_comma_space, repr_vec_space};

/// Short hand to represent the `type` type, i.e., a list of attributes.
pub type Type = Vec<Attribute>;

/// A symbol that can be defined or declared.
pub enum Symbol {
    //TODO: this shouldn't be an enum, a function is a variable.
    /// Simple element that can be assigned.
    Element {
        /// Unique index to denote this variable.
        id: usize,
        /// Type of the symbol.
        ty: Type,
        /// Initialisation value, if any.
        init_value: Option<Literal>,
    },
    /// Function that can be called
    Function {
        /// Unique index to denote this variable.
        id: usize,
        /// Type of the input arguments.
        args: Vec<Type>,
        /// Return type.
        ret: Type,
        /// Body of the function.
        body: Option<BasicBlocks>,
    },
}

impl Symbol {
    /// Returns the unique identifier of this symbol.
    pub const fn id(&self) -> usize {
        match self {
            Self::Element { id, .. } | Self::Function { id, .. } => *id,
        }
    }
}

display!(Symbol, self, f, {
    match self {
        Self::Element { id, ty, init_value } => write!(
            f,
            "{} x{id} = {}",
            repr_vec_space(ty),
            init_value
                .as_ref()
                .map_or_else(|| EMPTY.to_owned(), |val| format!("{val}"))
        ),
        Self::Function { id, args, ret, body } => write!(
            f,
            "f{id}({}) -> {}{}",
            repr_vec_comma_space(args.as_slice()),
            repr_vec_space(ret),
            body.as_ref()
                .map_or_else(|| " ;".to_owned(), ToString::to_string)
        ),
    }
});
