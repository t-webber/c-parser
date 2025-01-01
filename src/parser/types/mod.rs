pub mod binary;
pub mod blocks;
pub mod literal;
pub mod operator;
pub mod ternary;
pub mod unary;

use core::fmt;

use binary::Binary;
use blocks::Block;
use literal::{Literal, Variable};
use operator::{Associativity, Operator};
use ternary::Ternary;
use unary::Unary;

use super::keyword::control_flow::node::ControlFlowNode;
use crate::parser::repr_vec;

/// Struct to represent the AST
#[derive(Debug, Default, PartialEq)]
pub enum Ast {
    Binary(Binary),
    Block(Block),
    ControlFlow(ControlFlowNode),
    #[default]
    Empty,
    FunctionCall(FunctionCall),
    Leaf(Literal),
    ListInitialiser(ListInitialiser),
    ParensBlock(ParensBlock),
    Ternary(Ternary),
    Unary(Unary),
    // TODO: CompoundLiteral(CompoundLiteral), Cast,  & SpecialUnary(SpecialUnary),
}

#[derive(Debug, PartialEq)]
pub struct CompoundLiteral {
    args: Vec<Ast>,
    full: bool,
    op: CompoundLiteralOperator,
    type_: String,
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for CompoundLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}){{{}}}", self.type_, repr_vec(&self.args))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CompoundLiteralOperator;

impl Operator for CompoundLiteralOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> u32 {
        1
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub args: Vec<Ast>,
    pub full: bool,
    pub op: FunctionOperator,
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

#[derive(Debug, PartialEq, Default)]
pub struct ListInitialiser {
    pub elts: Vec<Ast>,
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
