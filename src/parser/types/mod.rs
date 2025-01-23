//! Module that defines the main node types of the [`Ast`]

pub mod binary;
pub mod braced_blocks;
pub mod literal;
pub mod operator;
pub mod parens;
pub mod ternary;
pub mod unary;
pub mod variable;

use core::fmt;

use binary::Binary;
use braced_blocks::BracedBlock;
use literal::Literal;
use operator::{Associativity, Operator};
use parens::{Cast, ParensBlock};
use ternary::Ternary;
use unary::Unary;
use variable::Variable;

use super::keyword::control_flow::node::ControlFlowNode;
use crate::parser::repr_vec;

/// Struct to represent the Abstract Syntax Tree of the whole C source file.
///
/// # Note
///
/// Can't derive [`Eq`] because it is not implemented for [`f32`].
#[derive(Debug, Default, PartialEq)]
pub enum Ast {
    /// Binary operator
    Binary(Binary),
    /// Braced-block, in `{...}`.
    ///
    /// A whole file is considered to be a block.
    BracedBlock(BracedBlock),
    /// Cast
    Cast(Cast),
    /// Control Flow blocks
    ControlFlow(ControlFlowNode),
    /// Empty AST
    #[default]
    Empty,
    /// Function arguments: `(x+y, !g(z), (a, !b)++, )`
    FunctionArgsBuild(Vec<Ast>),
    /// Function call
    FunctionCall(FunctionCall),
    /// Literal (constants, variables, etc.)
    Leaf(Literal),
    /// List initialiser: `{1, 2, 3, [6]=7}`
    ListInitialiser(ListInitialiser),
    /// Ast surrounded by parenthesis: `(x=2)`
    ParensBlock(ParensBlock),
    /// Ternary operator
    Ternary(Ternary),
    /// Unary operator
    Unary(Unary),
    /// Variables
    Variable(Variable),
}

/// Function call
///
/// This node represents functions declaration, functions
#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    /// arguments of the function
    pub args: Vec<Ast>,
    /// name of the function, and all its attributes (return type)
    pub variable: Variable,
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}\u{b0}({}))", self.variable, repr_vec(&self.args),)
    }
}

/// List initialiser
///
/// Node to represent list initialisers, such as `{1, 2, 3, [6]=12}`.
#[derive(Debug, PartialEq, Default)]
pub struct ListInitialiser {
    /// elements of the list
    pub elts: Vec<Ast>,
    /// indicates whether the closing `}` was found yet.
    ///
    /// If full is false, we can still push elements inside.
    pub full: bool,
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for ListInitialiser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.elts
                .iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
