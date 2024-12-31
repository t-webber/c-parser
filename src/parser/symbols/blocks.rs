extern crate alloc;
use alloc::vec::IntoIter;
use core::mem;

use super::super::modifiers::functions::{try_close_function, try_make_function};
use super::super::modifiers::list_initialiser::{
    apply_to_last_list_initialiser, can_push_list_initialiser
};
use super::super::parse_content::parse_block;
use super::super::state::{BlockState, ParsingState};
use super::super::types::binary::BinaryOperator;
use super::super::types::blocks::Block;
use super::super::types::{Ast, ListInitialiser, ParensBlock};
use crate::errors::api::{CompileError, Location};
use crate::lexer::api::Token;

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
        TodoBlock::CloseParens if !try_close_function(current) => {
            p_state.opened_blocks.push(BlockState::Parenthesis);
            Ok(())
        }
        TodoBlock::OpenParens if !try_make_function(current) => {
            let mut parenthesized_block = Ast::Empty;
            parse_block(tokens, p_state, &mut parenthesized_block)?;
            if p_state.opened_blocks.pop() == Some(BlockState::Parenthesis) {
                current
                    .push_block_as_leaf(ParensBlock::make_parens_ast(parenthesized_block))
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
            if apply_to_last_list_initialiser(current, &|_, full| *full = true).is_err() =>
        {
            p_state.opened_blocks.push(BlockState::Brace);
            Ok(())
        }
        TodoBlock::OpenBraceBlock => match can_push_list_initialiser(current) {
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
    current.push_braced_block(brace_block);
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
