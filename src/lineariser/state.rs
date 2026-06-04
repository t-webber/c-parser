//! Walks the [`Ast`](crate::parser::api::Ast) and converts it to the [`Ssa`]

use crate::lineariser::ssa::Ssa;

/// Linearising State used to convert the parsed
/// [`Ast`](crate::parser::api::Ast) into a [`Ssa`].
#[derive(Default)]
pub struct LState {
    /// Current state of the built [`Ssa`]
    pub ssa: Ssa,
}
