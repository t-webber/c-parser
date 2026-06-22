//! Module to define the [`Ssa`] structure.

use crate::lineariser::basic_block::BasicBlocks;
use crate::lineariser::symbol::Symbol;

/// Static Single Assignment structure.
#[derive(Debug)]
pub struct Ssa {
    /// Basic blocks
    pub basic_blocks: BasicBlocks,
    /// List of global symbols (variarbles, functions, etc.)
    pub symbols: Vec<Symbol>,
}

impl Ssa {
    /// Returns the display string for the [`Ssa`], sorted to ensure it always
    /// outputs the same string.
    pub fn display(mut self) -> String {
        self.symbols.sort_by_key(Symbol::id);
        self.symbols
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    }
}
