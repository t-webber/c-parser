//! Module to define the [`Ssa`] structure.

use crate::EMPTY;
use crate::parser::api::{Attribute, Literal};
use crate::utils::{display, repr_vec_comma_space, repr_vec_space};

/// Static Single Assignment structure.
#[derive(Default)]
pub struct Ssa {
    /// List of global symbols (variarbles, functions, etc.)
    pub global_symbols: Vec<Symbol>,
}

display!(Ssa, self, f, {
    write!(f, "Symbols:")?;
    for symb in &self.global_symbols {
        write!(f, "\n{symb}")?;
    }
    Ok(())
});

/// Short hand to represent the `type` type, i.e., a list of attributes.
type Type = Vec<Attribute>;

/// A symbol that can be defined or declared.
pub enum Symbol {
    //TODO: this shouldn't be an enum, a function is a variable.
    /// Simple element that can be assigned.
    Element {
        /// Unique index to denote this variable.
        id: usize,
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
        Self::Element { id, init_value } => write!(
            f,
            "  x{id} = {}",
            init_value
                .as_ref()
                .map_or_else(|| EMPTY.to_owned(), |val| format!("{val}"))
        ),
        Self::Function { id, args, ret } => write!(
            f,
            "  f{id}({}) -> {}",
            repr_vec_comma_space(args.as_slice()),
            repr_vec_space(ret)
        ),
    }
});
