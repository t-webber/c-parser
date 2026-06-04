//! Module to define the [`Ssa`] structure.

/// Static Single Assignment structure.
#[derive(Default)]
pub struct Ssa {
    /// List of global symbols (variarbles, functions, etc.)
    global_symbols: Vec<Symbol>,
}

/// A symbol that can be defined or declared.
pub struct Symbol;
