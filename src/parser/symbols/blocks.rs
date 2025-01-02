//! Handler for block character

extern crate alloc;
use alloc::vec::IntoIter;
use core::mem;

use super::super::modifiers::list_initialiser::{
    apply_to_last_list_initialiser, can_push_list_initialiser
};
use super::super::parse_content::parse_block;
use super::super::state::ParsingState;
use super::super::types::binary::BinaryOperator;
use super::super::types::braced_blocks::BracedBlock;
use super::super::types::{Ast, ListInitialiser, ParensBlock};
use crate::errors::api::{CompileError, Location};
use crate::lexer::api::Token;
use crate::parser::modifiers::functions::{can_make_function, make_function};
use crate::parser::state::BlockType;

/// State to indicate what needs to be done
pub enum TodoBlock {
    /// `}`
    CloseBraceBlock,
    /// `]`
    CloseBracket,
    /// `)`
    CloseParens,
    /// `{`
    OpenBraceBlock,
    /// `[`
    OpenBracket,
    /// `(`
    OpenParens,
    /// `;`
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
        TodoBlock::CloseParens => {
            p_state.push_closing_block(BlockType::Parenthesis, location);
            Ok(())
        }
        TodoBlock::OpenParens => handle_parenthesis_open(current, p_state, tokens, location),
        // bracket
        TodoBlock::CloseBracket => {
            p_state.push_closing_block(BlockType::Bracket, location);
            Ok(())
        }
        TodoBlock::OpenBracket => {
            let mut bracket_node = Ast::Empty;
            parse_block(tokens, p_state, &mut bracket_node)?;
            if p_state.pop_and_compare_block(&BlockType::Bracket) {
                if let Err(err) = current.push_op(BinaryOperator::ArraySubscript) {
                    Err(location.into_error(err))
                } else {
                    current
                        .push_block_as_leaf(bracket_node)
                        .map_err(|err| location.into_error(err))?;
                    parse_block(tokens, p_state, current)
                }
            } else {
                Err(BlockType::Bracket.mismatched_err_end(location))
            }
        }
        // brace
        TodoBlock::CloseBraceBlock
            if apply_to_last_list_initialiser(current, &|_, full| *full = true).is_err() =>
        {
            p_state.push_closing_block(BlockType::Brace, location);
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
        TodoBlock::CloseBraceBlock => parse_block(tokens, p_state, current),
    }
}

/// Handler for `{`
///
/// Deals with recursion and merges the braced-blocks
fn handle_brace_block_open(
    current: &mut Ast,
    tokens: &mut IntoIter<Token>,
    p_state: &mut ParsingState,
    location: Location,
) -> Result<(), CompileError> {
    let mut brace_block = Ast::BracedBlock(BracedBlock::default());
    parse_block(tokens, p_state, &mut brace_block)?;
    if !p_state.pop_and_compare_block(&BlockType::Brace) {
        return Err(BlockType::Brace.mismatched_err_end(location));
    }
    current.push_braced_block(brace_block);
    parse_block(tokens, p_state, current)
}

/// Handler for `(`
///
/// Deals with recursion and pushes the function arguments if needed
fn handle_parenthesis_open(
    current: &mut Ast,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: Location,
) -> Result<(), CompileError> {
    if can_make_function(current) {
        let mut arguments_node = Ast::FunctionArgsBuild(vec![Ast::Empty]);
        parse_block(tokens, p_state, &mut arguments_node)?;
        if p_state.pop_and_compare_block(&BlockType::Parenthesis) {
            if let Ast::FunctionArgsBuild(vec) = &mut arguments_node {
                if vec.last().is_some_and(|last| *last == Ast::Empty) {
                    vec.pop();
                    // if !vec.is_empty() {
                    //     todo!("todo: warning, found extra comma")
                    // }
                }
                make_function(current, mem::take(vec));
                parse_block(tokens, p_state, current)
            } else {
                panic!("a function args build cannot be dismissed as root");
            }
        } else {
            Err(BlockType::Parenthesis.mismatched_err_end(location))
        }
    } else {
        let mut parenthesized_block = Ast::Empty;
        parse_block(tokens, p_state, &mut parenthesized_block)?;
        if p_state.pop_and_compare_block(&BlockType::Parenthesis) {
            current
                .push_block_as_leaf(ParensBlock::make_parens_ast(parenthesized_block))
                .map_err(|err| location.into_error(err))?;
            parse_block(tokens, p_state, current)
        } else {
            Err(BlockType::Parenthesis.mismatched_err_end(location))
        }
    }
}

/// Handler for `;`
///
/// Pushes a new empty node if needed.
fn handle_semicolon(current: &mut Ast) {
    if let Ast::BracedBlock(BracedBlock { elts, full }) = current
        && !*full
    {
        elts.push(Ast::Empty);
    } else if *current != Ast::Empty {
        *current = Ast::BracedBlock(BracedBlock {
            elts: vec![mem::take(current), Ast::Empty],
            full: false,
        });
    } else {
        /* last is empty: nothing to be done */
    }
}
