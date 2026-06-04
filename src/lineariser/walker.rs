//! Implements the walker that is actually going to visit the nodes of the
//! [`Ast`].

use crate::lineariser::state::LState;
use crate::parser::api::Ast;

impl Ast {
    /// Walks the [`Ast`] to linearise it into the given [`LState`]
    pub fn linearise(self, _state: &mut LState) {
        match self {
            Self::Binary(_)
            | Self::BracedBlock(_)
            | Self::Cast(_)
            | Self::ControlFlow(_)
            | Self::Empty
            | Self::FunctionArgsBuild(_)
            | Self::FunctionCall(_)
            | Self::Leaf(_)
            | Self::ListInitialiser(_)
            | Self::ParensBlock(_)
            | Self::Ternary(_)
            | Self::Unary(_)
            | Self::Variable(_) => todo!(),
        }
    }
}
