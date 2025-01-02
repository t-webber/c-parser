//! Module to handle symbols, convert them to operators and push them into the
//! [`Ast`].

extern crate alloc;
mod blocks;
mod handlers;
mod sort_symbols;

use alloc::vec::IntoIter;

use blocks::blocks_handler;
use sort_symbols::handle_one_symbol;

use super::parse_content::parse_block;
use super::state::ParsingState;
use super::types::Ast;
use crate::errors::api::{Location, SingleRes};
use crate::lexer::api::{Symbol, Token};

/// Main handler to push a symbol into an [`Ast`].
///
/// This function deals also the recursion calls.
pub fn handle_symbol(
    symbol: Symbol,
    current: &mut Ast,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: Location,
) -> SingleRes<()> {
    match handle_one_symbol(symbol, current) {
        Err(err) => SingleRes::from(location.into_failure(err)),
        Ok(Some(block_state)) => blocks_handler(current, tokens, p_state, location, &block_state),
        Ok(None) => parse_block(tokens, p_state, current),
    }
}
