//! Module to convert a list of [`Token`] into an [`Ast`].

extern crate alloc;
use alloc::vec::IntoIter;

use super::keyword::handle_keyword;
use super::state::{BlockState, ParsingState};
use super::symbols::handle_symbol;
use super::types::Ast;
use super::types::blocks::BracedBlock;
use super::types::literal::{Literal, Variable};
use crate::errors::api::{CompileError, Location, Res};
use crate::lexer::api::{Token, TokenValue};

/// Deletes unnecessary outer block if necessary
fn clean_nodes(nodes: Vec<Ast>) -> Ast {
    let mut cleaned: Vec<Ast> = nodes
        .into_iter()
        .filter(|node| *node != Ast::Empty)
        .collect::<Vec<_>>();
    if cleaned.len() == 1 {
        cleaned.pop().expect("len == 1")
    } else {
        Ast::BracedBlock(BracedBlock {
            elts: cleaned,
            full: false,
        })
    }
}

/// Pushes a [`Literal`] into the [`Ast`]
fn handle_literal(
    current: &mut Ast,
    lit: Literal,
    location: Location,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
) -> Result<(), CompileError> {
    current
        .push_block_as_leaf(Ast::Leaf(lit))
        .map_err(|err| location.into_error(err))?;
    parse_block(tokens, p_state, current)
}

/// Returns errors for the unopened blocks (cf. [`BlockState`]).
fn mismatched_error(blocks: &mut Vec<BlockState>, location: &Location) -> Vec<CompileError> {
    let mut errors = vec![];
    while let Some(block) = blocks.pop() {
        errors.push(block.mismatched_err_begin(location.to_owned()));
    }
    errors
}

/// Function to parse one node, and by recursivity, one block. At the end of the
/// block, this function stops and is recalled from [`parse_tokens`].
pub fn parse_block(
    tokens: &mut IntoIter<Token>,
    p_state: &mut ParsingState,
    current: &mut Ast,
) -> Result<(), CompileError> {
    tokens.next().map_or(Ok(()), |token| {
        #[cfg(feature = "debug")]
        println!("Token = {token} & \tCurrent = {current}");
        let (value, location) = token.into_value_location();
        match value {
            TokenValue::Char(ch) => {
                handle_literal(current, Literal::Char(ch), location, p_state, tokens)
            }
            TokenValue::Ident(val) => handle_literal(
                current,
                Literal::Variable(Variable::from(val)),
                location,
                p_state,
                tokens,
            ),
            TokenValue::Number(nb) => {
                handle_literal(current, Literal::Number(nb), location, p_state, tokens)
            }
            TokenValue::Str(val) => {
                handle_literal(current, Literal::Str(val), location, p_state, tokens)
            }
            TokenValue::Symbol(symbol) => handle_symbol(symbol, current, p_state, tokens, location),
            TokenValue::Keyword(keyword) => {
                handle_keyword(keyword, current, p_state, tokens, location)
            }
        }
    })
}

/// Parses a list of tokens into an AST.
///
/// This function manages the blocks with successive calls and checks.
#[must_use]
#[inline]
pub fn parse_tokens(tokens: Vec<Token>, filename: String) -> Res<Ast> {
    let mut nodes = vec![];
    let mut tokens_iter = tokens.into_iter();
    while tokens_iter.len() != 0 {
        let mut outer_node_block = Ast::BracedBlock(BracedBlock::default());
        let mut p_state = ParsingState::default();
        if let Err(err) = parse_block(&mut tokens_iter, &mut p_state, &mut outer_node_block) {
            return Res::from_err(err);
        }
        if !p_state.opened_blocks.is_empty() {
            return Res::from_errors(mismatched_error(
                &mut p_state.opened_blocks,
                &Location::from(filename),
            ));
        }

        nodes.push(outer_node_block);
    }
    Res::from(clean_nodes(nodes))
}
