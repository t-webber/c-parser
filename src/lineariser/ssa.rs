//! Module to define the [`Ssa`] structure.

use crate::EMPTY;
use crate::parser::api::Literal;
use crate::utils::display;

/// Static Single Assignment structure.
#[derive(Default)]
pub struct Ssa {
    /// List of global symbols (variarbles, functions, etc.)
    pub global_symbols: Vec<Symbol>,
}

display!(Ssa, self, f, {
    write!(f, "symbols:[")?;
    for symb in &self.global_symbols {
        symb.fmt(f)?;
    }
    write!(f, "]")
});

/// A symbol that can be defined or declared.
pub struct Symbol {
    /// Unique index to denote this variable.
    pub id: usize,
    /// Initialisation value, if any.
    pub init_value: Option<Literal>,
    /// Original name of this symbol in the source code.
    pub name: String,
}

display!(Symbol, self, f, {
    write!(
        f,
        "{:2} {} = {}",
        self.id,
        self.name,
        self.init_value
            .as_ref()
            .map_or_else(|| EMPTY.to_owned(), |val| format!("{val}"))
    )
});
