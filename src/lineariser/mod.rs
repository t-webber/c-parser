//! Linearises the Abstract Syntax Tree into a Static Single Assignment
//! structure.

#![expect(clippy::todo, reason = "in construction")]
#![coverage(off)]

mod declare;
mod ssa;
mod state;
mod walker;

use crate::Res;
use crate::lineariser::ssa::Ssa;
use crate::lineariser::state::LState;
use crate::lineariser::walker::Linearise as _;
use crate::parser::api::Ast;

/// Converts an Abstract Syntax Tree into a Static Single Assignment.
#[must_use]
pub fn linearise(ast: Ast) -> Res<Ssa> {
    let mut state = LState::default();
    ast.linearise(&mut state);
    state.into_ssa()
}
