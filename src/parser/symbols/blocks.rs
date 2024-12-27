extern crate alloc;
use alloc::vec::IntoIter;
use core::mem;

use super::super::parse_content::parse_block;
use super::super::state::ParsingState;
use super::super::tree::binary::BinaryOperator;
use super::super::tree::node::Node;
use super::super::tree::ListInitialiser;
use crate::errors::api::{CompileError, Location};
use crate::lexer::api::Token;
use crate::parser::state::BlockState;
use crate::parser::tree::blocks::Block;

// TODO: check for nested
pub enum TodoBlock {
    CloseBraceBlock,
    CloseBracket,
    CloseParens,
    None,
    OpenBraceBlock,
    OpenBracket,
    OpenParens,
    SemiColon,
}

/// Manages recursions calls and creates blocks
pub fn blocks_handler(
    current: &mut Node,
    tokens: &mut IntoIter<Token>,
    p_state: &mut ParsingState,
    location: Location,
    block_state: &TodoBlock,
) -> Result<(), CompileError> {
    match block_state {
        // semi-colon
        TodoBlock::SemiColon => {
            handle_colon(current);
            parse_block(tokens, p_state, current)
        }
        // parenthesis
        TodoBlock::CloseParens if !current.try_close_function() => {
            p_state.opened_blocks.push(BlockState::Parenthesis);
            Ok(())
        }
        TodoBlock::OpenParens if !current.try_make_function() => {
            let mut parenthesized_block = Node::Empty;
            parse_block(tokens, p_state, &mut parenthesized_block)?;
            if p_state.opened_blocks.pop() == Some(BlockState::Parenthesis) {
                current
                    .push_block_as_leaf(Node::ParensBlock(Box::from(parenthesized_block)))
                    .map_err(|err| location.into_error(err))?;
                parse_block(tokens, p_state, current)
            } else {
                Err(location.into_error(mismatched_err('(', ')')))
            }
        }
        // bracket
        TodoBlock::CloseBracket => {
            p_state.opened_blocks.push(BlockState::Bracket);
            Ok(())
        }
        TodoBlock::OpenBracket => {
            let mut bracket_node = Node::Empty;
            parse_block(tokens, p_state, &mut bracket_node)?;
            if p_state.opened_blocks.pop() == Some(BlockState::Bracket) {
                if let Err(err) = current.push_op(BinaryOperator::ArraySubscript) {
                    Err(location.into_error(err))
                } else {
                    current
                        .push_block_as_leaf(bracket_node)
                        .map_err(|err| location.into_error(err))?;
                    parse_block(tokens, p_state, current)
                }
            } else {
                Err(location.into_error(mismatched_err('[', ']')))
            }
        }
        // brace
        TodoBlock::CloseBraceBlock
            if current
                .apply_to_last_list_initialiser(&|_, full| *full = true)
                .is_err() =>
        {
            p_state.opened_blocks.push(BlockState::Brace);
            Ok(())
        }
        TodoBlock::OpenBraceBlock => match current.can_push_list_initialiser() {
            Err(op) => Err(location.into_error(format!(
                "Found operator '{op}' applied on list initialiser '{{}}', but this is not allowed."
            ))),
            Ok(true) => {
                current
                    .push_block_as_leaf(Node::ListInitialiser(ListInitialiser::default()))
                    .map_err(|err| location.into_error(err))?;
                parse_block(tokens, p_state, current)
            }
            Ok(false) => handle_brace_block_open(current, tokens, p_state, location),
        },
        // others
        TodoBlock::None
        | TodoBlock::OpenParens
        | TodoBlock::CloseParens
        | TodoBlock::CloseBraceBlock => parse_block(tokens, p_state, current),
    }
}

fn handle_brace_block_open(
    current: &mut Node,
    tokens: &mut IntoIter<Token>,
    p_state: &mut ParsingState,
    location: Location,
) -> Result<(), CompileError> {
    let mut brace_block = Node::Block(Block::default());
    parse_block(tokens, p_state, &mut brace_block)?;
    if p_state.opened_blocks.pop() != Some(BlockState::Brace) {
        return Err(location.into_error(mismatched_err('{', '}')));
    }
    if let Node::Block(Block { full, .. }) = &mut brace_block {
        *full = true;
    } else {
        panic!("a block can't be changed to another node")
    }
    //
    if let Node::Block(Block { elts, full }) = current
        && !*full
    {
        elts.push(brace_block);
    } else if *current == Node::Empty {
        *current = brace_block;
    } else {
        *current = Node::Block(Block {
            elts: vec![mem::take(current), brace_block],
            full: false,
        });
    }
    parse_block(tokens, p_state, current)
}

fn handle_colon(current: &mut Node) {
    if let Node::Block(Block { elts, full }) = current
        && !*full
    {
        elts.push(Node::Empty);
    } else if *current != Node::Empty {
        *current = Node::Block(Block {
            elts: vec![mem::take(current), Node::Empty],
            full: false,
        });
    } else {
        /* last is empty: nothing to be done */
    }
}

fn mismatched_err(open: char, close: char) -> String {
    format!(
        "Mismatched {open}: You either forgot a closing {close} or there is an extra semi-colon."
    )
}
