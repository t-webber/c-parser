//! Defines the basic block logic, the elementary logic block of the
//! [`Ssa`](super::ssa::Ssa).

#![expect(dead_code, reason = "todo")]

use crate::EMPTY;
use crate::errors::api::IntoError as _;
use crate::lineariser::state::LState;
use crate::lineariser::symbol::Value;
use crate::parser::api::{
    Ast, BracedBlock, ControlFlowNode, FunctionCall, VariableName, VariableValue
};
use crate::utils::display;

/// List of instructions that can exist in a basic block.
#[derive(Debug)]
pub enum Instruction {
    /// `return <expr>`
    Return(usize),
}

display!(
    Instruction,
    self,
    f,
    match self {
        Self::Return(lit) => write!(f, "return x{lit}"),
    }
);

/// List of basic blocks, that materialise a function body.
#[derive(Debug)]
pub struct BasicBlocks(Vec<Vec<Instruction>>);

impl BasicBlocks {
    /// Adds a line to the last basic block
    fn add(&mut self, inst: Instruction) {
        if let Some(last) = self.0.last_mut() {
            last.push(inst);
        } else {
            self.0.push(vec![inst]);
        }
    }

    /// Creates a new basic block from the given function body.
    pub fn from_function_body(body: BracedBlock, state: &mut LState) -> Self {
        let mut this = Self(vec![]);
        for ast in body.elts {
            ast.push_in(&mut this, state);
        }
        this
    }
}

display!(
    BasicBlocks,
    self,
    f,
    if self.0.is_empty() {
        write!(f, " {EMPTY}")
    } else {
        for (id, bb) in self.0.iter().enumerate() {
            write!(f, "\n{:2}BB{id}:", "")?;
            for inst in bb {
                write!(f, "\n{:4}{inst}", "")?;
            }
        }
        Ok(())
    }
);

impl Ast {
    /// Pushes some content into the [`BasicBlocks`].
    fn push_in(self, bbs: &mut BasicBlocks, state: &mut LState) -> Option<usize> {
        match self {
            Self::ControlFlow(ControlFlowNode::Ast(return_ctrl)) =>
                return_ctrl.into_value().push_in(bbs, state).map_or_else(
                    || todo!(),
                    |ret| {
                        bbs.add(Instruction::Return(ret));
                        None
                    },
                ),
            Self::FunctionCall(FunctionCall { function_body: None, variable, arguments }) =>
                if let VariableValue::VariableName(loc, VariableName::UserDefined(fname)) =
                    variable.into_value()
                {
                    if let Some(func) = state.find_function(&fname) {
                        let ty = func.ret.clone();
                        let fid = func.id;
                        let mut args = vec![];
                        for arg in arguments {
                            let Some(id) = arg.push_in(bbs, state) else {
                                todo!()
                            };
                            args.push(id);
                        }
                        Some(state.push_element(Value::Call(fid, args), ty))
                    } else {
                        state.push_error(
                            loc.into_fault(format!("Call of undeclared function {fname}")),
                        );
                        None
                    }
                } else {
                    todo!()
                },
            Self::Empty => None,
            Self::Variable(var) => match var.into_value() {
                VariableValue::AttributeVariable(attr) => Some(attr.declare(state)),
                VariableValue::VariableName(_, VariableName::UserDefined(vname)) => state
                    .find_declaration(&vname)
                    .map_or_else(|| todo!(), |func| Some(func.id)),
                VariableValue::VariableName(_, VariableName::Keyword(_)) => todo!(),
            },
            Self::Leaf(lit) => Some(state.push_literal(lit)),
            Self::Binary(_)
            | Self::BracedBlock(_)
            | Self::Cast(_)
            | Self::FunctionArgsBuild(_)
            | Self::FunctionCall(_)
            | Self::ListInitialiser(_)
            | Self::ParensBlock(_)
            | Self::Ternary(_)
            | Self::Unary(_)
            | Self::ControlFlow(_) => todo!("{self:?}"),
        }
    }
}
