extern crate alloc;
use alloc::vec::IntoIter;
use core::mem;

use super::super::parse_content::parse_block;
use super::super::state::ParsingState;
use super::super::tree::ListInitialiser;
use super::super::tree::binary::BinaryOperator;
use super::super::tree::node::Ast;
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
    current: &mut Ast,
    tokens: &mut IntoIter<Token>,
    p_state: &mut ParsingState,
    location: Location,
    block_state: &TodoBlock,
) -> Result<(), CompileError> {
    match block_state {
        // semi-colon
        TodoBlock::SemiColon => {
            handle_semicolon(current);
            parse_block(tokens, p_state, current)
        }
        // parenthesis
        TodoBlock::CloseParens if !current.try_close_function() => {
            p_state.opened_blocks.push(BlockState::Parenthesis);
            Ok(())
        }
        TodoBlock::OpenParens if !current.try_make_function() => {
            let mut parenthesized_block = Ast::Empty;
            parse_block(tokens, p_state, &mut parenthesized_block)?;
            if p_state.opened_blocks.pop() == Some(BlockState::Parenthesis) {
                current
                    .push_block_as_leaf(Ast::ParensBlock(Box::from(parenthesized_block)))
                    .map_err(|err| location.into_error(err))?;
                parse_block(tokens, p_state, current)
            } else {
                Err(BlockState::Parenthesis.mismatched_err_end(location))
            }
        }
        // bracket
        TodoBlock::CloseBracket => {
            p_state.opened_blocks.push(BlockState::Bracket);
            Ok(())
        }
        TodoBlock::OpenBracket => {
            let mut bracket_node = Ast::Empty;
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
                Err(BlockState::Bracket.mismatched_err_end(location))
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
                    .push_block_as_leaf(Ast::ListInitialiser(ListInitialiser::default()))
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
    current: &mut Ast,
    tokens: &mut IntoIter<Token>,
    p_state: &mut ParsingState,
    location: Location,
) -> Result<(), CompileError> {
    let mut brace_block = Ast::Block(Block::default());
    parse_block(tokens, p_state, &mut brace_block)?;
    if p_state.opened_blocks.pop() != Some(BlockState::Brace) {
        return Err(BlockState::Brace.mismatched_err_end(location));
    }
    if let Ast::Block(Block { full, .. }) = &mut brace_block {
        *full = true;
    } else {
        panic!("a block can't be changed to another node")
    }
    #[expect(clippy::wildcard_enum_match_arm)]
    match current {
        Ast::Block(Block { elts, full }) if !*full => elts.push(brace_block),
        Ast::Empty => *current = brace_block,
        _ => {
            *current = Ast::Block(Block {
                elts: vec![mem::take(current), brace_block],
                full: false,
            });
        }
    }
    parse_block(tokens, p_state, current)
}

fn handle_semicolon(current: &mut Ast) {
    if let Ast::Block(Block { elts, full }) = current
        && !*full
    {
        elts.push(Ast::Empty);
    } else if *current != Ast::Empty {
        *current = Ast::Block(Block {
            elts: vec![mem::take(current), Ast::Empty],
            full: false,
        });
    } else {
        /* last is empty: nothing to be done */
    }
}
