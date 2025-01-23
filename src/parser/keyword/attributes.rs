//! Implements the function keywords

#![expect(clippy::arbitrary_source_item_ordering, reason = "macro used")]

use core::fmt;

use super::Ast;
use super::control_flow::traits::ControlFlow as _;
use super::sort::PushInNode;
use crate::lexer::api::Keyword;
use crate::parser::modifiers::push::Push as _;
use crate::parser::types::ListInitialiser;
use crate::parser::types::binary::Binary;
use crate::parser::types::braced_blocks::BracedBlock;
use crate::parser::types::ternary::Ternary;
use crate::parser::types::unary::Unary;
use crate::parser::types::variable::Variable;

/// Defines the attribute keywords.
macro_rules! define_attribute_keywords {
    ($($name:ident: $($variant:ident)*,)*) => {

        #[derive(Debug, PartialEq, Eq)]
        pub enum AttributeKeyword {
            $($name($name),)*
        }

        impl From<UnsortedAttributeKeyword> for AttributeKeyword {
            #[coverage(off)]
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
        #[coverage(off)]
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
    UserDefinedTypes: Struct Union Enum,
    SpecialAttributes: UAtomic Alignas Inline Restrict UGeneric UNoreturn,
);

impl From<AttributeKeyword> for Ast {
    fn from(attr: AttributeKeyword) -> Self {
        Self::Variable(Variable::from(attr))
    }
}

impl PushInNode for AttributeKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_in_node(&self, "attr keyword", node);
        match node {
            Ast::Empty => *node = Ast::from(self),
            Ast::Cast(_) => return Err("Attribute found after a cast, but not allowed".to_owned()),
            Ast::Variable(var) => var.push_keyword(self)?,
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
            Ast::ControlFlow(ctrl) if !ctrl.is_full() => {
                ctrl.push_block_as_leaf(Ast::from(self))?;
            }
            Ast::ControlFlow(_) => {
                return Err("Attribute found after full control flow, but not allowed.".to_owned());
            }
            Ast::FunctionCall(_) | Ast::ListInitialiser(ListInitialiser { full: true, .. }) => {
                return Err(format!(
                    "Attribute {self} can only be placed before variables, but was found after {node}. Did you forget a ';' ?"
                ));
            }
            Ast::FunctionArgsBuild(elts)
            | Ast::ListInitialiser(ListInitialiser { elts, .. })
            | Ast::BracedBlock(BracedBlock { elts, .. }) => match elts.last_mut() {
                Some(last) => return self.push_in_node(last),
                None => elts.push(Ast::from(self)),
            },
        }
        Ok(())
    }
}
