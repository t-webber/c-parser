extern crate alloc;
use alloc::vec::IntoIter;
use core::mem;

use super::super::parse_content::parse_block;
use super::super::state::{Block, ParsingState};
use super::super::tree::binary::BinaryOperator;
use super::super::tree::node::Node;
use super::super::tree::ListInitialiser;
use crate::errors::api::{CompileError, Location};
use crate::lexer::api::Token;

// TODO: check for nested
pub enum BlockState {
    CloseBraceBlock,
    CloseBracket,
    CloseParens,
    None,
    OpenBraceBlock,
    OpenBracket,
    OpenParens,
    SemiColon,
}

pub fn blocks_handler(
    current: &mut Node,
    tokens: &mut IntoIter<Token>,
    p_state: &mut ParsingState,
    location: Location,
    block_state: &BlockState,
) -> Result<(), CompileError> {
    match block_state {
        // semi-colon
        BlockState::SemiColon => {
            if let Node::Block(block) = current
                && !block.last().is_none_or(|node| *node == Node::Empty)
                {
                block.push(Node::Empty);
            } else if *current != Node::Empty {
                *current = Node::Block(vec![mem::take(current), Node::Empty]);
            } else {
                /* last is empty: nothing to be done */
            }
            parse_block(tokens, p_state, current)
        }
        // parenthesis
        BlockState::CloseParens if !current.try_close_function() => {
            p_state.opened_blocks.push(Block::Parenthesis);
            Ok(())
        }
        BlockState::OpenParens if !current.try_make_function() => {
            let mut parenthesized_block = Node::Empty;
            parse_block(tokens, p_state, &mut parenthesized_block)?;
            if p_state.opened_blocks.pop() == Some(Block::Parenthesis) {
                current
                    .push_block_as_leaf(Node::ParensBlock(Box::from(parenthesized_block)))
                    .map_err(|err| location.into_error( err))?;
                parse_block(tokens, p_state, current)
            } else {
                Err(location.into_error( mismatched_err('(', ')')))
            }
        }
        // bracket
        BlockState::CloseBracket => {
            p_state.opened_blocks.push(Block::Bracket);
            Ok(())
        }
        BlockState::OpenBracket => {
            let mut bracket_node = Node::Empty;
            parse_block(tokens, p_state, &mut bracket_node)?;
            if p_state.opened_blocks.pop() == Some(Block::Bracket) {
                if let Err(err) = current.push_op(BinaryOperator::ArraySubscript) { Err(location.into_error( err)) } else {
                    current
                    .push_block_as_leaf(bracket_node)
                    .map_err(|err| location.into_error( err))?;
                    parse_block(tokens, p_state, current)
                }
            } else {
                Err(location.into_error(mismatched_err('[', ']')))
            }
        }
        // brace
        BlockState::CloseBraceBlock
            if current
                .edit_list_initialiser(&|_, full| *full = true)
                .is_err() =>
        {
            p_state.opened_blocks.push(Block::Brace);
            Ok(())
        }
        BlockState::OpenBraceBlock => {
            match current.can_push_list_initialiser() {
                Err(op) => Err(location.into_error( format!("Found operator '{op}' applied on list initialiser '{{}}', but this is not allowed."))),
                Ok(true) => {
                    current.push_block_as_leaf(Node::ListInitialiser(ListInitialiser::default())).map_err(|err| location.into_error( err))?;
                    parse_block(tokens, p_state, current)
                }
                Ok(false) => {
                    let mut brace_block = Node::Empty;
                    parse_block(tokens, p_state, &mut brace_block)?;
                    if p_state.opened_blocks.pop() == Some(Block::Brace) {
                        if let Node::Block(vec) = current {
                            vec.push(brace_block);
                        } else if *current == Node::Empty {
                            *current = brace_block;
                        } else {
                            *current =
                                Node::Block(vec![mem::take(current), brace_block]);
                        }
                        parse_block(tokens, p_state, current)
                    } else {
                        Err(location.into_error(mismatched_err('{', '}')))
                    }

                }
            }
        }
        // others
        BlockState::None
        | BlockState::OpenParens
        | BlockState::CloseParens
        | BlockState::CloseBraceBlock => parse_block(tokens, p_state, current),
    }
}

fn mismatched_err(open: char, close: char) -> String {
    format!(
        "Mismatched {open}: You either forgot a closing {close} or there is an extra semi-colon."
    )
}
