//! Defines the control flow nodes.

use core::fmt;

use super::super::super::types::braced_blocks::BracedBlock;
use super::super::super::types::{Ast, ParensBlock};
use super::keyword::ControlFlowKeyword;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::types::literal::{Literal, Variable, VariableName};
use crate::parser::{repr_option, repr_vec};

/// Node representation of a control flow.
#[derive(Debug, PartialEq)]
pub enum ControlFlowNode {
    /// Keyword expects a node: `return 3+4`
    Ast(ControlFlowKeyword, Box<Ast>, bool),
    /// Keyword expects a colon and a node: `goto: label`
    ColonAst(ControlFlowKeyword, Option<Box<Ast>>, bool),
    /// `if` keyword
    Condition(Option<ParensBlock>, Box<Ast>, bool, Option<Box<Ast>>, bool),
    /// Keyword expects another control flow: `typedef struct`
    ControlFlow(ControlFlowKeyword, Option<Box<ControlFlowNode>>),
    /// Keyword expects an identifier and a braced block: `struct Blob {}`
    IdentBlock(ControlFlowKeyword, Option<String>, Option<BracedBlock>),
    /// Keyword expects a parenthesised block and a braced block: `switch (cond)
    /// {};`
    ParensBlock(ControlFlowKeyword, Option<ParensBlock>, Box<Ast>, bool),
    /// Keyword expects a semicolon: `break;`
    SemiColon(ControlFlowKeyword),
}

impl ControlFlowNode {
    /// Sets the control flow to full
    pub fn fill(&mut self) {
        match self {
            Self::Ast(.., full)
            | Self::ColonAst(.., full)
            | Self::ParensBlock(.., full)
            | Self::Condition(.., full) => {
                *full = true;
            }
            Self::ControlFlow(..) | Self::IdentBlock(..) | Self::SemiColon(..) => (),
        }
    }

    /// Function to return an [`Ast`], if exists
    pub const fn get_ast(&self) -> Option<&Ast> {
        match self {
            Self::Condition(_, _, true, Some(ast), false)
            | Self::Condition(_, ast, false, ..)
            | Self::ColonAst(_, Some(ast), false)
            | Self::ParensBlock(.., ast, false)
            | Self::Ast(_, ast, false) => Some(ast),
            Self::ControlFlow(..)
            | Self::Condition(..)
            | Self::IdentBlock(..)
            | Self::SemiColon(_)
            | Self::ParensBlock(.., true)
            | Self::ColonAst(.., true)
            | Self::ColonAst(_, None, false)
            | Self::Ast(.., true) => None,
        }
    }
    /// Function to return an [`Ast`], if exists
    pub fn get_ast_mut(&mut self) -> Option<&mut Ast> {
        match self {
            Self::Condition(_, _, true, Some(ast), false)
            | Self::Condition(_, ast, false, ..)
            | Self::ColonAst(_, Some(ast), false)
            | Self::ParensBlock(.., ast, false)
            | Self::Ast(_, ast, false) => Some(ast),
            Self::ControlFlow(..)
            | Self::IdentBlock(..)
            | Self::Condition(..)
            | Self::SemiColon(_)
            | Self::ParensBlock(.., true)
            | Self::ColonAst(.., true)
            | Self::ColonAst(_, None, false)
            | Self::Ast(.., true) => None,
        }
    }

    /// Get keyword from node
    pub const fn get_keyword(&self) -> &ControlFlowKeyword {
        match self {
            Self::Ast(keyword, ..)
            | Self::ColonAst(keyword, ..)
            | Self::ControlFlow(keyword, _)
            | Self::IdentBlock(keyword, ..)
            | Self::ParensBlock(keyword, ..)
            | Self::SemiColon(keyword) => keyword,
            Self::Condition(..) => &ControlFlowKeyword::If,
        }
    }

    /// Checks if the control flow is complete
    ///
    /// Complete means that the control flow means something on its own, no
    /// addition data is required. It doesn't mean you can't push data in it
    /// anymore, it just means you don't have to.
    pub const fn is_complete(&self) -> bool {
        match self {
            Self::Condition(.., full_s, failure, full_f) => {
                *full_f || (*full_s && failure.is_none())
            }
            Self::Ast(.., full) | Self::ColonAst(.., full) => *full,
            Self::ControlFlow(_, node) => node.is_some(),
            Self::IdentBlock(_, ident, node) => node.is_some() && ident.is_some(),
            Self::ParensBlock(_, parens, _, full) => parens.is_some() && *full,
            Self::SemiColon(_) => true,
        }
    }
    /// Checks if the control flow is full
    ///
    /// Full means that nothing can be pushed inside anymore
    pub const fn is_full(&self) -> bool {
        match self {
            Self::Condition(.., full_s, _, full_f) => *full_f && *full_s,
            Self::Ast(.., full) | Self::ColonAst(.., full) => *full,
            Self::ControlFlow(_, node) => node.is_some(),
            Self::IdentBlock(_, ident, node) => node.is_some() && ident.is_some(),
            Self::ParensBlock(_, parens, _, full) => parens.is_some() && *full,
            Self::SemiColon(_) => true,
        }
    }

    /// Tries to push a block as leaf inside the control flow node.
    ///
    /// See [`Ast::push_block_as_leaf`] for more information.
    pub fn push_block_as_leaf(&mut self, node: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        println!("\t\tPushing {node} as leaf in ctrl {self}");
        match self {
            Self::Ast(_, ast, false)
            | Self::ColonAst(_, Some(ast), false)
            | Self::Condition(Some(_), _, true, Some(ast), false)
            | Self::Condition(Some(_), ast, false, None, false)
            | Self::ParensBlock(_, Some(_), ast, false) => ast.push_block_as_leaf(node)?,
            Self::ColonAst(keyword, None, false) => {
                return Err(format!("Missing colon after {keyword}."));
            }
            Self::ControlFlow(keyword, old_ctrl @ None) => {
                if let Ast::ControlFlow(node_ctrl) = node {
                    *old_ctrl = Some(Box::from(node_ctrl));
                } else {
                    return Err(format!("{keyword} expected a keyword but found {node}",));
                }
            }
            Self::ParensBlock(keyword, old_parens @ None, _, false) => {
                if let Ast::ParensBlock(node_parens) = node {
                    *old_parens = Some(node_parens);
                } else {
                    return Err(format!(
                        "{keyword} expected a parenthesised block but found {node}",
                    ));
                }
            }
            Self::IdentBlock(_, Some(_), old_block @ None) => {
                if let Ast::BracedBlock(mut node_block) = node {
                    node_block.full = true;
                    *old_block = Some(node_block);
                } else {
                    *old_block = Some(BracedBlock {
                        elts: vec![node],
                        full: true,
                    });
                }
            }
            Self::IdentBlock(keyword, ident @ None, None) => {
                if let Ast::Leaf(Literal::Variable(Variable { attrs, name })) = node {
                    if attrs.is_empty() {
                        if let VariableName::UserDefined(name_str) = name {
                            *ident = Some(name_str);
                        } else {
                            return Err(format!(
                                "Expected identifier after {keyword}, but found keyword {name}"
                            ));
                        }
                    } else {
                        return Err(format!(
                            "Expected identifier after {keyword}, but found variable {name} with attributes {}",
                            repr_vec(&attrs)
                        ));
                    }
                }
            }
            Self::Condition(cond @ None, ..) => {
                if let Ast::ParensBlock(parens) = node {
                    *cond = Some(parens);
                } else {
                    return Err("Missing condition after `if` keyword.".to_owned());
                }
            }
            Self::Ast(.., true)
            | Self::ColonAst(.., true)
            | Self::ControlFlow(_, Some(_))
            | Self::Condition(..)
            | Self::ParensBlock(..)
            | Self::IdentBlock(_, _, Some(_))
            | Self::SemiColon(_) => {
                panic!("Tried to push not on full block, but it is not pushable")
            }
        }
        Ok(())
    }

    /// Tries to push a colon inside the control flow node.
    pub fn push_colon(&mut self) -> Result<(), String> {
        if let Self::ColonAst(_, node @ None, false) = self {
            *node = Some(Box::from(Ast::Empty));
            Ok(())
        } else {
            Err("Found extra colon: illegal in control flow keyword context.".to_owned())
        }
    }

    /// Tries to push an operator inside the control flow node.
    ///
    /// See [`Ast::push_op`] for more information.
    pub fn push_op<T: fmt::Display + OperatorConversions>(&mut self, op: T) -> Result<(), String> {
        #[cfg(feature = "debug")]
        println!("\t\tPushing op {op} in {self}");
        match self {
            Self::Condition(Some(_), ast, false, None, false)
            | Self::Condition(Some(_), _, true, Some(ast), false)
            | Self::ParensBlock(_, Some(_), ast, false)
            | Self::Ast(_, ast, false)
            | Self::ColonAst(_, Some(ast), false) => ast.push_op(op),
            Self::ColonAst(_, ast @ None, false) => {
                op.try_to_node().map(|node| *ast = Some(Box::new(node)))
            }
            Self::ControlFlow(_, Some(node)) => node.push_op(op),
            Self::IdentBlock(_, Some(_), Some(BracedBlock { elts, full: false })) => {
                if let Some(last) = elts.last_mut() {
                    last.push_op(op)
                } else {
                    elts.push(op.try_to_node()?);
                    Ok(())
                }
            }
            Self::Ast(.., true)
            | Self::ColonAst(.., true)
            | Self::ControlFlow(_, None)
            | Self::ParensBlock(.., true)
            | Self::ParensBlock(_, None, ..)
            | Self::IdentBlock(..)
            | Self::Condition(None, ..)
            | Self::Condition(.., true, _, true)
            | Self::Condition(.., true, None, false)
            | Self::SemiColon(_) => Err(format!(
                "Illegal operator {op} in {} control flow.",
                self.get_keyword()
            )),
            Self::Condition(_, _, false, _, true) | Self::Condition(_, _, false, Some(_), _) => {
                panic!("never happens")
            }
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for ControlFlowNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ast(keyword, ast, full) => {
                write!(f, "<{keyword} {ast}{}>", if *full { ".." } else { "" })
            }
            Self::ColonAst(keyword, ast, full) => {
                write!(
                    f,
                    "<{keyword}: {}{}>",
                    repr_option(ast),
                    if *full { ".." } else { "" }
                )
            }
            Self::ControlFlow(keyword, ctrl) => {
                write!(f, "<{keyword} {}>", repr_option(ctrl))
            }
            Self::IdentBlock(keyword, ident, block) => write!(
                f,
                "<{keyword} {} {}>",
                repr_option(ident),
                repr_option(block)
            ),
            Self::ParensBlock(keyword, parens_block, ast, full) => {
                write!(
                    f,
                    "<{keyword} {} {ast}{}>",
                    repr_option(parens_block),
                    if *full { ".." } else { "" }
                )
            }
            Self::SemiColon(keyword) => write!(f, "<{keyword}>"),
            Self::Condition(cond, success, full_s, failure, full_f) => write!(
                f,
                "<if {} {success}{}{}{}>",
                repr_option(cond),
                if *full_s { "" } else { ".." },
                failure
                    .as_ref()
                    .map_or_else(String::new, |ast| format!(" else {ast}")),
                if *full_f { "" } else { ".\u{b2}." }
            ),
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
        | Ast::ParensBlock(_)
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(_)
        | Ast::FunctionArgsBuild(_) => false,
        Ast::ControlFlow(
            ControlFlowNode::Condition(_, ast, full @ false, ..)
            | ControlFlowNode::Condition(_, _, true, Some(ast), full @ false),
        ) => {
            if try_push_semicolon_control(ast) {
                if !ast.can_push_leaf() {
                    *full = true;
                }
            } else {
                *full = true;
            }
            true
        }
        Ast::ControlFlow(ControlFlowNode::Condition(_, _, true, None, full @ false)) => {
            *full = true;
            true
        }
        Ast::ControlFlow(ctrl) => ctrl.get_ast_mut().is_some_and(try_push_semicolon_control),
        Ast::BracedBlock(BracedBlock { elts, full }) => {
            !*full && elts.last_mut().is_some_and(try_push_semicolon_control)
        }
    }
}
