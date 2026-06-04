//! Module to define the [`Ssa`] structure.

use crate::parser::api::Literal;

/// Static Single Assignment structure.
#[derive(Default)]
pub struct Ssa {
    /// List of global symbols (variarbles, functions, etc.)
    pub global_symbols: Vec<Symbol>,
}

/// A symbol that can be defined or declared.
pub struct Symbol {
    /// Unique index to denote this variable.
    pub id: usize,
    /// Initialisation value, if any.
    pub init_value: Option<Literal>,
    /// Original name of this symbol in the source code.
    pub name: String,
}
