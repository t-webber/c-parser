use super::{repr_option_node, AddArgument, Associativity, Node, Operator, TakeOperator};
use core::fmt;

#[derive(Debug, PartialEq)]
pub struct Unary {
    pub(super) operator: UnaryOperator,
    pub(super) arg: Option<Box<Node>>,
}

impl AddArgument for Unary {
    fn add_argument(&mut self, arg: Node) -> bool {
        if let Self {
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

impl From<Unary> for Node {
    fn from(val: Unary) -> Self {
        Self::Unary(val)
    }
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
    Cast(String),
    /// Dereference (`*`)
    Indirection,
    /// Address-of (`&`)
    AddressOf,
    SizeOf,
    AlignOf,
}

impl From<UnaryOperator> for Node {
    fn from(op: UnaryOperator) -> Self {
        Self::Unary(Unary::from(op))
    }
}

impl From<UnaryOperator> for Unary {
    fn from(operator: UnaryOperator) -> Self {
        Self {
            operator,
            arg: None,
        }
    }
}

impl TakeOperator<Unary> for UnaryOperator {
    fn take_operator(self) -> Unary {
        Unary {
            operator: self,
            arg: None,
        }
    }
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

#[allow(clippy::min_ident_chars, clippy::wildcard_enum_match_arm)]
impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arg = repr_option_node(self.arg.as_ref());
        match self.operator {
            UnaryOperator::Cast(ref cast) => write!(f, "({cast}){arg}"),
            UnaryOperator::AlignOf => write!(f, "alignof({arg})"),
            UnaryOperator::SizeOf => write!(f, "sizeof({arg})"),
            ref op => {
                if op.associativity() == Associativity::LeftToRight {
                    write!(f, "({arg}{op})")
                } else {
                    write!(f, "({op}{arg})")
                }
            }
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
                Self::Cast(_) | Self::SizeOf | Self::AlignOf =>
                    panic!("This is not mean't to happen: never call display on cast, as it is handled from unary"),
            }
        )
    }
}
