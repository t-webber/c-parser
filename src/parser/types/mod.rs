//! Module that defines the main node types of the [`Ast`]

pub mod binary;
pub mod blocks;
pub mod literal;
pub mod operator;
pub mod ternary;
pub mod unary;

use core::fmt;

use binary::Binary;
use blocks::BracedBlock;
use literal::{Literal, Variable};
use operator::{Associativity, Operator};
use ternary::Ternary;
use unary::Unary;

use super::keyword::control_flow::node::ControlFlowNode;
use crate::parser::repr_vec;

/// Struct to represent the AST
#[derive(Debug, Default, PartialEq)]
pub enum Ast {
    /// Binary operator
    Binary(Binary),
    /// Braced-block, in `{...}`.
    ///
    /// A whole file is considered to be a block.
    BracedBlock(BracedBlock),
    /// Control Flow blocks
    ControlFlow(ControlFlowNode),
    /// Empty AST
    #[default]
    Empty,
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
    // TODO: CompoundLiteral(CompoundLiteral), Cast, SpecialUnary(SpecialUnary),
}

/// Function call
///
/// This node represents functions declaration, functions
#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    /// arguments of the function
    pub args: Vec<Ast>,
    /// indicates whether the closing parenthesis for the arguments was found or
    /// not
    ///
    /// If full is false, we can still push arguments inside.
    pub full: bool,
    /// Function operator
    ///
    /// This is a constant type, but is used to access the methods of the
    /// [`Operator`] trait.
    pub op: FunctionOperator,
    /// name of the function, and all its attributes (return type)
    pub variable: Variable,
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}\u{b0}({}{}))",
            self.variable,
            repr_vec(&self.args),
            if self.full { "" } else { ".." },
        )
    }
}

/// Function operator
///
/// This is a constant type, but is used to access the methods of the
/// [`Operator`] trait.
#[derive(Debug, PartialEq, Eq)]
pub struct FunctionOperator;

impl Operator for FunctionOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> u32 {
        1
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

/// Struct to represent parenthesis
///
/// The [`Ast`] is what is inside of the parenthesis.
///
/// # Examples
///
/// If the C source is `(x = 2)`, the node is a [`ParensBlock`] with value the
/// [`Ast`] of `x=2`.
#[derive(Debug, Default, PartialEq)]
pub struct ParensBlock(Box<Ast>);

impl ParensBlock {
    /// Adds parenthesis around an [`Ast`].
    ///
    /// # Examples
    ///
    /// ```ignore
    /// assert!(ParensBlock::make_parens_ast(Ast::Empty) == Ast::ParensBlock(Box::new(Ast::Empty)));
    /// ```
    pub fn make_parens_ast(node: Ast) -> Ast {
        Ast::ParensBlock(Self(Box::new(node)))
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for ParensBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.0)
    }
}
