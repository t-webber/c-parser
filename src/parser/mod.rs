mod state;
mod symbols;
mod tree;
use crate::errors::compile::{as_error, CompileError};
use crate::errors::location::Location;
use crate::errors::result::Res;
use crate::lexer::api::tokens_types::{Token, TokenValue};
extern crate alloc;
use alloc::vec::IntoIter;

use state::ParsingState;
use symbols::handle_symbol;
use tree::node::Node;
use tree::Literal;

fn handle_literal(
    current: &mut Node,
    lit: Literal,
    location: Location,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
) -> Result<(), CompileError> {
    current
        .push_block_as_leaf(Node::Leaf(lit))
        .map_err(|err| as_error!(location, "{err}"))?;
    parse_block(tokens, p_state, current)
}

fn parse_block(
    tokens: &mut IntoIter<Token>,
    p_state: &mut ParsingState,
    current: &mut Node,
) -> Result<(), CompileError> {
    tokens.next().map_or(Ok(()), |token| {
        let (value, location) = token.into_value_location();
        match value {
            TokenValue::Char(ch) => {
                handle_literal(current, Literal::Char(ch), location, p_state, tokens)
            }
            TokenValue::Identifier(val) => {
                handle_literal(current, Literal::Variable(val), location, p_state, tokens)
            }
            TokenValue::Number(nb) => {
                handle_literal(current, Literal::Number(nb), location, p_state, tokens)
            }
            TokenValue::Str(val) => {
                handle_literal(current, Literal::Str(val), location, p_state, tokens)
            }
            TokenValue::Symbol(symbol) => {
                handle_symbol(&symbol, current, p_state, tokens, location)
            }
            TokenValue::Keyword(key) => todo!("{key:?}"),
        }
    })
}

#[must_use]
#[inline]
pub fn parse_tokens(tokens: Vec<Token>) -> Res<Node> {
    let mut nodes = vec![];
    let mut tokens_iter = tokens.into_iter();
    while tokens_iter.len() != 0 {
        let mut outer_node_block = Node::default();
        let mut p_state = ParsingState::default();
        if let Err(err) = parse_block(&mut tokens_iter, &mut p_state, &mut outer_node_block) {
            return Res::from_err(err);
        }
        nodes.push(outer_node_block);
    }
    Res::from(Node::Block(nodes))
}
