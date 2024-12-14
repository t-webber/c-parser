use core::fmt;
use core::mem;

use crate::lexer::api::types::Number;

pub trait Operator: fmt::Debug {
    fn precedence(&self) -> u32;
    fn associativity(&self) -> Associativity;
}

trait AddArgument: Into<Node> {
    fn add_argument(&mut self, arg: Node) -> bool;
}

trait FromOperator<T: AddArgument> {
    fn from_operator(self) -> T;
}

impl Into<Node> for Unary {
    fn into(self) -> Node {
        Node::Unary(self)
    }
}

impl FromOperator<Unary> for UnaryOperator {
    fn from_operator(self) -> Unary {
        Unary {
            operator: self,
            arg: None,
        }
    }
}
impl FromOperator<Binary> for BinaryOperator {
    fn from_operator(self) -> Binary {
        Binary {
            operator: self,
            arg_l: None,
            arg_r: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Associativity {
    LeftToRight,
    RightToLeft,
}

#[derive(Debug, PartialEq)]
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

impl Into<Node> for Binary {
    fn into(self) -> Node {
        Node::Binary(self)
    }
}

impl AddArgument for Binary {
    fn add_argument(&mut self, arg: Node) -> bool {
        if let Binary {
            arg_l: op_arg @ None,
            ..
        }
        | Binary {
            arg_r: op_arg @ None,
            ..
        } = self
        {
            *op_arg = Some(Box::from(arg));
            true
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct CompoundLiteral {
    args: Vec<Node>,
    operator: CompoundLiteralOperator,
    type_: String,
}

#[derive(Debug, PartialEq)]
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
    name: String,
    operator: FunctionOperator,
    args: Vec<Node>,
}

#[derive(Debug, PartialEq)]
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
pub enum Literal {
    #[default]
    Empty,
    String(String),
    Variable(String),
    Char(char),
    Str(String),
    Number(Number),
}

#[derive(Debug, Default, PartialEq)]
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
    /// This functions returns Err if two many arguments were provided,
    /// like in the expression: `a+b c`.
    pub fn push_node_as_leaf(&mut self, node: Node) -> Result<(), &'static str> {
        match self {
            Self::Empty => *self = node,
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
            }) => *last = Some(Box::new(node)),
            // push in Vec<Node>
            Self::Block(vec) => vec.push(node),
            // todo
            Self::Vec(_) | Self::FunctionCall(_) | Self::CompoundLiteral(_) => todo!(),
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

    pub fn take_last_leaf(&mut self) -> Option<Literal> {
        match self {
            node @ Self::Leaf(_) => {
                if let Self::Leaf(leaf) = mem::replace(node, Node::Empty) {
                    Some(leaf)
                } else {
                    panic!("never happens because old is leaf...")
                }
            }
            Self::Binary(
                Binary {
                    arg_r: Some(child), ..
                }
                | Binary {
                    arg_l: Some(child), ..
                },
            )
            | Self::Ternary(
                Ternary {
                    failure: Some(child),
                    ..
                }
                | Ternary {
                    success: Some(child),
                    ..
                }
                | Ternary {
                    condition: Some(child),
                    ..
                },
            )
            | Self::Unary(Unary {
                arg: Some(child), ..
            }) => child.take_last_leaf(),
            Self::Block(vec) => {
                if let Some(last) = vec.last_mut() {
                    last.take_last_leaf()
                } else {
                    None
                }
            }
            // todo
            Self::Vec(_) | Self::FunctionCall(_) | Self::CompoundLiteral(_) => todo!(),
            // Errors
            _ => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        *self == Node::Empty
    }

    pub fn push_op<U: AddArgument, T: Operator + FromOperator<U> + Into<Node>>(
        &mut self,
        operator: T,
    ) -> Result<(), &'static str> {
        //TODO: this doesn't work for cast, sizeof and alignof
        match operator.associativity() {
            Associativity::LeftToRight => match self.take_last_leaf() {
                None => {
                    // This is error is never printed, because the only left-to-right operators are postfix increments, and those are catched.
                    return Err(
                        "Found left-to-right unary operator, without having a leaf before.",
                    );
                }
                Some(leaf) => {
                    let mut new_leaf = operator.from_operator();
                    new_leaf.add_argument(Node::Leaf(leaf));
                    self.push_node_as_leaf(new_leaf.into());
                }
            },
            Associativity::RightToLeft => {
                if let Err(_) = self.push_node_as_leaf(operator.into()) {
                    // Example: `int c = a+b!;`
                    return Err(
                        "Found right-to-left unary operator, within a context not waiting for leaf.",
                    );
                }
            }
        }
        Ok(())
    }
}

impl Into<Node> for UnaryOperator {
    fn into(self) -> Node {
        Node::Unary(Unary::from(self))
    }
}
impl Into<Node> for BinaryOperator {
    fn into(self) -> Node {
        Node::Binary(Binary::from(self))
    }
}

#[derive(Debug, PartialEq)]
pub struct Ternary {
    operator: TernaryOperator,
    condition: Option<Box<Node>>,
    success: Option<Box<Node>>,
    failure: Option<Box<Node>>,
}

#[derive(Debug, PartialEq)]
pub struct TernaryOperator;

impl Operator for TernaryOperator {
    fn associativity(&self) -> Associativity {
        Associativity::RightToLeft
    }

    fn precedence(&self) -> u32 {
        13
    }
}

#[derive(Debug, PartialEq)]
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

impl AddArgument for Unary {
    fn add_argument(&mut self, arg: Node) -> bool {
        if let Unary {
            arg: old_arg @ None,
            ..
        } = self
        {
            *old_arg = Some(Box::new(arg));
            true
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    // Defined,
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
            Self::PostfixIncrement | Self::PostfixDecrement => Associativity::LeftToRight,
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
