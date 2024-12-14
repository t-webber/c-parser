use super::{AddArgument, Associativity, FromOperator, Node, Operator};

#[derive(Debug, PartialEq)]
pub struct Binary {
    pub(super) operator: BinaryOperator,
    pub(super) arg_l: Option<Box<Node>>,
    pub(super) arg_r: Option<Box<Node>>,
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

impl Into<Node> for Binary {
    fn into(self) -> Node {
        Node::Binary(self)
    }
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

impl Into<Node> for BinaryOperator {
    fn into(self) -> Node {
        Node::Binary(Binary::from(self))
    }
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

impl FromOperator<Binary> for BinaryOperator {
    fn from_operator(self) -> Binary {
        Binary {
            operator: self,
            arg_l: None,
            arg_r: None,
        }
    }
}
