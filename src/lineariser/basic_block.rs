//! Defines the basic block logic, the elementary logic block of the
//! [`Ssa`](super::ssa::Ssa).

#![expect(dead_code, reason = "todo")]

use crate::EMPTY;
use crate::errors::api::{IntoError as _, Located};
use crate::lineariser::state::LState;
use crate::lineariser::symbol::{Type, Value};
use crate::parser::api::{
    Ast, AttributeVariable, BracedBlock, ControlFlowNode, Declaration, DeclarationValue, FunctionCall, VariableName, VariableValue
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
#[derive(Debug, Default)]
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

    /// Creates a new basic block from the given braced block.
    pub fn from_braced_block(body: BracedBlock, state: &mut LState) -> Self {
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
                VariableValue::AttributeVariable(attr) => Some(attr.push_in(bbs, state)),
                VariableValue::VariableName(_, VariableName::UserDefined(vname)) => state
                    .find_declaration(&vname)
                    .map_or_else(|| todo!(), |decl| Some(decl.metadata.id)),
                VariableValue::VariableName(_, VariableName::Keyword(_)) => todo!(),
            },
            Self::Leaf(lit) => Some(state.push_literal(lit.drop_location())),
            Self::BracedBlock(bb) => {
                state.increment_depth();
                for elt in bb.elts {
                    elt.push_in(bbs, state);
                }
                state.decrement_depth();
                None
            }
            Self::Binary(_)
            | Self::Cast(_)
            | Self::FunctionArgsBuild(_)
            | Self::ListInitialiser(_)
            | Self::ParensBlock(_)
            | Self::Ternary(_)
            | Self::Unary(_)
            | Self::ControlFlow(_) => todo!("{self:?}"),
        }
    }
}

impl AttributeVariable {
    /// Pushes some content into the [`BasicBlocks`].
    fn push_in(self, bbs: &mut BasicBlocks, state: &mut LState) -> usize {
        #[cfg(feature = "debug")]
        crate::lgp!(notab: "Pushing attr var {self}");
        let ty = self.attrs;
        let mut last_id = None;
        for decl in self.declarations.into_iter().flatten() {
            last_id = Some(decl.push_in(bbs, state, &ty));
        }
        last_id.unwrap_or_else(|| todo!())
    }
}

impl Declaration {
    /// Pushes some content into the [`BasicBlocks`].
    fn push_in(self, bbs: &mut BasicBlocks, state: &mut LState, ty: &Type) -> usize {
        #[cfg(feature = "debug")]
        crate::lgp!(notab: "Pushing decl {self} with type {ty:?}");
        let (name, value) = self.into_name_value();
        let init_value = match value {
            DeclarationValue::None => Value::DeclaredOnly,
            DeclarationValue::Value(Ast::Leaf(lit)) => Value::Literal(lit.drop_location()),
            DeclarationValue::Value(ast) => ast
                .push_in(bbs, state)
                .map_or_else(|| todo!(), Value::Variable),
            DeclarationValue::Bitfield(_) => todo!(),
        };
        state.push_declaration(name, ty, init_value)
    }
}

impl FunctionCall {
    /// Pushes some content into the [`BasicBlocks`].
    fn push_in(self, bbs: &mut BasicBlocks, state: &mut LState) -> Option<usize> {
        let Self { mut arguments, function_body, variable } = self;

        match variable.into_value() {
            VariableValue::AttributeVariable(attr) =>
                if let Some((name, ret)) = attr.into_single_variable() {
                    declare_function(name, arguments, ret, function_body, state);
                    None
                } else {
                    todo!()
                },
            VariableValue::VariableName(loc, VariableName::UserDefined(name))
                if function_body.is_some() =>
            {
                state.push_error(loc.to_fault(format!("Missing return type for function {name}")));
                declare_function(loc.wrap(name), arguments, vec![], function_body, state);
                None
            }
            VariableValue::VariableName(loc, VariableName::Keyword(kwd))
                if function_body.is_some() =>
            {
                state.push_error(loc.to_fault(format!(
                    "Attempt to declare function with an invalid name, `{kwd}` is a keyword"
                )));
                None
            }
            VariableValue::VariableName(loc, VariableName::UserDefined(name)) => {
                if let Some(func) = state.find_function(&name) {
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
                    state.push_error(loc.into_fault(format!("Call of undeclared function {name}")));
                    None
                }
            }
            VariableValue::VariableName(loc, VariableName::Keyword(kwd)) => {
                if arguments.len() > 1 {
                    state.push_error(loc.into_fault(format!(
                        "Too many arguments in call to `{kwd}`: expected 1, got {}",
                        arguments.len()
                    )));
                    return None;
                }
                let Some(_) = arguments.pop() else {
                    state.push_error(loc.into_fault(format!(
                        "Missing argument in call to `{kwd}`: expected 1, got 0",
                    )));
                    return None;
                };
                todo!()
            }
        }
    }
}

/// Declares a function with the given signature.
fn declare_function(
    name: Located<String>,
    arguments: Vec<Ast>,
    ret: Type,
    body: Option<BracedBlock>,
    state: &mut LState,
) {
    let mut args = vec![];
    for arg in arguments {
        if let Ast::Variable(arg_var) = arg
            && let VariableValue::AttributeVariable(arg_attr) = arg_var.into_value()
            && let Some((_, arg_ty)) = arg_attr.into_single_variable()
        {
            args.push(arg_ty);
        } else {
            todo!()
        }
    }
    state.push_function(name, args, ret, body);
}
