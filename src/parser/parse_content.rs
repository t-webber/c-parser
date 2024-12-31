extern crate alloc;
use alloc::vec::IntoIter;

use super::keyword::handle_keyword;
use super::state::{BlockState, ParsingState};
use super::symbols::handle_symbol;
use super::types::Ast;
use super::types::blocks::Block;
use super::types::literal::{Literal, Variable};
use crate::errors::api::{CompileError, Location, Res};
use crate::lexer::api::{Token, TokenValue};

fn clean_nodes(nodes: Vec<Ast>) -> Ast {
    let mut cleaned: Vec<Ast> = nodes
        .into_iter()
        .filter(|node| *node != Ast::Empty)
        .collect::<Vec<_>>();
    if cleaned.len() == 1 {
        cleaned.pop().expect("len == 1")
    } else {
        Ast::Block(Block {
            elts: cleaned,
            full: false,
        })
    }
}

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

fn mismatched_error(
    blocks: &mut Vec<BlockState>,
    next_token: Option<Token>,
    filename: String,
) -> Vec<CompileError> {
    let mut errors = vec![];
    let location = next_token.map_or_else(
        || Location::from(filename),
        |token| token.into_value_location().1,
    );
    while let Some(block) = blocks.pop() {
        errors.push(block.mismatched_err_begin(location.clone()));
    }
    errors
}

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
            TokenValue::Identifier(val) => handle_literal(
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

#[must_use]
#[inline]
pub fn parse_tokens(tokens: Vec<Token>) -> Res<Ast> {
    let mut nodes = vec![];
    let filename = tokens.first().map(|node| node.get_location().to_filename());
    let mut tokens_iter = tokens.into_iter();
    while tokens_iter.len() != 0 {
        let mut outer_node_block = Ast::Block(Block::default());
        let mut p_state = ParsingState::default();
        if let Err(err) = parse_block(&mut tokens_iter, &mut p_state, &mut outer_node_block) {
            return Res::from_err(err);
        }
        if !p_state.opened_blocks.is_empty() {
            return Res::from_errors(mismatched_error(
                &mut p_state.opened_blocks,
                tokens_iter.next(),
                filename.expect("while loop never entered if tokens empty"),
            ));
        }

        nodes.push(outer_node_block);
    }
    Res::from(clean_nodes(nodes))
}
