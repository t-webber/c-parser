//! Handler for block character

extern crate alloc;
use alloc::vec::IntoIter;
use core::mem;

use super::braced_blocks::BracedBlock;
use super::default::ListInitialiser;
use super::parens::ParensBlock;
use crate::Res;
use crate::errors::api::{ErrorLocation, IntoError as _};
use crate::lexer::api::Token;
use crate::parser::keyword::control_flow::node::{
    switch_wanting_block, try_push_semicolon_control
};
use crate::parser::modifiers::functions::{CanMakeFnRes, MakeFunction as _};
use crate::parser::modifiers::list_initialiser::{
    apply_to_last_list_initialiser, can_push_list_initialiser
};
use crate::parser::modifiers::push::Push as _;
use crate::parser::operators::api::BinaryOperator;
use crate::parser::parse_content::{ParseAction, parse_block};
use crate::parser::state::{BlockType, ParsingState};
use crate::parser::tree::api::Ast;

/// State to indicate what needs to be done
#[derive(Debug)]
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
    location: ErrorLocation,
    block_state: &TodoBlock,
) -> Res<ParseAction> {
    match block_state {
        // semi-colon
        TodoBlock::SemiColon => {
            handle_semicolon(current);
            Res::from(ParseAction::Continue)
        }
        // parenthesis
        TodoBlock::CloseParens => {
            p_state.push_closing_block(BlockType::Parenthesis, location);
            Res::from(ParseAction::Stop)
        }
        TodoBlock::OpenParens => handle_parenthesis_open(current, p_state, tokens, location),
        // bracket
        TodoBlock::CloseBracket => {
            p_state.push_closing_block(BlockType::Bracket, location);
            Res::from(ParseAction::Stop)
        }
        TodoBlock::OpenBracket => {
            let mut bracket_node = Ast::Empty;
            p_state.push_ctrl_flow(false);
            parse_block(tokens, p_state, &mut bracket_node)?;
            if p_state.pop_ctrl_flow().is_none() {
                return Res::from(BlockType::Bracket.mismatched_err_end(location));
            }
            if p_state.pop_and_compare_block(&BlockType::Bracket) {
                if let Err(err) = current.push_op(BinaryOperator::ArraySubscript) {
                    Res::from(location.into_crash(err))
                } else {
                    current
                        .push_block_as_leaf(bracket_node)
                        .map_err(|err| location.into_crash(err))?;
                    Res::from(ParseAction::Continue)
                }
            } else {
                Res::from(BlockType::Bracket.mismatched_err_end(location))
            }
        }
        // brace
        TodoBlock::CloseBraceBlock
            if apply_to_last_list_initialiser(current, &|_, full| *full = true).is_none() =>
        {
            p_state.push_closing_block(BlockType::Brace, location);
            Res::from(ParseAction::Stop)
        }
        TodoBlock::OpenBraceBlock => match can_push_list_initialiser(current) {
            Err(op) => Res::from(location.into_crash(format!(
                "Found operator '{op}' applied on list initialiser '{{}}', but this is not allowed."
            ))),
            Ok(true) => {
                current
                    .push_block_as_leaf(Ast::ListInitialiser(ListInitialiser::default()))
                    .map_err(|err| location.into_crash(err))?;
                Res::from(ParseAction::Continue)
            }
            Ok(false) => handle_brace_block_open(current, tokens, p_state, location),
        },
        // others
        TodoBlock::CloseBraceBlock => Res::from(ParseAction::Continue),
    }
}

/// Handler for `{`
///
/// Deals with recursion and merges the braced-blocks
fn handle_brace_block_open(
    current: &mut Ast,
    tokens: &mut IntoIter<Token>,
    p_state: &mut ParsingState,
    location: ErrorLocation,
) -> Res<ParseAction> {
    let mut brace_block = Ast::BracedBlock(BracedBlock::default());
    p_state.push_ctrl_flow(switch_wanting_block(current));
    parse_block(tokens, p_state, &mut brace_block)?;
    if p_state.pop_ctrl_flow().is_none() {
        return Res::from(BlockType::Brace.mismatched_err_end(location));
    }
    if !p_state.pop_and_compare_block(&BlockType::Brace) {
        return Res::from(BlockType::Brace.mismatched_err_end(location));
    }
    if let Ast::BracedBlock(BracedBlock { full, .. }) = &mut brace_block {
        *full = true;
    } else {
        unreachable!("a block can't be changed to another node")
    }
    current
        .push_braced_block(brace_block)
        .map_err(|msg| location.into_crash(msg))?;
    Res::from(ParseAction::Continue)
}

/// Handler for `(`
///
/// Deals with recursion and pushes the function arguments if needed
fn handle_parenthesis_open(
    current: &mut Ast,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: ErrorLocation,
) -> Res<ParseAction> {
    match current.can_make_function() {
        CanMakeFnRes::CanMakeFn(variable_depth) =>
            make_function(current, p_state, tokens, location, variable_depth),
        CanMakeFnRes::None =>
            handle_non_function_parenthesis_open(current, p_state, tokens, location),
        CanMakeFnRes::TooDeep => Res::from(
            location.into_crash("Code to complex: AST to deep to fit depth in 32 bits.".to_owned()),
        ),
    }
}

/// Create a function for the found '('
///
/// Builds a function on a variable and adds its arguments.
fn make_function(
    current: &mut Ast,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: ErrorLocation,
    variable_depth: u32,
) -> Res<ParseAction> {
    let mut arguments_node = Ast::FunctionArgsBuild(vec![Ast::Empty]);
    p_state.push_ctrl_flow(false);
    parse_block(tokens, p_state, &mut arguments_node)?;
    if p_state.pop_ctrl_flow().is_none() {
        return Res::from(BlockType::Parenthesis.mismatched_err_end(location));
    }
    if p_state.pop_and_compare_block(&BlockType::Parenthesis) {
        if let Ast::FunctionArgsBuild(vec) = &mut arguments_node {
            let mut error = None;
            if vec.last().is_some_and(Ast::is_empty) {
                vec.pop();
                if !vec.is_empty() {
                    error = Some(
                        location.to_suggestion(
                            "Found extra comma in function argument list. Please remove the comma."
                                .to_owned(),
                        ),
                    );
                }
            }
            current.make_function(variable_depth, mem::take(vec));
            Res::from(ParseAction::Continue).add_err(error)
        } else {
            unreachable!("a function args build cannot be dismissed as root");
        }
    } else {
        Res::from(BlockType::Parenthesis.mismatched_err_end(location))
    }
}

/// Handles an opening '(', but when it can't be a function call.
fn handle_non_function_parenthesis_open(
    current: &mut Ast,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: ErrorLocation,
) -> Res<ParseAction> {
    let mut parenthesized_block = Ast::Empty;
    parse_block(tokens, p_state, &mut parenthesized_block)?;
    parenthesized_block.fill();
    if p_state.pop_and_compare_block(&BlockType::Parenthesis) {
        current
            .push_block_as_leaf(ParensBlock::make_parens_ast(parenthesized_block))
            .map_err(|err| location.into_crash(err))?;
        Res::from(ParseAction::Continue)
    } else {
        Res::from(BlockType::Parenthesis.mismatched_err_end(location))
    }
}

/// Handler for `;`
///
/// Pushes a new empty node if needed.
fn handle_semicolon(current: &mut Ast) {
    if try_push_semicolon_control(current) {
        return;
    }
    if let Ast::BracedBlock(BracedBlock { elts, full }) = current
        && !*full
    {
        if let Some(last) = elts.last_mut() {
            last.fill();
        }
        elts.push(Ast::Empty);
    } else if !current.is_empty() {
        *current = Ast::BracedBlock(BracedBlock {
            elts: vec![mem::take(current), Ast::Empty],
            full: false,
        });
    }
}
