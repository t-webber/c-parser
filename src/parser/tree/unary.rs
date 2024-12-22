use super::{node::Node, repr_option_node, Associativity, Operator};
use core::fmt;

#[derive(Debug, PartialEq)]
pub struct Unary {
    pub(super) op: UnaryOperator,
    pub(super) arg: Option<Box<Node>>,
}

#[derive(Debug, PartialEq, Eq)]
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
    /// Dereference (`*`)
    Indirection,
    /// Address-of (`&`)
    AddressOf,
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
            | Self::Indirection
            | Self::AddressOf => Associativity::RightToLeft,
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
            | Self::Indirection
            | Self::AddressOf => 2,
        }
    }
}

#[allow(clippy::min_ident_chars, clippy::wildcard_enum_match_arm)]
impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.op.associativity() == Associativity::LeftToRight {
            write!(f, "({}{})", repr_option_node(self.arg.as_ref()), self.op)
        } else {
            write!(f, "({}{})", self.op, repr_option_node(self.arg.as_ref()))
        }
    }
}

#[allow(clippy::min_ident_chars)]
impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::PostfixIncrement | Self::PrefixIncrement => "++",
                Self::PostfixDecrement | Self::PrefixDecrement => "--",
                Self::Plus => "+",
                Self::Minus => "-",
                Self::BitwiseNot => "~",
                Self::LogicalNot => "!",
                Self::Indirection => "*",
                Self::AddressOf => "&",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum SpecialUnary {
    Cast(String, Option<Box<Node>>),
    SizeOf(Option<Box<Node>>),
    AlignOf(Option<Box<Node>>),
}

impl Operator for SpecialUnary {
    fn associativity(&self) -> Associativity {
        Associativity::RightToLeft
    }

    fn precedence(&self) -> u32 {
        2
    }
}

#[allow(clippy::min_ident_chars)]
impl fmt::Display for SpecialUnary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cast(cast, arg) => write!(f, "({cast}){}", repr_option_node(arg.as_ref())),
            Self::AlignOf(arg) => write!(f, "alignof({})", repr_option_node(arg.as_ref())),
            Self::SizeOf(arg) => write!(f, "sizeof({})", repr_option_node(arg.as_ref())),
        }
    }
}
