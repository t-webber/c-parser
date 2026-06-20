//! Module to define the [`Ssa`] structure.

use crate::lineariser::symbol::Symbol;
use crate::utils::display;

/// Static Single Assignment structure.
#[derive(Default, Debug)]
pub struct Ssa {
    /// List of global symbols (variarbles, functions, etc.)
    global_symbols: Vec<Symbol>,
}

impl Ssa {
    /// Adds a new global symbol in the SSA.
    pub fn push_symbol(&mut self, symbol: Symbol) {
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
