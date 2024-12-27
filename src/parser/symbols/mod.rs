extern crate alloc;
mod blocks;
mod handlers;
mod sort_symbols;

use alloc::vec::IntoIter;

use blocks::blocks_handler;
use sort_symbols::handle_one_symbol;

use super::state::ParsingState;
use super::tree::node::Ast;
use crate::errors::api::{CompileError, Location};
use crate::lexer::api::{Symbol, Token};

pub fn handle_symbol(
    symbol: Symbol,
    current: &mut Ast,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: Location,
) -> Result<(), CompileError> {
    match handle_one_symbol(symbol, current) {
        Err(err) => Err(location.into_error(err)),
        Ok(block_state) => blocks_handler(current, tokens, p_state, location, &block_state),
    }
}
