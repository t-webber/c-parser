use super::{AddArgument, Associativity, Node, Operator, TakeOperator};
use crate::parser::tree::repr_option_node;
use core::fmt;

#[derive(Debug, PartialEq)]
pub struct Binary {
    pub(super) operator: BinaryOperator,
    pub(super) arg_l: Option<Box<Node>>,
    pub(super) arg_r: Option<Box<Node>>,
}

impl AddArgument for Binary {
    fn add_argument(&mut self, arg: Node) -> bool {
        if let Self {
            arg_l: op_arg @ None,
            ..
        }
        | Self {
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

impl From<Binary> for Node {
    fn from(val: Binary) -> Self {
        Self::Binary(val)
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

#[allow(clippy::min_ident_chars)]
impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            repr_option_node(self.arg_l.as_ref()),
            self.operator,
            repr_option_node(self.arg_r.as_ref())
        )
    }
}

macro_rules! define_binary_operator {
    ($($name_left:ident $precedence_left:expr, $repr_left:expr)*; $($name_right:ident $precedence_right:expr, $repr_right:expr)*) => {
       #[derive(Debug, PartialEq, Eq)]
       pub enum BinaryOperator {
         $($name_left,)*
         $($name_right,)*
       }

       impl Operator for BinaryOperator {
            fn associativity(&self) -> Associativity {
                match self {
                    $(Self::$name_left => Associativity::LeftToRight,)*
                    $(Self::$name_right => Associativity::RightToLeft,)*
                }
            }

            fn precedence(&self) -> u32 {
                match self {
                    $(Self::$name_left => $precedence_left,)*
                    $(Self::$name_right => $precedence_right,)*
                }
            }
        }

        #[allow(clippy::min_ident_chars)]
        impl fmt::Display for BinaryOperator {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", match self {
                    $(Self::$name_left => $repr_left,)*
                    $(Self::$name_right => $repr_right,)*

                })
            }
       }
    };
}

impl From<BinaryOperator> for Node {
    fn from(value: BinaryOperator) -> Self {
        Self::Binary(Binary::from(value))
    }
}

define_binary_operator!(
    // left to right
    ArraySubscript 1, "array subscript"
    StructEnumMemberAccess 1, "struct enum member access"
    StructEnumMemberPointerAccess 1, "struct enum member pointer access"
    Multiply 3, "multiply"
    Divide 3, "divide"
    Modulo 3, "modulo"
    Add 4, "add"
    Subtract 4, "subtract"
    RightShift 5, "right shift"
    LeftShift 5, "left shift"
    Lt 6, "lt"
    Le 6, "le"
    Gt 6, "gt"
    Ge 6, "ge"
    Equal 7, "equal"
    Different 7, "different"
    BitwiseAnd 8, "bitwise and"
    BitwiseXor 9, "bitwise xor"
    BitwiseOr 10, "bitwise or"
    LogicalAnd 11, "logical and"
    LogicalOr 12, "logical or"
    Comma 15, "comma";
    // right to left
    Assign 14, "assign"
    AddAssign 14, "add assign"
    SubAssign 14, "sub assign"
    MulAssign 14, "mul assign"
    DivAssign 14, "div assign"
    ModAssign 14, "mod assign"
    LeftShiftAssign 14, "left shift assign"
    RightShiftAssign 14, "right shift assign"
    AndAssign 14, "and assign"
    XorAssign 14, "xor assign"
    OrAssign 14, "or assign");

impl TakeOperator<Binary> for BinaryOperator {
    fn take_operator(self) -> Binary {
        Binary {
            operator: self,
            arg_l: None,
            arg_r: None,
        }
    }
}
