//! Defines the control flow nodes.

use core::fmt;

use super::super::super::types::blocks::BracedBlock;
use super::super::super::types::{Ast, ParensBlock};
use super::keyword::ControlFlowKeyword;
use crate::parser::repr_option;

/// Node representation of a control flow.
#[derive(Debug, PartialEq)]
pub enum ControlFlowNode {
    /// Keyword expects a node: `return 3+4`
    Ast(ControlFlowKeyword, Box<Ast>),
    /// Keyword expects a colon and a node: `goto: label`
    ColonAst(ControlFlowKeyword, Option<Box<Ast>>),
    /// Keyword expects another control flow: `typedef struct`
    ControlFlow(ControlFlowKeyword, Option<Box<ControlFlowNode>>),
    /// Keyword expects an identifier and a braced block: `struct Blob {}`
    IdentBlock(ControlFlowKeyword, Option<String>, Option<BracedBlock>),
    /// Keyword expects a parenthesised block and a braced block: `switch (cond)
    /// {};`
    ParensBlock(ControlFlowKeyword, Option<ParensBlock>, Option<BracedBlock>),
    /// Keyword expects a semicolon: `break;`
    SemiColon(ControlFlowKeyword),
}

impl ControlFlowNode {
    /// Get keyword from node
    pub const fn get_keyword(&self) -> &ControlFlowKeyword {
        match self {
            Self::Ast(keyword, _)
            | Self::ColonAst(keyword, _)
            | Self::ControlFlow(keyword, _)
            | Self::IdentBlock(keyword, _, _)
            | Self::ParensBlock(keyword, _, _)
            | Self::SemiColon(keyword) => keyword,
        }
    }

    /// Checks if the control flow is full
    pub fn is_full(&self) -> bool {
        match self {
            Self::Ast(_, ast) => **ast != Ast::Empty,
            Self::ColonAst(_, ast) => ast.as_ref().is_some_and(|node| **node != Ast::Empty),
            Self::ControlFlow(_, control_flow_node) => control_flow_node
                .as_ref()
                .is_some_and(|node| node.is_full()),
            Self::IdentBlock(_, ident, block) => ident.is_some() && block.is_some(),
            Self::ParensBlock(_, parens_block, block) => parens_block.is_some() && block.is_some(),
            Self::SemiColon(_) => true,
        }
    }

    /// Tries to push a block as leaf inside the control flow node.
    ///
    /// See [`Ast::push_block_as_leaf`] for more information.
    pub fn push_block_as_leaf(&mut self, node: Ast) -> Result<(), String> {
        #[expect(clippy::wildcard_enum_match_arm)]
        match self {
            Self::Ast(_, ast) | Self::ColonAst(_, Some(ast)) if **ast == Ast::Empty => {
                *ast = Box::new(node);
            }
            Self::ColonAst(_, None) => return Err("Missing colon after keyword.".to_owned()),
            Self::ControlFlow(keyword, old_ctrl @ None) => {
                if let Ast::ControlFlow(node_ctrl) = node {
                    *old_ctrl = Some(Box::from(node_ctrl));
                } else {
                    return Err(format!("{keyword} expected a keyword but found {node}",));
                }
            }
            Self::ParensBlock(keyword, old_parens @ None, _) => {
                if let Ast::ParensBlock(node_parens) = node {
                    *old_parens = Some(node_parens);
                } else {
                    return Err(format!(
                        "{keyword} expected a parenthesised block but found {node}",
                    ));
                }
            }
            Self::ParensBlock(keyword, _, old_block @ None)
            | Self::IdentBlock(keyword, _, old_block @ None) => {
                if let Ast::BracedBlock(node_block) = node {
                    *old_block = Some(node_block);
                } else {
                    return Err(format!(
                        "{keyword} expected a braced block but found {node}",
                    ));
                }
            }
            _ => panic!("Tried to push not on full block, but it is not pushable"),
        }
        Ok(())
    }

    /// Tries to push a colon inside the control flow node.
    pub fn push_colon(&mut self) -> Result<(), String> {
        if let Self::ColonAst(_, node @ None) = self {
            *node = Some(Box::from(Ast::Empty));
            Ok(())
        } else {
            Err("Found extra colon: illegal in control flow keyword context.".to_owned())
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for ControlFlowNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ast(keyword, ast) => write!(f, "({keyword} {ast})"),
            Self::ColonAst(keyword, ast) => {
                write!(f, "({keyword}: {})", repr_option(ast))
            }
            Self::ControlFlow(keyword, ctrl) => {
                write!(f, "({keyword} {})", repr_option(ctrl))
            }
            Self::IdentBlock(keyword, ident, block) => write!(
                f,
                "({keyword} {} {})",
                repr_option(ident),
                repr_option(block)
            ),
            Self::ParensBlock(keyword, parens_block, block) => {
                write!(
                    f,
                    "({keyword} {} {})",
                    repr_option(parens_block),
                    repr_option(block)
                )
            }
            Self::SemiColon(keyword) => write!(f, "({keyword})"),
        }
    }
}
