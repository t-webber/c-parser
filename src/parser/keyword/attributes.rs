//! Implements the function keywords

use super::control_flow::traits::ControlFlow as _;
use super::sort::PushInNode;
use crate::lexer::api::Keyword;
use crate::parser::api::AstValue;
use crate::parser::modifiers::push::Push as _;
use crate::parser::operators::api::{Binary, Ternary, Unary};
use crate::parser::symbols::api::{BracedBlock, ListInitialiser};
use crate::parser::tree::Ast;
use crate::parser::variable::Variable;
use crate::utils::display;

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

        display!(AttributeKeyword, self, f,
                match self {
                    $($(Self::$name($name::$variant) => Keyword::$variant.fmt(f),)*)*
                }
                );

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
        AstValue::Variable(Variable::from(attr)).into()
    }
}

impl PushInNode for AttributeKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_in_node(&self, "attr keyword", node);
        match &mut node.value {
            AstValue::Empty => *node = Ast::from(self),
            AstValue::Cast(_) =>
                return Err("Attribute found after a cast, but not allowed".to_owned()),
            AstValue::Variable(var) => var.push_keyword(self)?,
            AstValue::ParensBlock(_) | AstValue::Leaf(_) => {
                return Err(format!(
                    "invalid attribute. Attribute keywords can only be applied to variables, but found {node}"
                ));
            }
            AstValue::Unary(Unary { arg, .. })
            | AstValue::Binary(Binary { arg_r: arg, .. })
            | AstValue::Ternary(
                Ternary { failure: Some(arg), .. } | Ternary { success: arg, .. },
            ) => return self.push_in_node(arg),
            AstValue::ControlFlow(ctrl) if !ctrl.is_full() => {
                ctrl.push_block_as_leaf(Ast::from(self))?;
            }
            AstValue::ControlFlow(_) => {
                return Err("Attribute found after full control flow, but not allowed.".to_owned());
            }
            AstValue::FunctionCall(_)
            | AstValue::ListInitialiser(ListInitialiser { full: true, .. }) => {
                return Err(format!(
                    "Attribute {self} can only be placed before variables, but was found after {node}. Did you forget a ';' ?"
                ));
            }
            AstValue::FunctionArgsBuild(elts)
            | AstValue::ListInitialiser(ListInitialiser { elts, .. })
            | AstValue::BracedBlock(BracedBlock { elts, .. }) => match elts.last_mut() {
                Some(last) => return self.push_in_node(last),
                None => elts.push(Ast::from(self)),
            },
        }
        Ok(())
    }
}
