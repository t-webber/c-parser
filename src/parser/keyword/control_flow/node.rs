//! Defines the control flow nodes.

use core::fmt;

use super::keyword::ControlFlowKeyword;
use super::traits::ControlFlow;
use super::types::case::AstColonAstCtrl;
use super::types::colon_ast::{ColonAstCtrl, ColonAstKeyword};
use super::types::conditional::ConditionCtrl;
use super::types::do_while::DoWhileCtrl;
use super::types::goto::ColonIdentCtrl;
use super::types::ident_block::{IdentBlockCtrl, IdentBlockKeyword};
use super::types::parens_block::{ParensBlockCtrl, ParensBlockKeyword};
use super::types::return_ctrl::ReturnCtrl;
use super::types::semi_colon::{SemiColonCtrl, SemiColonKeyword};
use super::types::typedef::TypedefCtrl;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::types::Ast;
use crate::parser::types::braced_blocks::BracedBlock;

/// Derives a method of a [`ControlFlow`] trait for [`ControlFlowNode`] by
/// directly applying the method on the corresponding control flow.
macro_rules! derive_method{
    ($val:ident, $method:ident $(,$arg:ident)*) => {
        match $val {
            Self::Ast(ctrl) => ctrl.$method($($arg,)*),
            Self::AstColonAst(ctrl) => ctrl.$method($($arg,)*),
            Self::ColonAst(ctrl) => ctrl.$method($($arg,)*),
            Self::ColonIdent(ctrl) => ctrl.$method($($arg,)*),
            Self::Condition(ctrl) => ctrl.$method($($arg,)*),
            Self::DoWhile(ctrl) => ctrl.$method($($arg,)*),
            Self::IdentBlock(ctrl) => ctrl.$method($($arg,)*),
            Self::ParensBlock(ctrl) => ctrl.$method($($arg,)*),
            Self::SemiColon(ctrl) => ctrl.$method($($arg,)*),
            Self::Typedef(ctrl) => ctrl.$method($($arg,)*),
        }
    };
}

/// Node representation of a control flow.
#[derive(Debug, PartialEq)]
pub enum ControlFlowNode {
    /// Keyword expects a node: `return 3+4`
    Ast(ReturnCtrl),
    /// Keyword expects a colon and a node: `goto: label` or `default`
    AstColonAst(AstColonAstCtrl),
    /// Keyword expects a node and then a colon: `case 2:`
    ColonAst(ColonAstCtrl),
    /// Keywords expected a colon then a identifier: `goto: label`
    ColonIdent(ColonIdentCtrl),
    /// `if` keyword
    Condition(ConditionCtrl),
    /// `do` keyword
    DoWhile(DoWhileCtrl),
    /// Keyword expects an identifier and a braced block: `struct Blob {}`
    IdentBlock(IdentBlockCtrl),
    /// Keyword expects a parens and a braced block: `switch (cond) {};`
    ParensBlock(ParensBlockCtrl),
    /// Keyword expects a semicolon: `break;`
    SemiColon(SemiColonCtrl),
    /// Typedef control flow: `typedef struct`
    Typedef(TypedefCtrl),
}

impl ControlFlow for ControlFlowNode {
    type Keyword = ControlFlowKeyword;

    fn fill(&mut self) {
        derive_method!(self, fill);
    }

    fn from_keyword(keyword: Self::Keyword) -> Self {
        match keyword {
            Self::Keyword::Break => {
                Self::SemiColon(SemiColonCtrl::from_keyword(SemiColonKeyword::Break))
            }
            Self::Keyword::Case => Self::AstColonAst(AstColonAstCtrl::default()),
            Self::Keyword::Continue => {
                Self::SemiColon(SemiColonCtrl::from_keyword(SemiColonKeyword::Continue))
            }
            Self::Keyword::Default => {
                Self::ColonAst(ColonAstCtrl::from_keyword(ColonAstKeyword::Default))
            }
            Self::Keyword::Do => Self::DoWhile(DoWhileCtrl::default()),
            Self::Keyword::Enum => {
                Self::IdentBlock(IdentBlockCtrl::from_keyword(IdentBlockKeyword::Enum))
            }
            Self::Keyword::For => {
                Self::ParensBlock(ParensBlockCtrl::from_keyword(ParensBlockKeyword::For))
            }
            Self::Keyword::Goto => Self::ColonIdent(ColonIdentCtrl::default()),
            Self::Keyword::If => Self::Condition(ConditionCtrl::default()),
            Self::Keyword::Return => Self::Ast(ReturnCtrl::default()),
            Self::Keyword::Struct => {
                Self::IdentBlock(IdentBlockCtrl::from_keyword(IdentBlockKeyword::Struct))
            }
            Self::Keyword::Switch => {
                Self::ParensBlock(ParensBlockCtrl::from_keyword(ParensBlockKeyword::Switch))
            }
            Self::Keyword::Typedef => Self::Typedef(TypedefCtrl::default()),
            Self::Keyword::Union => {
                Self::IdentBlock(IdentBlockCtrl::from_keyword(IdentBlockKeyword::Union))
            }
            Self::Keyword::While => {
                Self::ParensBlock(ParensBlockCtrl::from_keyword(ParensBlockKeyword::While))
            }
            Self::Keyword::Label(label) => {
                Self::ColonAst(ColonAstCtrl::from_keyword(ColonAstKeyword::Label(label)))
            }
        }
    }

    fn get_ast(&self) -> Option<&Ast> {
        derive_method!(self, get_ast)
    }

    fn get_mut(&mut self) -> Option<&mut Ast> {
        derive_method!(self, get_mut)
    }

    fn is_complete(&self) -> bool {
        derive_method!(self, is_complete)
    }

    fn is_condition(&self) -> bool {
        derive_method!(self, is_condition)
    }

    fn is_full(&self) -> bool {
        derive_method!(self, is_full)
    }

    fn is_switch(&self) -> bool {
        derive_method!(self, is_switch)
    }

    fn is_while(&self) -> bool {
        derive_method!(self, is_while)
    }

    fn push_colon(&mut self) -> bool {
        derive_method!(self, push_colon)
    }

    fn push_semicolon(&mut self) -> bool {
        derive_method!(self, push_semicolon)
    }
}

impl Push for ControlFlowNode {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf(&ast, self, "ctrl node");
        if self.is_full() {
            Err("Tried to push node in full control flow".to_owned())
        } else {
            derive_method!(self, push_block_as_leaf, ast)
        }
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_op(&op, self, "ctrl node");
        if self.is_full() {
            //TODO: This doesn't display because is caught
            Err("Tried to push operator in full control flow".to_owned())
        } else {
            derive_method!(self, push_op, op)
        }
    }
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for ControlFlowNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        derive_method!(self, fmt, f)
    }
}

/// Find if t<he current [`Ast`] corresponds to an unclosed `switch` control
/// flow, waiting for the block.
///
/// This function is called when reading `{` to see whether
pub fn switch_wanting_block(current: &Ast) -> bool {
    match current {
        Ast::Empty
        | Ast::Leaf(_)
        | Ast::Unary(_)
        | Ast::Binary(_)
        | Ast::Cast(_)
        | Ast::Ternary(_)
        | Ast::Variable(_)
        | Ast::ParensBlock(_)
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(_)
        | Ast::FunctionArgsBuild(_)
        | Ast::BracedBlock(BracedBlock { full: true, .. }) => false,
        Ast::ControlFlow(ctrl) => {
            ctrl.is_switch() || ctrl.get_ast().is_some_and(switch_wanting_block)
        }
        Ast::BracedBlock(BracedBlock { full: false, elts }) => {
            elts.last().is_some_and(switch_wanting_block)
        }
    }
}

/// Try to push a semicolon into a control flow.
///
/// Adding a semicolon makes the state of a condition move one, by marking the
/// first piece full.
pub fn try_push_semicolon_control(current: &mut Ast) -> bool {
    match current {
        Ast::Empty
        | Ast::Leaf(_)
        | Ast::Unary(_)
        | Ast::Cast(_)
        | Ast::Binary(_)
        | Ast::Ternary(_)
        | Ast::Variable(_)
        | Ast::ParensBlock(_)
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(_)
        | Ast::FunctionArgsBuild(_) => false,
        Ast::ControlFlow(ctrl) => ctrl.push_semicolon(),
        Ast::BracedBlock(BracedBlock { elts, full }) => {
            !*full && elts.last_mut().is_some_and(try_push_semicolon_control)
        }
    }
}
