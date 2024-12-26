extern crate alloc;
mod blocks;
mod handlers;
mod one_symbol;

use alloc::vec::IntoIter;

use blocks::blocks_handler;
use one_symbol::handle_one_symbol;

use super::state::ParsingState;
use super::tree::node::Node;
use crate::errors::compile::CompileError;
use crate::prelude::tokens_types::{Symbol, Token};
use crate::prelude::Location;

pub fn handle_symbol(
    symbol: Symbol,
    current: &mut Node,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: Location,
) -> Result<(), CompileError> {
    match handle_one_symbol(symbol, current) {
        Err(err) => Err(location.into_error(err)),
        Ok(block_state) => blocks_handler(current, tokens, p_state, location, &block_state),
    }
}
