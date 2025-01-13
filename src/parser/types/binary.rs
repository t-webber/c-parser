//! Defines the unary operator nodes.

#![allow(clippy::arbitrary_source_item_ordering)]

use core::fmt;

use super::unary::UnaryOperator;
use super::{Associativity, Ast, Operator};

/// Defines and implements the [`BinaryOperator`] type.
macro_rules! define_binary_operator {
    ($($name_left:ident $precedence_left:expr, $repr_left:expr)*; $($name_right:ident $precedence_right:expr, $repr_right:expr)*) => {
       #[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

            fn is_array_subscript(&self) -> bool {
                *self == Self::ArraySubscript
            }

            fn is_eq(&self) -> bool {
                *self == Self::Assign
            }

            fn is_star(&self) -> bool {
                *self == Self::Multiply
            }

            fn precedence(&self) -> u32 {
                match self {
                    $(Self::$name_left => $precedence_left,)*
                    $(Self::$name_right => $precedence_right,)*
                }
            }
        }

        #[expect(clippy::min_ident_chars)]
        impl fmt::Display for BinaryOperator {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", match self {
                    $(Self::$name_left => $repr_left,)*
                    $(Self::$name_right => $repr_right,)*

                })
            }
       }
    };
}

/// Binary node of the [`Ast`]
#[derive(Debug, PartialEq)]
pub struct Binary {
    /// Operator
    pub op: BinaryOperator,
    /// Argument on the left side of the operator.
    pub arg_l: Box<Ast>,
    /// Argument on the right side of the operator.
    pub arg_r: Box<Ast>,
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.op == BinaryOperator::ArraySubscript {
            if *self.arg_r == Ast::Empty {
                write!(f, "({}[])", self.arg_l)
            } else {
                write!(f, "({}[{}])", self.arg_l, self.arg_r)
            }
        } else {
            write!(f, "({} {} {})", self.arg_l, self.op, self.arg_r)
        }
    }
}

define_binary_operator!(
    // left to right
    ArraySubscript 1, "[]"
    StructEnumMemberAccess 1, "."
    StructEnumMemberPointerAccess 1, "->"
    Multiply 3, "*"
    Divide 3, "/"
    Modulo 3, "%"
    Add 4, "+"
    Subtract 4, "-"
    ShiftRight 5, ">>"
    ShiftLeft 5, "<<"
    Lt 6, "<"
    Le 6, "<="
    Gt 6, ">"
    Ge 6, ">="
    Equal 7, "=="
    Different 7, "!="
    BitwiseAnd 8, "&"
    BitwiseXor 9, "^"
    BitwiseOr 10, "|"
    LogicalAnd 11, "&&"
    LogicalOr 12, "||"
    Comma 15, ",";
    // right to left
    Assign 14, "="
    AddAssign 14, "+="
    SubAssign 14, "-="
    MulAssign 14, "*="
    DivAssign 14, "/="
    ModAssign 14, "%="
    ShiftLeftAssign 14, "<<="
    ShiftRightAssign 14, ">>="
    AndAssign 14, "&="
    XorAssign 14, "^="
    OrAssign 14, "|="
);

impl PartialEq<UnaryOperator> for BinaryOperator {
    fn eq(&self, _: &UnaryOperator) -> bool {
        false
    }
}
