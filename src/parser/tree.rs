use core::fmt;

use crate::lexer::api::types::Number;

pub trait Operator: fmt::Debug {
    fn precedence(&self) -> u32;
    fn associativity(&self) -> Associativity;
}

#[derive(Debug)]
pub enum Associativity {
    LeftToRight,
    RightToLeft,
}

#[derive(Debug)]
pub struct Binary {
    operator: BinaryOperator,
    arg_l: Option<Box<Node>>,
    arg_r: Option<Box<Node>>,
}

impl From<BinaryOperator> for Binary {
    fn from(operator: BinaryOperator) -> Self {
        Self {
            operator,
            arg_l: None,
            arg_r: None,
        }
    }
}

#[derive(Debug)]
pub enum BinaryOperator {
    // `[]`
    ArraySubscript,
    // (`.`)
    StructEnumMemberAccess,
    // (`->`)
    StructEnumMemberPointerAccess,
    Multiply,
    Divide,
    Modulo,
    Add,
    Subtract,
    RightShift,
    LeftShift,
    Lt,
    Le,
    Gt,
    Ge,
    Equal,
    Different,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    LeftShiftAssign,
    RightShiftAssign,
    AndAssign,
    XorAssign,
    OrAssign,
    Comma,
}

impl Operator for BinaryOperator {
    fn associativity(&self) -> Associativity {
        match self {
            Self::ArraySubscript
            | Self::StructEnumMemberAccess
            | Self::StructEnumMemberPointerAccess
            | Self::Multiply
            | Self::Divide
            | Self::Modulo
            | Self::Add
            | Self::Subtract
            | Self::RightShift
            | Self::LeftShift
            | Self::Lt
            | Self::Le
            | Self::Gt
            | Self::Ge
            | Self::Equal
            | Self::Different
            | Self::BitwiseAnd
            | Self::BitwiseXor
            | Self::BitwiseOr
            | Self::LogicalAnd
            | Self::LogicalOr
            | Self::Comma => Associativity::LeftToRight,
            Self::Assign
            | Self::AddAssign
            | Self::SubAssign
            | Self::MulAssign
            | Self::DivAssign
            | Self::ModAssign
            | Self::LeftShiftAssign
            | Self::RightShiftAssign
            | Self::AndAssign
            | Self::XorAssign
            | Self::OrAssign => Associativity::RightToLeft,
        }
    }

    fn precedence(&self) -> u32 {
        match self {
            Self::ArraySubscript
            | Self::StructEnumMemberAccess
            | Self::StructEnumMemberPointerAccess => 1,
            Self::Multiply | Self::Divide | Self::Modulo => 3,
            Self::Add | Self::Subtract => 4,
            Self::RightShift | Self::LeftShift => 5,
            Self::Lt | Self::Le | Self::Gt | Self::Ge => 6,
            Self::Equal | Self::Different => 7,
            Self::BitwiseAnd => 8,
            Self::BitwiseXor => 9,
            Self::BitwiseOr => 10,
            Self::LogicalAnd => 11,
            Self::LogicalOr => 12,
            Self::Assign
            | Self::AddAssign
            | Self::SubAssign
            | Self::MulAssign
            | Self::DivAssign
            | Self::ModAssign
            | Self::LeftShiftAssign
            | Self::RightShiftAssign
            | Self::AndAssign
            | Self::XorAssign
            | Self::OrAssign => 14,
            Self::Comma => 15,
        }
    }
}

#[derive(Debug)]
pub struct CompoundLiteral {
    args: Vec<Node>,
    operator: CompoundLiteralOperator,
    type_: String,
}

#[derive(Debug)]
pub struct CompoundLiteralOperator;

impl Operator for CompoundLiteralOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> u32 {
        1
    }
}

#[derive(Debug)]
pub struct FunctionCall {
    name: String,
    operator: FunctionOperator,
    args: Vec<Node>,
}

#[derive(Debug)]
pub struct FunctionOperator;

impl Operator for FunctionOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> u32 {
        1
    }
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Variable(String),
    Char(char),
    Str(String),
    Number(Number),
}

#[derive(Debug, Default)]
pub enum Node {
    #[default]
    Empty,
    Binary(Binary),
    CompoundLiteral(CompoundLiteral),
    FunctionCall(FunctionCall),
    Leaf(Literal),
    Ternary(Ternary),
    Unary(Unary),
    Vec(Vec<Node>),
    Block(Vec<Node>),
}

impl Node {
    pub fn try_push_leaf(&mut self, literal: Literal) -> Result<(), &'static str> {
        let node_leaf = Self::Leaf(literal);
        match self {
            Self::Empty => *self = node_leaf,
            // push in Option<Box<Node>>
            Self::Binary(
                Binary {
                    arg_l: last @ None, ..
                }
                | Binary {
                    arg_l: Some(_),
                    arg_r: last @ None,
                    ..
                },
            )
            | Self::Ternary(
                Ternary {
                    condition: last @ None,
                    ..
                }
                | Ternary {
                    success: last @ None,
                    ..
                }
                | Ternary {
                    failure: last @ None,
                    ..
                },
            )
            | Self::Unary(Unary {
                arg: last @ None, ..
            }) => *last = Some(Box::new(node_leaf)),
            // push in Vec<Node>
            Self::Block(vec)
            | Self::Vec(vec)
            | Self::FunctionCall(FunctionCall { args: vec, .. }) => vec.push(node_leaf),
            // todo
            Self::CompoundLiteral(_) => todo!(),
            // Errors
            Self::Leaf(_) => {
                return Err("Found 2 consecutive litteral without a logical relation.")
            }
            Self::Unary(_) => {
                return Err("Found 2 arguments for a unary operator. Did you forget an operator?")
            }
            Self::Binary(_) => {
                return Err("Found 3 arguments for a binary operator. Did you forget an operator?")
            }
            Self::Ternary(_) => {
                return Err(
                    "Found 4 arguments for the ternary operator. Did you forget an operator?",
                )
            }
        };
        Ok(())
    }
}

impl From<UnaryOperator> for Node {
    fn from(value: UnaryOperator) -> Self {
        Node::Unary(Unary::from(value))
    }
}

impl From<BinaryOperator> for Node {
    fn from(value: BinaryOperator) -> Self {
        Node::Binary(Binary::from(value))
    }
}

#[derive(Debug)]
pub struct Ternary {
    operator: TernaryOperator,
    condition: Option<Box<Node>>,
    success: Option<Box<Node>>,
    failure: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct TernaryOperator;

impl Operator for TernaryOperator {
    fn associativity(&self) -> Associativity {
        Associativity::RightToLeft
    }

    fn precedence(&self) -> u32 {
        13
    }
}

#[derive(Debug)]
pub struct Unary {
    operator: UnaryOperator,
    arg: Option<Box<Node>>,
}

impl From<UnaryOperator> for Unary {
    fn from(operator: UnaryOperator) -> Self {
        Self {
            operator,
            arg: None,
        }
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    Defined,
    PostfixIncrement,
    PostfixDecrement,
    PrefixIncrement,
    PrefixDecrement,
    Plus,
    Minus,
    BitwiseNot,
    LogicalNot,
    Cast(String),
    /// Dereference (`*`)
    Indirection,
    /// Address-of (`&`)
    AddressOf,
    SizeOf,
    AlignOf,
}

impl Operator for UnaryOperator {
    fn associativity(&self) -> Associativity {
        match self {
            Self::Defined | Self::PostfixIncrement | Self::PostfixDecrement => {
                Associativity::LeftToRight
            }
            Self::PrefixIncrement
            | Self::PrefixDecrement
            | Self::Plus
            | Self::Minus
            | Self::BitwiseNot
            | Self::LogicalNot
            | Self::Cast(_)
            | Self::Indirection
            | Self::AddressOf
            | Self::SizeOf
            | Self::AlignOf => Associativity::RightToLeft,
        }
    }

    fn precedence(&self) -> u32 {
        match self {
            Self::Defined => 0,
            Self::PostfixIncrement | Self::PostfixDecrement => 1,
            Self::PrefixIncrement
            | Self::PrefixDecrement
            | Self::Plus
            | Self::Minus
            | Self::BitwiseNot
            | Self::LogicalNot
            | Self::Cast(_)
            | Self::Indirection
            | Self::AddressOf
            | Self::SizeOf
            | Self::AlignOf => 2,
        }
    }
}
