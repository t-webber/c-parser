#![expect(clippy::arbitrary_source_item_ordering, reason = "macro used")]

use core::fmt;

use super::super::tree::binary::Binary;
use super::super::tree::blocks::Block;
use super::super::tree::literal::{Literal, Variable};
use super::super::tree::unary::Unary;
use super::super::tree::{FunctionCall, ListInitialiser, Ternary};
use super::Ast;
use super::types::PushInNode;
use crate::lexer::api::Keyword;

macro_rules! define_attribute_keywords {
    ($($name:ident: $($variant:ident)*,)*) => {

        #[derive(Debug, PartialEq, Eq)]
        pub enum AttributeKeyword {
            $($name($name),)*
        }

        impl From<UnsortedAttributeKeyword> for AttributeKeyword {
            fn from(value: UnsortedAttributeKeyword) -> Self {
                match value {
                    $($(UnsortedAttributeKeyword::$variant => Self::$name($name::$variant),)*)*
                }
            }
        }

        pub enum UnsortedAttributeKeyword {
            $($($variant,)*)*
        }

        $(
            #[derive(Debug, PartialEq, Eq)]
            pub enum $name {
                $($variant,)*
            }
        )*

        #[expect(clippy::min_ident_chars)]
        impl fmt::Display for AttributeKeyword {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $($(Self::$name($name::$variant) => Keyword::$variant.fmt(f),)*)*
                }
            }
        }

    };
}

define_attribute_keywords!(
    BasicDataType: Bool Char Double Float Int UComplex UDecimal128 UDecimal32 UDecimal64 UImaginary UBigInt Void,
    Modifiers: Signed Unsigned Long Short,
    Storage: Auto ThreadLocal Extern Static Register,
    Qualifiers: Const Constexpr Volatile Default,
    // UserDefinedTypes: Typedef Struct Union Enum,
    SpecialAttributes: UAtomic Alignas Inline Restrict UGeneric UNoreturn,
);

impl AttributeKeyword {
    fn into_node(self) -> Ast {
        Ast::Leaf(Literal::Variable(Variable::from(self)))
    }
}

impl PushInNode for AttributeKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        match node {
            Ast::Empty => *node = self.into_node(),
            Ast::Leaf(Literal::Variable(var)) => var.push_keyword(self),
            Ast::ParensBlock(_) | Ast::Leaf(_) => {
                return Err(format!(
                    "invalid attribute. Attribute keywords can only be applied to variables, but found {node}"
                ));
            }
            Ast::Unary(Unary { arg, .. })
            | Ast::Binary(Binary { arg_r: arg, .. })
            | Ast::Ternary(
                Ternary {
                    failure: Some(arg), ..
                }
                | Ternary { success: arg, .. },
            ) => return self.push_in_node(arg),
            Ast::ControlFlow(_)
            | Ast::FunctionCall(FunctionCall { full: true, .. })
            | Ast::ListInitialiser(ListInitialiser { full: true, .. }) => {
                return Err(format!(
                    "Attribute {self} can only be placed before variables, but found {node}"
                ));
            }
            Ast::ListInitialiser(ListInitialiser { elts, .. })
            | Ast::FunctionCall(FunctionCall { args: elts, .. })
            | Ast::Block(Block { elts, .. }) => match elts.last_mut() {
                Some(last) => return self.push_in_node(last),
                None => elts.push(self.into_node()),
            },
        }
        Ok(())
    }
}
