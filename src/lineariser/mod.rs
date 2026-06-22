//! Linearises the Abstract Syntax Tree into a Static Single Assignment
//! structure.

#![expect(clippy::todo, reason = "in construction")]

mod basic_block;
mod declare;
mod ssa;
mod state;
mod symbol;

use crate::lineariser::basic_block::BasicBlocks;
use crate::lineariser::ssa::Ssa;
use crate::lineariser::state::LState;
use crate::{BracedBlock, Res};

/// Converts an Abstract Syntax Tree into a Static Single Assignment.
#[must_use]
pub fn linearise(ast: BracedBlock) -> Res<Ssa> {
    let mut state = LState::default();
    state.init();
    let bbs = BasicBlocks::from_braced_block(ast, &mut state);
    state
        .into_symbol_list()
        .map(|symbols| Ssa { basic_blocks: bbs, symbols })
}
