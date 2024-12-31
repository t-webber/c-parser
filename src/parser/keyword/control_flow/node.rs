use super::keyword::ControlFlowKeyword;
use crate::parser::tree::ast::{Ast, ParensBlock};
use crate::parser::tree::blocks::Block;

#[derive(Debug, PartialEq)]
pub enum ControlFlowNode {
    Ast(ControlFlowKeyword, Box<Ast>),
    ColonAst(ControlFlowKeyword, Option<Box<Ast>>),
    ControlFlow(ControlFlowKeyword, Option<Box<ControlFlowNode>>),
    IdentBlock(ControlFlowKeyword, Option<String>, Option<Block>),
    ParensBlock(ControlFlowKeyword, Option<ParensBlock>, Option<Block>),
    SemiColon(ControlFlowKeyword),
}

impl ControlFlowNode {
    pub const fn get_keyword(&self) -> &ControlFlowKeyword {
        match self {
            Self::Ast(control_flow_keyword, _)
            | Self::ColonAst(control_flow_keyword, _)
            | Self::ControlFlow(control_flow_keyword, _)
            | Self::IdentBlock(control_flow_keyword, _, _)
            | Self::ParensBlock(control_flow_keyword, _, _)
            | Self::SemiColon(control_flow_keyword) => control_flow_keyword,
        }
    }

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
                if let Ast::Block(node_block) = node {
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

    pub fn push_colon(&mut self) -> Result<(), String> {
        if let Self::ColonAst(_, node @ None) = self {
            *node = Some(Box::from(Ast::Empty));
            Ok(())
        } else {
            Err("Found extra colon: illegal in control flow keyword context.".to_owned())
        }
    }
}
