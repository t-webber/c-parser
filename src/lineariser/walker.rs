//! Implements the walker that is actually going to visit the nodes of the
//! [`Ast`].

use crate::lineariser::state::LState;
use crate::parser::api::{Ast, Variable, VariableValue};

/// Trait to factor signature and documentation.
pub trait Linearise {
    /// Walks the node to linearise it into the given [`LState`]
    fn linearise(self, state: &mut LState);
}

impl Linearise for Ast {
    fn linearise(self, state: &mut LState) {
        match self {
            Self::Binary(_)
            | Self::Cast(_)
            | Self::ControlFlow(_)
            | Self::FunctionArgsBuild(_)
            | Self::FunctionCall(_)
            | Self::Leaf(_)
            | Self::ListInitialiser(_)
            | Self::ParensBlock(_)
            | Self::Ternary(_)
            | Self::Unary(_) => todo!("{self}"),
            Self::BracedBlock(block) =>
                for node in block.elts {
                    state.increment_depth();
                    node.linearise(state);
                    state.decrement_depth();
                },
            Self::Variable(var) => var.linearise(state),
            Self::Empty => (),
        }
    }
}

impl Linearise for Variable {
    fn linearise(self, state: &mut LState) {
        match self.into_value() {
            this @ VariableValue::VariableName(_) => todo!("{this}"),
            VariableValue::AttributeVariable(attr) => attr.declare(state),
        }
    }
}
