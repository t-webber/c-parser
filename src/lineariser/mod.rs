//! Linearises the Abstract Syntax Tree into a Static Single Assignment
//! structure.

#![expect(clippy::todo, dead_code, reason = "in construction")]
#![coverage(off)]

mod ssa;
mod state;
mod walker;

use crate::lineariser::ssa::Ssa;
use crate::lineariser::state::LState;
use crate::parser::api::Ast;

/// Converts an [`Ast`] to a [`Ssa`].
pub fn linearise(ast: Ast) -> Ssa {
    let mut state = LState::default();
    ast.linearise(&mut state);
    state.ssa
}
