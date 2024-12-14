use super::{AddArgument, Associativity, FromOperator, Node, Operator};

#[derive(Debug, PartialEq)]
pub struct Unary {
    pub(super) operator: UnaryOperator,
    pub(super) arg: Option<Box<Node>>,
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

impl Into<Node> for Unary {
    fn into(self) -> Node {
        Node::Unary(self)
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

impl Into<Node> for UnaryOperator {
    fn into(self) -> Node {
        Node::Unary(Unary::from(self))
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

impl FromOperator<Unary> for UnaryOperator {
    fn from_operator(self) -> Unary {
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
