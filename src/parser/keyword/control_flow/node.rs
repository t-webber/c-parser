//! Defines the control flow nodes.

use core::fmt;

use super::keyword::ControlFlowKeyword;
use super::traits::ControlFlow;
use super::types::case::AstColonAstCtrl;
use super::types::conditional::ConditionCtrl;
use super::types::default_ctrl::ColonAstCtrl;
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
        match self {
            Self::Ast(ctrl) => ctrl.fill(),
            Self::AstColonAst(ctrl) => ctrl.fill(),
            Self::ColonAst(ctrl) => ctrl.fill(),
            Self::ColonIdent(ctrl) => ctrl.fill(),
            Self::Condition(ctrl) => ctrl.fill(),
            Self::DoWhile(ctrl) => ctrl.fill(),
            Self::IdentBlock(ctrl) => ctrl.fill(),
            Self::ParensBlock(ctrl) => ctrl.fill(),
            Self::SemiColon(ctrl) => ctrl.fill(),
            Self::Typedef(ctrl) => ctrl.fill(),
        }
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
            Self::Keyword::Default => Self::ColonAst(ColonAstCtrl::default()),
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
        }
    }

    fn get_ast(&self) -> Option<&Ast> {
        match self {
            Self::Ast(ctrl) => ctrl.get_ast(),
            Self::AstColonAst(ctrl) => ctrl.get_ast(),
            Self::ColonAst(ctrl) => ctrl.get_ast(),
            Self::ColonIdent(ctrl) => ctrl.get_ast(),
            Self::Condition(ctrl) => ctrl.get_ast(),
            Self::DoWhile(ctrl) => ctrl.get_ast(),
            Self::IdentBlock(ctrl) => ctrl.get_ast(),
            Self::ParensBlock(ctrl) => ctrl.get_ast(),
            Self::SemiColon(ctrl) => ctrl.get_ast(),
            Self::Typedef(ctrl) => ctrl.get_ast(),
        }
    }

    fn get_keyword(&self) -> ControlFlowKeyword {
        match self {
            Self::Ast(ctrl) => ctrl.get_keyword(),
            Self::AstColonAst(ctrl) => ctrl.get_keyword(),
            Self::ColonAst(ctrl) => ctrl.get_keyword(),
            Self::ColonIdent(ctrl) => ctrl.get_keyword(),
            Self::Condition(ctrl) => ctrl.get_keyword(),
            Self::DoWhile(ctrl) => ctrl.get_keyword(),
            Self::IdentBlock(ctrl) => ctrl.get_keyword(),
            Self::ParensBlock(ctrl) => ctrl.get_keyword(),
            Self::SemiColon(ctrl) => ctrl.get_keyword(),
            Self::Typedef(ctrl) => ctrl.get_keyword(),
        }
    }

    fn get_mut(&mut self) -> Option<&mut Ast> {
        match self {
            Self::Ast(ctrl) => ctrl.get_mut(),
            Self::AstColonAst(ctrl) => ctrl.get_mut(),
            Self::ColonAst(ctrl) => ctrl.get_mut(),
            Self::ColonIdent(ctrl) => ctrl.get_mut(),
            Self::Condition(ctrl) => ctrl.get_mut(),
            Self::DoWhile(ctrl) => ctrl.get_mut(),
            Self::IdentBlock(ctrl) => ctrl.get_mut(),
            Self::ParensBlock(ctrl) => ctrl.get_mut(),
            Self::SemiColon(ctrl) => ctrl.get_mut(),
            Self::Typedef(ctrl) => ctrl.get_mut(),
        }
    }

    fn is_complete(&self) -> bool {
        match self {
            Self::Ast(ctrl) => ctrl.is_complete(),
            Self::AstColonAst(ctrl) => ctrl.is_complete(),
            Self::ColonAst(ctrl) => ctrl.is_complete(),
            Self::ColonIdent(ctrl) => ctrl.is_complete(),
            Self::Condition(ctrl) => ctrl.is_complete(),
            Self::DoWhile(ctrl) => ctrl.is_complete(),
            Self::IdentBlock(ctrl) => ctrl.is_complete(),
            Self::ParensBlock(ctrl) => ctrl.is_complete(),
            Self::SemiColon(ctrl) => ctrl.is_complete(),
            Self::Typedef(ctrl) => ctrl.is_complete(),
        }
    }

    fn is_full(&self) -> bool {
        match self {
            Self::Ast(ctrl) => ctrl.is_full(),
            Self::AstColonAst(ctrl) => ctrl.is_full(),
            Self::ColonAst(ctrl) => ctrl.is_full(),
            Self::ColonIdent(ctrl) => ctrl.is_full(),
            Self::Condition(ctrl) => ctrl.is_full(),
            Self::DoWhile(ctrl) => ctrl.is_full(),
            Self::IdentBlock(ctrl) => ctrl.is_full(),
            Self::ParensBlock(ctrl) => ctrl.is_full(),
            Self::SemiColon(ctrl) => ctrl.is_full(),
            Self::Typedef(ctrl) => ctrl.is_full(),
        }
    }

    fn push_colon(&mut self) -> bool {
        match self {
            Self::Ast(ctrl) => ctrl.push_colon(),
            Self::AstColonAst(ctrl) => ctrl.push_colon(),
            Self::ColonAst(ctrl) => ctrl.push_colon(),
            Self::ColonIdent(ctrl) => ctrl.push_colon(),
            Self::Condition(ctrl) => ctrl.push_colon(),
            Self::DoWhile(ctrl) => ctrl.push_colon(),
            Self::IdentBlock(ctrl) => ctrl.push_colon(),
            Self::ParensBlock(ctrl) => ctrl.push_colon(),
            Self::SemiColon(ctrl) => ctrl.push_colon(),
            Self::Typedef(ctrl) => ctrl.push_colon(),
        }
    }

    fn push_semicolon(&mut self) -> bool {
        match self {
            Self::Ast(ctrl) => ctrl.push_semicolon(),
            Self::AstColonAst(ctrl) => ctrl.push_semicolon(),
            Self::ColonAst(ctrl) => ctrl.push_semicolon(),
            Self::ColonIdent(ctrl) => ctrl.push_semicolon(),
            Self::Condition(ctrl) => ctrl.push_semicolon(),
            Self::DoWhile(ctrl) => ctrl.push_semicolon(),
            Self::IdentBlock(ctrl) => ctrl.push_semicolon(),
            Self::ParensBlock(ctrl) => ctrl.push_semicolon(),
            Self::SemiColon(ctrl) => ctrl.push_semicolon(),
            Self::Typedef(ctrl) => ctrl.push_semicolon(),
        }
    }
}

impl Push for ControlFlowNode {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        println!("\tPushing {ast} as leaf in ctrl {self}");
        if self.is_full() {
            Err("Tried to push node in full control flow".to_owned())
        } else {
            match self {
                Self::Ast(ctrl) => ctrl.push_block_as_leaf(ast),
                Self::AstColonAst(ctrl) => ctrl.push_block_as_leaf(ast),
                Self::ColonAst(ctrl) => ctrl.push_block_as_leaf(ast),
                Self::ColonIdent(ctrl) => ctrl.push_block_as_leaf(ast),
                Self::Condition(ctrl) => ctrl.push_block_as_leaf(ast),
                Self::DoWhile(ctrl) => ctrl.push_block_as_leaf(ast),
                Self::IdentBlock(ctrl) => ctrl.push_block_as_leaf(ast),
                Self::ParensBlock(ctrl) => ctrl.push_block_as_leaf(ast),
                Self::SemiColon(ctrl) => ctrl.push_block_as_leaf(ast),
                Self::Typedef(ctrl) => ctrl.push_block_as_leaf(ast),
            }
        }
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        println!("\tPushing op {op} as in ctrl {self}");
        if self.is_full() {
            Err("Tried to push operator in full control flow".to_owned())
        } else {
            match self {
                Self::Ast(ctrl) => ctrl.push_op(op),
                Self::AstColonAst(ctrl) => ctrl.push_op(op),
                Self::ColonAst(ctrl) => ctrl.push_op(op),
                Self::ColonIdent(ctrl) => ctrl.push_op(op),
                Self::Condition(ctrl) => ctrl.push_op(op),
                Self::DoWhile(ctrl) => ctrl.push_op(op),
                Self::IdentBlock(ctrl) => ctrl.push_op(op),
                Self::ParensBlock(ctrl) => ctrl.push_op(op),
                Self::SemiColon(ctrl) => ctrl.push_op(op),
                Self::Typedef(ctrl) => ctrl.push_op(op),
            }
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for ControlFlowNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ast(ctrl) => ctrl.fmt(f),
            Self::AstColonAst(ctrl) => ctrl.fmt(f),
            Self::ColonAst(ctrl) => ctrl.fmt(f),
            Self::ColonIdent(ctrl) => ctrl.fmt(f),
            Self::Condition(ctrl) => ctrl.fmt(f),
            Self::DoWhile(ctrl) => ctrl.fmt(f),
            Self::IdentBlock(ctrl) => ctrl.fmt(f),
            Self::ParensBlock(ctrl) => ctrl.fmt(f),
            Self::SemiColon(ctrl) => ctrl.fmt(f),
            Self::Typedef(ctrl) => ctrl.fmt(f),
        }
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
        | Ast::Ternary(_)
        | Ast::Variable(_)
        | Ast::ParensBlock(_)
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(_)
        | Ast::FunctionArgsBuild(_)
        | Ast::BracedBlock(BracedBlock { full: true, .. }) => false,
        Ast::ControlFlow(ctrl) => {
            ctrl.get_keyword() == ControlFlowKeyword::Switch
                || ctrl.get_ast().is_some_and(switch_wanting_block)
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
