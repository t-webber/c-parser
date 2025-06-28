//! Module to convert a list of [`Token`] into an [`Ast`].

extern crate alloc;
use alloc::vec::IntoIter;

use super::keyword::handle_keyword;
use super::literal::Literal;
use super::modifiers::push::Push as _;
use super::state::ParsingState;
use super::symbols::api::BracedBlock;
use super::symbols::handle_symbol;
use super::tree::api::Ast;
use super::variable::Variable;
use crate::errors::api::{ErrorLocation, IntoError as _, Res};
use crate::lexer::api::{Token, TokenValue};

/// Deletes unnecessary outer block if necessary
fn clean_nodes(nodes: Vec<Ast>) -> Ast {
    let mut cleaned: Vec<Ast> = nodes
        .into_iter()
        .filter(|node| !node.is_empty())
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
    lit: Ast,
    location: ErrorLocation,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
) -> Res<()> {
    current
        .push_block_as_leaf(lit)
        .map_err(|err| location.into_crash(err))?;
    parse_block(tokens, p_state, current)
}

/// Function to parse one node, and by recursivity, one block. At the end of the
/// block, this function stops and is recalled from [`parse_tokens`].
pub fn parse_block(
    tokens: &mut IntoIter<Token>,
    p_state: &mut ParsingState,
    current: &mut Ast,
) -> Res<()> {
    tokens.next().map_or_else(
        || Res::from(()),
        |token| {
            #[cfg(feature = "debug")]
            println!("\n{:16} & {current}", format!("{token}"),);
            let (value, location) = token.into_value_location();
            match value {
                TokenValue::Char(ch) => handle_literal(
                    current,
                    Ast::Leaf(Literal::Char(ch)),
                    location,
                    p_state,
                    tokens,
                ),
                TokenValue::Ident(val) => handle_literal(
                    current,
                    Ast::Variable(Variable::from(val)),
                    location,
                    p_state,
                    tokens,
                ),
                TokenValue::Number(nb) => handle_literal(
                    current,
                    Ast::Leaf(Literal::Number(nb)),
                    location,
                    p_state,
                    tokens,
                ),
                TokenValue::Str(val) => handle_literal(
                    current,
                    Ast::Leaf(Literal::Str(val)),
                    location,
                    p_state,
                    tokens,
                ),
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
pub fn parse_tokens(tokens: Vec<Token>) -> Res<Ast> {
    let mut nodes = vec![];
    let mut errors = vec![];
    let mut tokens_iter = tokens.into_iter();
    while tokens_iter.len() != 0 {
        let mut outer_node_block = Ast::BracedBlock(BracedBlock::default());
        let mut p_state = ParsingState::default();
        let res = parse_block(&mut tokens_iter, &mut p_state, &mut outer_node_block);
        if res.has_failures() {
            errors.extend(res.into_errors());
            return Res::from((clean_nodes(nodes), errors));
        }
        errors.extend(res.into_errors());
        if p_state.has_opening_blocks() {
            errors.extend(p_state.mismatched_error());
            return Res::from((clean_nodes(nodes), errors));
        }
        nodes.push(outer_node_block);
    }
    Res::from((clean_nodes(nodes), errors))
}
