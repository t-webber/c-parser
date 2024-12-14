mod state;
mod symbols;
mod tree;
use crate::as_error;
use crate::errors::compile::Res;
use crate::errors::{compile::CompileError, location::Location};
use crate::lexer::api::tokens_types::{Symbol, Token, TokenValue};
extern crate alloc;
use alloc::vec::IntoIter;
use state::ParsingState;
use symbols::handle_symbol;
use tree::{Literal, Node};

fn handle_literal(
    current: &mut Node,
    leaf: Literal,
    location: Location,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
) -> Result<(), CompileError> {
    current
        .push_node_as_leaf(Node::Leaf(leaf))
        .map_err(|err| as_error!(location, "{err}"))?;
    parse_block(tokens, p_state, current)
}

fn parse_block(
    tokens: &mut IntoIter<Token>,
    p_state: &mut ParsingState,
    current: &mut Node,
) -> Result<(), CompileError> {
    if let Some(token) = tokens.next() {
        let (value, location) = token.into_value_location();
        match value {
            TokenValue::Char(ch) => {
                handle_literal(current, Literal::Char(ch), location, p_state, tokens)?;
            }
            TokenValue::Identifier(val) => {
                handle_literal(current, Literal::Variable(val), location, p_state, tokens)?;
            }
            TokenValue::Number(nb) => {
                handle_literal(current, Literal::Number(nb), location, p_state, tokens)?;
            }
            TokenValue::Str(val) => {
                handle_literal(current, Literal::Str(val), location, p_state, tokens)?;
            }
            TokenValue::Symbol(Symbol::Colon) if p_state.wanting_colon => {
                p_state.wanting_colon = false;
            }
            TokenValue::Symbol(symbol) => {
                handle_symbol(&symbol, current, p_state, tokens, location)?;
            }
            TokenValue::Keyword(_) => todo!(),
        }
    }
    Ok(())
}

pub fn parse_tokens(tokens: Vec<Token>) -> Res<Node> {
    let mut nodes = vec![];
    let mut errors = vec![];
    let mut tokens_iter = tokens.into_iter();
    while tokens_iter.len() != 0 {
        let mut outer_node_block = Node::default();
        let mut p_state = ParsingState::default();
        if let Err(err) = parse_block(&mut tokens_iter, &mut p_state, &mut outer_node_block) {
            errors.push(err);
        }
        nodes.push(outer_node_block);
    }
    Res::from((Node::Block(nodes), errors))
}
