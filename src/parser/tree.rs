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
    BitwiseRightShift,
    BitwiseLeftShift,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    Equal,
    Different,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivivdeAssign,
    ModuloAssign,
    BitwiseLeftShiftAssign,
    BitwiseRightShiftAssign,
    BitwiseAndAssign,
    BitwiseXorAssign,
    BitwiseOrAssign,
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
            | Self::BitwiseRightShift
            | Self::BitwiseLeftShift
            | Self::LessThan
            | Self::LessEqual
            | Self::GreaterThan
            | Self::GreaterEqual
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
            | Self::SubtractAssign
            | Self::MultiplyAssign
            | Self::DivivdeAssign
            | Self::ModuloAssign
            | Self::BitwiseLeftShiftAssign
            | Self::BitwiseRightShiftAssign
            | Self::BitwiseAndAssign
            | Self::BitwiseXorAssign
            | Self::BitwiseOrAssign => Associativity::RightToLeft,
        }
    }

    fn precedence(&self) -> u32 {
        match self {
            Self::ArraySubscript
            | Self::StructEnumMemberAccess
            | Self::StructEnumMemberPointerAccess => 1,
            Self::Multiply | Self::Divide | Self::Modulo => 3,
            Self::Add | Self::Subtract => 4,
            Self::BitwiseRightShift | Self::BitwiseLeftShift => 5,
            Self::LessThan | Self::LessEqual | Self::GreaterThan | Self::GreaterEqual => 6,
            Self::Equal | Self::Different => 7,
            Self::BitwiseAnd => 8,
            Self::BitwiseXor => 9,
            Self::BitwiseOr => 10,
            Self::LogicalAnd => 11,
            Self::LogicalOr => 12,
            Self::Assign
            | Self::AddAssign
            | Self::SubtractAssign
            | Self::MultiplyAssign
            | Self::DivivdeAssign
            | Self::ModuloAssign
            | Self::BitwiseLeftShiftAssign
            | Self::BitwiseRightShiftAssign
            | Self::BitwiseAndAssign
            | Self::BitwiseXorAssign
            | Self::BitwiseOrAssign => 14,
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
    pub fn try_push_leaf(&mut self, literal: Literal) -> Result<(), String> {
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
            Self::Leaf(_) => return Err(String::new()), //TODO: write the errors
            Self::Unary(_) => return Err(String::new()),
            Self::Binary(_) => return Err(String::new()),
            Self::Ternary(_) => return Err(String::new()),
        };
        Ok(())
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
