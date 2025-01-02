//! Module to convert a list of [`Token`] into an [`Ast`].

extern crate alloc;
use alloc::vec::IntoIter;

use super::keyword::handle_keyword;
use super::state::ParsingState;
use super::symbols::handle_symbol;
use super::types::Ast;
use super::types::braced_blocks::BracedBlock;
use super::types::literal::{Literal, Variable};
use crate::errors::api::{Location, Res, SingleRes};
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
) -> SingleRes<()> {
    current
        .push_block_as_leaf(Ast::Leaf(lit))
        .map_err(|err| location.into_failure(err))?;
    parse_block(tokens, p_state, current)
}

/// Function to parse one node, and by recursivity, one block. At the end of the
/// block, this function stops and is recalled from [`parse_tokens`].
pub fn parse_block(
    tokens: &mut IntoIter<Token>,
    p_state: &mut ParsingState,
    current: &mut Ast,
) -> SingleRes<()> {
    tokens.next().map_or_else(
        || SingleRes::from(()),
        |token| {
            #[cfg(feature = "debug")]
            println!("Token = {token}\t & Current = {current}\n\t & State = {p_state:?}");
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
                TokenValue::Symbol(symbol) => {
                    handle_symbol(symbol, current, p_state, tokens, location)
                }
                TokenValue::Keyword(keyword) => {
                    handle_keyword(keyword, current, p_state, tokens, location)
                }
            }
        },
    )
}

/// Parses a list of tokens into an AST.
///
/// This function manages the blocks with successive calls and checks.
#[must_use]
#[inline]
pub fn parse_tokens(tokens: Vec<Token>) -> Res<Ast> {
    let mut nodes = vec![];
    let mut tokens_iter = tokens.into_iter();
    while tokens_iter.len() != 0 {
        let mut outer_node_block = Ast::BracedBlock(BracedBlock::default());
        let mut p_state = ParsingState::default();
        let res = parse_block(&mut tokens_iter, &mut p_state, &mut outer_node_block);
        if res.is_failure() {
            return res.into_res_with_discard(clean_nodes(nodes));
        }
        if p_state.has_opening_blocks() {
            return Res::from_errors(p_state.mismatched_error());
        }
        nodes.push(outer_node_block);
    }
    Res::from(clean_nodes(nodes))
}
