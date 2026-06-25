//! Walks a generic ast, updating state and creating symbols and basic blocks.

use crate::lineariser::basic_block::{BasicBlocks, Id, Instruction};
use crate::lineariser::state::LState;
use crate::lineariser::symbol::Value;
use crate::parser::api::{Ast, ControlFlowNode, Unary, VariableName, VariableValue};

impl Ast {
    /// Pushes some content into the basic blocks.
    pub fn push_in(self, bbs: &mut BasicBlocks, state: &mut LState) -> Option<Id> {
        #[cfg(feature = "debug")]
        crate::lgp!(notab: "Pushing ast {self}");
        match self {
            Self::ControlFlow(ControlFlowNode::Ast(return_ctrl)) =>
                return_ctrl.into_value().push_in(bbs, state).map_or_else(
                    || todo!(),
                    |ret| {
                        bbs.add(Instruction::Return(ret));
                        None
                    },
                ),
            Self::FunctionCall(func) => func.push_in(bbs, state),
            Self::Empty => None,
            Self::Variable(var) => match var.into_value() {
                VariableValue::AttributeVariable(attr) => {
                    attr.push_in(bbs, state);
                    None
                }
                VariableValue::VariableName(loc, VariableName::UserDefined(vname)) =>
                    #[expect(clippy::option_if_let_else, reason = "clippy bug")]
                    if let Some(decl) = state.find_declaration(&vname) {
                        Some(decl.metadata.id.into())
                    } else {
                        state.push_error(loc.fail(format!("Use of undeclared variable {vname}")));
                        Some(Id::NotFound)
                    },
                VariableValue::VariableName(_, VariableName::Keyword(_)) => todo!(),
            },
            Self::Leaf(lit) => Some(state.push_literal(lit.drop_location()).into()),
            Self::BracedBlock(bb) => {
                state.increment_depth();
                for elt in bb.elts {
                    elt.push_in(bbs, state);
                }
                state.decrement_depth();
                None
            }
            Self::Binary(bin) => Some(bin.push_in(bbs, state)),
            Self::Ternary(ter) => Some(ter.push_in(bbs, state)),
            Self::Unary(Unary { arg, op }) => {
                let loc = if arg.is_empty() {
                    op.as_location()
                } else {
                    arg.location()
                };
                match arg.push_in(bbs, state) {
                    Some(Id::NotFound) => Some(Id::NotFound),
                    Some(Id::Found(id)) => Some(
                        state
                            .push_element(Value::Unary(op.drop_location(), id), vec![])
                            .into(),
                    ),
                    None => {
                        state.stat_not_expr(loc);
                        Some(Id::NotFound)
                    }
                }
            }
            Self::Cast(_)
            | Self::FunctionArgsBuild(..)
            | Self::ListInitialiser(_)
            | Self::ParensBlock(_)
            | Self::ControlFlow(_) => todo!("{self:?}"),
        }
    }
}
