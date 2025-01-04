//! Module to sort the keywords into different categories.

use super::super::types::Ast;
use super::super::types::literal::Literal;
use super::attributes::{AttributeKeyword as Attr, UnsortedAttributeKeyword as UnsortedAttr};
use super::control_flow::keyword::ControlFlowKeyword as CtrlFlow;
use super::functions::FunctionKeyword as Func;
use crate::lexer::api::Keyword;
use crate::parser::types::braced_blocks::BracedBlock;

/// Context information needed to decide the type of a keyword
#[derive(PartialEq, Eq)]
pub enum Context {
    /// Inside a `case`
    ///
    /// # Examples
    ///
    /// `default` keyword is a control flow in a `case`, but an attribute
    /// otherwise.
    Case,
    /// No useful information
    None,
    /// Following a `typedef`
    ///
    /// `struct`, `enum` and `union` are control flows if preceded by `typedef`
    /// and not attributes.
    Typedef,
}

impl Context {
    /// Checks if the context is case
    pub fn is_case(&self) -> bool {
        *self == Self::Case
    }

    /// Checks if the context is typedef
    pub fn is_typedef(&self) -> bool {
        *self == Self::Typedef
    }
}

impl From<&Ast> for Context {
    fn from(node: &Ast) -> Self {
        match node {
            Ast::ControlFlow(ctrl) if !ctrl.is_full() => match ctrl.get_keyword() {
                CtrlFlow::Case => Self::Case,
                CtrlFlow::Typedef => Self::Typedef,
                CtrlFlow::Break
                | CtrlFlow::Continue
                | CtrlFlow::Default
                | CtrlFlow::Do
                | CtrlFlow::Else
                | CtrlFlow::Enum
                | CtrlFlow::For
                | CtrlFlow::Goto
                | CtrlFlow::If
                | CtrlFlow::Return
                | CtrlFlow::Struct
                | CtrlFlow::Switch
                | CtrlFlow::Union
                | CtrlFlow::While => Self::None,
            },
            Ast::Empty
            | Ast::Leaf(_)
            | Ast::Unary(_)
            | Ast::Binary(_)
            | Ast::Ternary(_)
            | Ast::ParensBlock(_)
            | Ast::ControlFlow(_)
            | Ast::FunctionCall(_)
            | Ast::ListInitialiser(_)
            | Ast::BracedBlock(BracedBlock { full: true, .. }) => Self::None,
            Ast::FunctionArgsBuild(elts) | Ast::BracedBlock(BracedBlock { elts, full: false }) => {
                elts.last().map_or(Self::None, Self::from)
            }
        }
    }
}

/// Enum for the different types of keywords that exist.
pub enum KeywordParsing {
    /// Attribute keyword: applied on a variable
    Attr(Attr),
    /// Control flow keyword: `return`, `for`, `goto`, `case`, ...
    CtrlFlow(CtrlFlow),
    /// Boolean constant `false`
    False,
    /// Function keyword: `sizeof`, `static_assert`, ...
    Func(Func),
    /// `NULL` constant
    Nullptr,
    /// Boolean constant `true`
    True,
}

impl From<(Keyword, Context)> for KeywordParsing {
    fn from((keyword, context): (Keyword, Context)) -> Self {
        match keyword {
            // constants
            Keyword::True => Self::True,
            Keyword::False => Self::False,
            Keyword::Null | Keyword::Nullptr => Self::Nullptr,
            // functions
            Keyword::Sizeof => Self::Func(Func::Sizeof),
            Keyword::Typeof => Self::Func(Func::Typeof),
            Keyword::TypeofUnqual => Self::Func(Func::TypeofUnqual),
            Keyword::Alignof | Keyword::UAlignof => Self::Func(Func::Alignof),
            Keyword::StaticAssert | Keyword::UStaticAssert => Self::Func(Func::StaticAssert),
            // control flows
            Keyword::Do => Self::CtrlFlow(CtrlFlow::Do),
            Keyword::If => Self::CtrlFlow(CtrlFlow::If),
            Keyword::For => Self::CtrlFlow(CtrlFlow::For),
            Keyword::Case => Self::CtrlFlow(CtrlFlow::Case),
            Keyword::Else => Self::CtrlFlow(CtrlFlow::Else),
            Keyword::Goto => Self::CtrlFlow(CtrlFlow::Goto),
            Keyword::While => Self::CtrlFlow(CtrlFlow::While),
            Keyword::Break => Self::CtrlFlow(CtrlFlow::Break),
            Keyword::Return => Self::CtrlFlow(CtrlFlow::Return),
            Keyword::Switch => Self::CtrlFlow(CtrlFlow::Switch),
            Keyword::Continue => Self::CtrlFlow(CtrlFlow::Continue),
            Keyword::Default if context.is_case() => Self::CtrlFlow(CtrlFlow::Default),
            // user-defined types
            Keyword::Enum if context.is_typedef() => Self::CtrlFlow(CtrlFlow::Enum),
            Keyword::Union if context.is_typedef() => Self::CtrlFlow(CtrlFlow::Union),
            Keyword::Struct if context.is_typedef() => Self::CtrlFlow(CtrlFlow::Struct),
            Keyword::Enum => Self::Attr(Attr::from(UnsortedAttr::Enum)),
            Keyword::Union => Self::Attr(Attr::from(UnsortedAttr::Union)),
            Keyword::Struct => Self::Attr(Attr::from(UnsortedAttr::Struct)),
            Keyword::Typedef => Self::CtrlFlow(CtrlFlow::Typedef),
            // attributes
            Keyword::Int => Self::Attr(Attr::from(UnsortedAttr::Int)),
            Keyword::Long => Self::Attr(Attr::from(UnsortedAttr::Long)),
            Keyword::Auto => Self::Attr(Attr::from(UnsortedAttr::Auto)),
            Keyword::Char => Self::Attr(Attr::from(UnsortedAttr::Char)),
            Keyword::Void => Self::Attr(Attr::from(UnsortedAttr::Void)),
            Keyword::Short => Self::Attr(Attr::from(UnsortedAttr::Short)),
            Keyword::Float => Self::Attr(Attr::from(UnsortedAttr::Float)),
            Keyword::Const => Self::Attr(Attr::from(UnsortedAttr::Const)),
            Keyword::Inline => Self::Attr(Attr::from(UnsortedAttr::Inline)),
            Keyword::Double => Self::Attr(Attr::from(UnsortedAttr::Double)),
            Keyword::Signed => Self::Attr(Attr::from(UnsortedAttr::Signed)),
            Keyword::Extern => Self::Attr(Attr::from(UnsortedAttr::Extern)),
            Keyword::Static => Self::Attr(Attr::from(UnsortedAttr::Static)),
            Keyword::UAtomic => Self::Attr(Attr::from(UnsortedAttr::UAtomic)),
            Keyword::UBigInt => Self::Attr(Attr::from(UnsortedAttr::UBigInt)),
            Keyword::Default => Self::Attr(Attr::from(UnsortedAttr::Default)),
            Keyword::Unsigned => Self::Attr(Attr::from(UnsortedAttr::Unsigned)),
            Keyword::Register => Self::Attr(Attr::from(UnsortedAttr::Register)),
            Keyword::Restrict => Self::Attr(Attr::from(UnsortedAttr::Restrict)),
            Keyword::Volatile => Self::Attr(Attr::from(UnsortedAttr::Volatile)),
            Keyword::UComplex => Self::Attr(Attr::from(UnsortedAttr::UComplex)),
            Keyword::UGeneric => Self::Attr(Attr::from(UnsortedAttr::UGeneric)),
            Keyword::UNoreturn => Self::Attr(Attr::from(UnsortedAttr::UNoreturn)),
            Keyword::Constexpr => Self::Attr(Attr::from(UnsortedAttr::Constexpr)),
            Keyword::UDecimal64 => Self::Attr(Attr::from(UnsortedAttr::UDecimal64)),
            Keyword::UImaginary => Self::Attr(Attr::from(UnsortedAttr::UImaginary)),
            Keyword::UDecimal32 => Self::Attr(Attr::from(UnsortedAttr::UDecimal32)),
            Keyword::UDecimal128 => Self::Attr(Attr::from(UnsortedAttr::UDecimal128)),
            Keyword::Alignas | Keyword::UAlignas => Self::Attr(Attr::from(UnsortedAttr::Alignas)),
            Keyword::Bool | Keyword::UBool => Self::Attr(Attr::from(UnsortedAttr::Bool)),
            Keyword::ThreadLocal | Keyword::UThreadLocal => {
                Self::Attr(Attr::from(UnsortedAttr::ThreadLocal))
            }
        }
    }
}

impl PushInNode for KeywordParsing {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        match self {
            Self::Func(func) => func.push_in_node(node),
            Self::Attr(attr) => attr.push_in_node(node),
            Self::CtrlFlow(ctrl) => ctrl.push_in_node(node),
            Self::Nullptr => node.push_block_as_leaf(Ast::Leaf(Literal::Nullptr)),
            Self::True => node.push_block_as_leaf(Ast::Leaf(Literal::ConstantBool(true))),
            Self::False => node.push_block_as_leaf(Ast::Leaf(Literal::ConstantBool(false))),
        }
    }
}

/// Trait to push a keyword inside a current [`Ast`].
pub trait PushInNode {
    /// Function to push a keyword inside a current [`Ast`].
    fn push_in_node(self, node: &mut Ast) -> Result<(), String>;
}
