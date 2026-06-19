//! Module to define the [`Ssa`] structure.

use crate::lineariser::symbol::Symbol;
use crate::utils::display;

/// Static Single Assignment structure.
#[derive(Default)]
pub struct Ssa {
    /// List of global symbols (variarbles, functions, etc.)
    global_symbols: Vec<Symbol>,
}

impl Ssa {
    /// Returns the symbol associated with a given id.
    pub fn get_symbol_mut(&mut self, id: usize) -> Option<&mut Symbol> {
        self.global_symbols.get_mut(id)
    }

    /// Adds a new global symbol in the SSA.
    pub fn push_symbol(&mut self, symbol: Symbol) {
        debug_assert_eq!(self.global_symbols.len(), symbol.id(), "brakes invariant");
        self.global_symbols.push(symbol);
    }
}

display!(Ssa, self, f, {
    self.global_symbols
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
        .fmt(f)
});
