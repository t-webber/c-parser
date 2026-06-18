//! Defines the basic block logic, the elementary logic block of the
//! [`Ssa`](super::ssa::Ssa).

#![expect(dead_code, reason = "todo")]

use crate::parser::api::BracedBlock;
use crate::utils::display;

/// Elementary logic block of the [`Ssa`](super::ssa::Ssa).
///
/// It contains a label, a symbol table and a list of operations.
pub struct BasicBlock {
    /// Unique id of the basic block.
    id: usize,
}

impl BasicBlock {
    /// Creates a new basic block from the given function body.
    pub fn from_function_body(id: usize, _: BracedBlock) -> Self {
        Self { id }
    }
}

display!(BasicBlock, self, f, write!(f, ".."));
