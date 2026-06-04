//! Walks the [`Ast`](crate::parser::api::Ast) and converts it to the [`Ssa`]

use crate::lineariser::ssa::{Ssa, Symbol};

/// Linearising State used to convert the parsed
/// [`Ast`](crate::parser::api::Ast) into a [`Ssa`].
#[derive(Default)]
pub struct LState {
    /// Unique id of the next symbol to be declared.
    next_symbol_id: usize,
    /// Current state of the built [`Ssa`]
    ssa: Ssa,
}

impl LState {
    /// Increment the id and return the one that can be used.
    ///
    /// This function ensures that every id is unique.
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "todo: fail when no more ids available"
    )]
    pub const fn get_and_bump_symbol_id(&mut self) -> usize {
        let old = self.next_symbol_id;
        self.next_symbol_id += 1;
        old
    }

    /// Returns the inner [`Ssa`]
    pub fn into_ssa(self) -> Ssa {
        self.ssa
    }

    /// Pushes a [`Symbol`] in the appropriate symbol table.
    pub fn push_symbol(&mut self, symbol: Symbol) {
        self.ssa.global_symbols.push(symbol);
    }
}
