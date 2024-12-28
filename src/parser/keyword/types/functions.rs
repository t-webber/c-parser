use core::fmt;

use super::super::Ast;
use super::PushInNode;
use crate::parser::tree::Literal;

pub enum FunctionKeyword {
    Alignof,
    Sizeof,
    StaticAssert,
    Typeof,
    TypeofUnqual,
    UAlignof,
    UThreadLocal,
}

impl PushInNode for FunctionKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        node.push_block_as_leaf(Ast::Leaf(Literal::Variable(self.to_string())))
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for FunctionKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alignof => "alignof".fmt(f),
            Self::Sizeof => "sizeof".fmt(f),
            Self::StaticAssert => "static_assert".fmt(f),
            Self::Typeof => "typeof".fmt(f),
            Self::TypeofUnqual => "typeof_unqual".fmt(f),
            Self::UAlignof => "u_alignof".fmt(f),
            Self::UThreadLocal => "u_thread_local".fmt(f),
        }
    }
}
