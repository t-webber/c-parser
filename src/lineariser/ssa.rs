//! Module to define the [`Ssa`] structure.

use crate::lineariser::symbol::Symbol;

/// Static Single Assignment structure.
#[derive(Default, Debug)]
pub struct Ssa {
    /// List of global symbols (variarbles, functions, etc.)
    global_symbols: Vec<Symbol>,
}

impl Ssa {
    /// Returns the display string for the [`Ssa`], sorted to ensure it always
    /// outputs the same string.
    pub fn display(self) -> String {
        let mut symbols = self.global_symbols;
        symbols.sort_by_key(Symbol::id);
        symbols
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Adds a new global symbol in the SSA.
    pub fn push_symbol(&mut self, symbol: Symbol) {
        self.global_symbols.push(symbol);
    }
}
