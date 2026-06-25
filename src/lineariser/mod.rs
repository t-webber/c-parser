//! Linearises the Abstract Syntax Tree into a Static Single Assignment
//! structure.

#![expect(clippy::todo, reason = "in construction")]

mod basic_block;
mod macros;
mod ssa;
mod state;
mod symbol;
mod walk;

use crate::lineariser::basic_block::BasicBlocks;
use crate::lineariser::ssa::Ssa;
use crate::lineariser::state::LState;
use crate::utils::StringResolver;
use crate::{BracedBlock, Res};

/// Converts an Abstract Syntax Tree into a Static Single Assignment.
#[must_use]
pub fn linearise(ast: StringResolver<BracedBlock>) -> Res<StringResolver<Ssa>> {
    ast.map_res(|bb| {
        let mut state = LState::default();
        state.init();
        let bbs = BasicBlocks::from_braced_block(bb, &mut state);
        state
            .into_symbol_list()
            .map(|symbols| Ssa { basic_blocks: bbs, symbols })
    })
}
