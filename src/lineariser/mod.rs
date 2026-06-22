//! Linearises the Abstract Syntax Tree into a Static Single Assignment
//! structure.

#![expect(clippy::todo, reason = "in construction")]

mod basic_block;
mod declare;
mod ssa;
mod state;
mod symbol;
mod walker;

use crate::lineariser::ssa::Ssa;
use crate::lineariser::state::LState;
use crate::lineariser::walker::Linearise as _;
use crate::parser::api::Ast;
use crate::{BracedBlock, Res};

/// Converts an Abstract Syntax Tree into a Static Single Assignment.
#[must_use]
pub fn linearise(ast: BracedBlock) -> Res<Ssa> {
    let mut state = LState::default();
    Ast::BracedBlock(ast).linearise(&mut state);
    state.into_ssa()
}
