//! Module to sort the keywords into different categories.

use super::attributes::{AttributeKeyword as Attr, UnsortedAttributeKeyword as UnsortedAttr};
use super::control_flow::keyword::ControlFlowKeyword as CtrlFlow;
use super::control_flow::node::ControlFlowNode;
use super::control_flow::pushable::PushableKeyword;
use super::control_flow::traits::ControlFlow as _;
use super::functions::FunctionKeyword as Func;
use crate::lexer::api::Keyword;
use crate::parser::literal::Literal;
use crate::parser::modifiers::push::Push as _;
use crate::parser::symbols::api::BracedBlock;
use crate::parser::tree::Ast;

/// Context information needed to decide the type of a keyword
#[derive(PartialEq, Eq, Default, Debug)]
pub enum Context {
    /// Inside an `if` block
    IfNoElse,
    /// No context found
    #[default]
    None,
    /// Inside a `switch`
    ///
    /// # Examples
    ///
    /// `default` keyword is a control flow in a `switch`, but an attribute
    /// otherwise.
    Switch,
    /// Following a `typedef`
    ///
    /// `struct`, `enum` and `union` are control flows if preceded by `typedef`
    /// and not attributes.
    Typedef,
}

impl Context {
    /// Concatenates [`Context`] with the context of a child.
    ///
    /// The child is has more priority so replaces the father's context, except
    /// if no context were found in the child.
    fn concat(self, other: Self) -> Self {
        if other == Self::None { self } else { other }
    }
}

impl From<&Ast> for Context {
    fn from(node: &Ast) -> Self {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::custom_print(&format!("Getting context of {node}"));
        match node {
            Ast::ControlFlow(ctrl) => {
                let ctx = if ctrl.is_condition() {
                    if let ControlFlowNode::Condition(condition) = ctrl
                        && condition.no_else()
                    {
                        Self::IfNoElse
                    } else {
                        Self::None
                    }
                } else {
                    // typedef ignored because struct after typedef is not necessarily a struct
                    // definition, it could be `typedef struct A B;`
                    Self::None
                };
                ctx.concat(Self::from(ctrl.as_ast()))
            }

            Ast::Empty
            | Ast::Cast(_)
            | Ast::Leaf(_)
            | Ast::Unary(_)
            | Ast::Binary(_)
            | Ast::Ternary(_)
            | Ast::Variable(_)
            | Ast::ParensBlock(_)
            | Ast::FunctionCall(_)
            | Ast::ListInitialiser(_)
            | Ast::BracedBlock(BracedBlock { full: true, .. }) => Self::default(),
            Ast::FunctionArgsBuild(elts) | Ast::BracedBlock(BracedBlock { elts, full: false }) =>
                elts.last().map_or_else(Self::default, Self::from),
        }
    }
}

impl<T> From<Option<T>> for Context
where
    Self: From<T>,
{
    fn from(value: Option<T>) -> Self {
        value.map_or(Self::None, |val| Self::from(val))
    }
}

/// Enum for the different types of keywords that exist.
#[derive(Debug)]
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
    Null,
    /// Keywords that need to be pushed in an existing control flow block
    Pushable(PushableKeyword),
    /// Boolean constant `true`
    True,
}

impl PushInNode for KeywordParsing {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        match self {
            Self::Func(func) => func.push_in_node(node),
            Self::Attr(attr) => attr.push_in_node(node),
            Self::CtrlFlow(ctrl) => ctrl.push_in_node(node),
            Self::Null => node.push_block_as_leaf(Ast::Leaf(Literal::Null)),
            Self::True => node.push_block_as_leaf(Ast::Leaf(Literal::ConstantBool(true))),
            Self::False => node.push_block_as_leaf(Ast::Leaf(Literal::ConstantBool(false))),
            Self::Pushable(pushable) => pushable.push_in_node(node),
        }
    }
}

impl TryFrom<(Keyword, Context)> for KeywordParsing {
    type Error = String;
    fn try_from((keyword, context): (Keyword, Context)) -> Result<Self, Self::Error> {
        Ok(match keyword {
            // constants
            Keyword::True => Self::True,
            Keyword::False => Self::False,
            Keyword::Null => Self::Null,
            // functions
            Keyword::Sizeof => Self::Func(Func::Sizeof),
            Keyword::Alignof | Keyword::UAlignof => Self::Func(Func::Alignof),
            Keyword::StaticAssert | Keyword::UStaticAssert => Self::Func(Func::StaticAssert),
            // pushable
            Keyword::Case => Self::CtrlFlow(CtrlFlow::Case),
            Keyword::Default if context == Context::Switch => Self::CtrlFlow(CtrlFlow::Default),
            Keyword::Else if context == Context::IfNoElse => Self::Pushable(PushableKeyword::Else),
            Keyword::Do => Self::CtrlFlow(CtrlFlow::Do),
            Keyword::For => Self::CtrlFlow(CtrlFlow::For),
            Keyword::Goto => Self::CtrlFlow(CtrlFlow::Goto),
            Keyword::While => Self::CtrlFlow(CtrlFlow::While),
            Keyword::Break => Self::CtrlFlow(CtrlFlow::Break),
            Keyword::Return => Self::CtrlFlow(CtrlFlow::Return),
            Keyword::Switch => Self::CtrlFlow(CtrlFlow::Switch),
            Keyword::Continue => Self::CtrlFlow(CtrlFlow::Continue),
            // conditionals
            Keyword::If => Self::CtrlFlow(CtrlFlow::If),
            Keyword::Else => return Err("Found nomad `else` without `if`.".to_owned()),
            // user-defined types
            Keyword::Enum if context == Context::Typedef => Self::CtrlFlow(CtrlFlow::Enum),
            Keyword::Union if context == Context::Typedef => Self::CtrlFlow(CtrlFlow::Union),
            Keyword::Struct if context == Context::Typedef => Self::CtrlFlow(CtrlFlow::Struct),
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
            Keyword::Bool | Keyword::UBool => Self::Attr(Attr::from(UnsortedAttr::Bool)),
            Keyword::Alignas | Keyword::UAlignas => Self::Attr(Attr::from(UnsortedAttr::Alignas)),
            Keyword::ThreadLocal | Keyword::UThreadLocal =>
                Self::Attr(Attr::from(UnsortedAttr::ThreadLocal)),
        })
    }
}

/// Trait to push a keyword inside a current [`Ast`].
pub trait PushInNode {
    /// Function to push a keyword inside a current [`Ast`].
    fn push_in_node(self, node: &mut Ast) -> Result<(), String>;
}
