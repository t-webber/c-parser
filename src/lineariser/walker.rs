//! Implements the walker that is actually going to visit the nodes of the
//! [`Ast`].

use crate::lineariser::state::LState;
use crate::parser::api::{Ast, FunctionCall, Variable, VariableValue};

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
            | Self::Leaf(_)
            | Self::ListInitialiser(_)
            | Self::ParensBlock(_)
            | Self::Ternary(_)
            | Self::Unary(_) => todo!("{self:?}"),
            Self::BracedBlock(block) => {
                state.increment_depth();
                for node in block.elts {
                    node.linearise(state);
                }
                state.decrement_depth();
            }
            Self::Variable(var) => var.linearise(state),
            Self::Empty => (),
            Self::FunctionCall(func) => func.linearise(state),
        }
    }
}

impl Linearise for Variable {
    fn linearise(self, state: &mut LState) {
        match self.into_value() {
            this @ VariableValue::VariableName(..) => todo!("{this:?}"),
            VariableValue::AttributeVariable(attr) => attr.declare(state),
        }
    }
}

impl Linearise for FunctionCall {
    fn linearise(self, state: &mut LState) {
        if let VariableValue::AttributeVariable(var) = self.variable.into_value()
            && let Some((name, ret)) = var.into_single_variable()
        {
            let mut args = vec![];
            for arg in self.arguments {
                if let Ast::Variable(arg_var) = arg
                    && let VariableValue::AttributeVariable(arg_attr) = arg_var.into_value()
                    && let Some((_, arg_ty)) = arg_attr.into_single_variable()
                {
                    args.push(arg_ty);
                } else {
                    todo!()
                }
            }
            state.push_function(name, args, ret, None);
        } else {
            todo!()
        }
    }
}
