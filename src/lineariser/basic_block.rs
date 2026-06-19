//! Defines the basic block logic, the elementary logic block of the
//! [`Ssa`](super::ssa::Ssa).

#![expect(dead_code, reason = "todo")]

use crate::EMPTY;
use crate::parser::api::{Ast, BracedBlock, ControlFlowNode, Literal};
use crate::utils::display;

/// List of instructions that can exist in a basic block.
enum Instruction {
    /// `return <expr>`
    Return(Literal),
}

display!(
    Instruction,
    self,
    f,
    match self {
        Self::Return(lit) => write!(f, "return {lit}"),
    }
);

/// List of basic blocks, that materialise a function body.
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
    pub fn from_function_body(body: BracedBlock) -> Self {
        let mut this = Self(vec![]);
        for ast in body.elts {
            ast.push_in(&mut this);
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

/// Action of pushing some content into the [`BasicBlocks`].
trait PushInBbs {
    /// Pushes some content into the [`BasicBlocks`].
    fn push_in(self, bbs: &mut BasicBlocks);
}

impl PushInBbs for Ast {
    fn push_in(self, bbs: &mut BasicBlocks) {
        if let Self::ControlFlow(ctrl) = self
            && let ControlFlowNode::Ast(return_ctrl) = ctrl
            && let Self::Leaf(lit) = *return_ctrl.into_value()
        {
            bbs.add(Instruction::Return(lit));
        } else {
            todo!()
        }
    }
}
