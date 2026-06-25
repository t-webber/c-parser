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
use crate::errors::api::{ErrorLocation, Located};
use crate::parser::modifiers::push::Push;
use crate::parser::operators::api::OperatorConversions;
use crate::parser::symbols::api::BracedBlock;
use crate::parser::tree::Ast;
use crate::utils::display;

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
#[derive(Debug)]
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
    type Keyword = Located<ControlFlowKeyword>;

    fn as_ast(&self) -> Option<&Ast> {
        derive_method!(self, as_ast)
    }

    fn as_ast_mut(&mut self) -> Option<&mut Ast> {
        derive_method!(self, as_ast_mut)
    }

    fn as_while(&self) -> Result<Option<ErrorLocation>, String> {
        derive_method!(self, as_while)
    }

    fn fill(&mut self) {
        derive_method!(self, fill);
    }

    fn from_keyword(keyword: Self::Keyword) -> Self {
        let (value, loc) = keyword.into_inner();
        match value {
            ControlFlowKeyword::Break =>
                Self::SemiColon(SemiColonCtrl::from_keyword(loc.wrap(SemiColonKeyword::Break))),
            ControlFlowKeyword::Case => Self::AstColonAst(AstColonAstCtrl::from_keyword(loc)),
            ControlFlowKeyword::Continue =>
                Self::SemiColon(SemiColonCtrl::from_keyword(loc.wrap(SemiColonKeyword::Continue))),
            ControlFlowKeyword::Default =>
                Self::ColonAst(ColonAstCtrl::from_keyword(loc.wrap(ColonAstKeyword::Default))),
            ControlFlowKeyword::Do => Self::DoWhile(DoWhileCtrl::from_keyword(loc)),
            ControlFlowKeyword::Enum =>
                Self::IdentBlock(IdentBlockCtrl::from_keyword(loc.wrap(IdentBlockKeyword::Enum))),
            ControlFlowKeyword::For =>
                Self::ParensBlock(ParensBlockCtrl::from_keyword(loc.wrap(ParensBlockKeyword::For))),
            ControlFlowKeyword::Goto => Self::ColonIdent(ColonIdentCtrl::from_keyword(loc)),
            ControlFlowKeyword::If => Self::Condition(ConditionCtrl::from_keyword(loc)),
            ControlFlowKeyword::Return => Self::Ast(ReturnCtrl::from_keyword(loc)),
            ControlFlowKeyword::Struct =>
                Self::IdentBlock(IdentBlockCtrl::from_keyword(loc.wrap(IdentBlockKeyword::Struct))),
            ControlFlowKeyword::Switch => Self::ParensBlock(ParensBlockCtrl::from_keyword(
                loc.wrap(ParensBlockKeyword::Switch),
            )),
            ControlFlowKeyword::Typedef => Self::Typedef(TypedefCtrl::from_keyword(loc)),
            ControlFlowKeyword::Union =>
                Self::IdentBlock(IdentBlockCtrl::from_keyword(loc.wrap(IdentBlockKeyword::Union))),
            ControlFlowKeyword::While => Self::ParensBlock(ParensBlockCtrl::from_keyword(
                loc.wrap(ParensBlockKeyword::While),
            )),
            ControlFlowKeyword::Label(label) =>
                Self::ColonAst(ColonAstCtrl::from_keyword(loc.wrap(ColonAstKeyword::Label(label)))),
        }
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

    fn location(&self) -> ErrorLocation {
        derive_method!(self, location)
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
        T: OperatorConversions + fmt::Display,
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

display!(ControlFlowNode, self, f, derive_method!(self, fmt, f));

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
        | Ast::FunctionArgsBuild(..)
        | Ast::BracedBlock(BracedBlock { full: true, .. }) => false,
        Ast::ControlFlow(ctrl) =>
            ctrl.is_switch() || ctrl.as_ast().is_some_and(switch_wanting_block),
        Ast::BracedBlock(BracedBlock { full: false, elts, .. }) =>
            elts.last().is_some_and(switch_wanting_block),
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
        | Ast::FunctionArgsBuild(..) => false,
        Ast::ControlFlow(ctrl) => ctrl.push_semicolon(),
        Ast::BracedBlock(BracedBlock { elts, full, .. }) =>
            !*full && elts.last_mut().is_some_and(try_push_semicolon_control),
    }
}
