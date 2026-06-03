//! Module to handle symbols, convert them to operators and push them into the
//! [`Ast`].

extern crate alloc;

pub use blocks::braced_blocks::BracedBlock;
pub use blocks::default::{FunctionCall, ListInitialiser};
pub use blocks::parens::{Cast, ParensBlock};

mod blocks;
mod handlers;
mod sort_symbols;

use alloc::vec::IntoIter;

use blocks::recursion::blocks_handler;
use lexer::{Symbol, Token};
use parse_content::ParseAction;
use sort_symbols::handle_one_symbol;
use state::ParsingState;
use crate::tree::Ast;

use crate::errors::{ErrorLocation, IntoError as _, Res};

/// Main handler to push a symbol into an [`Ast`].
///
/// This function deals also the recursion calls.
pub fn handle_symbol(
    symbol: Symbol,
    current: &mut Ast,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: ErrorLocation,
) -> Res<ParseAction> {
    match handle_one_symbol(symbol, current) {
        Err(err) => location.into_crash(err).into_res(),
        Ok(Some(block_state)) => blocks_handler(current, tokens, p_state, location, &block_state),
        Ok(None) => Res::ok(ParseAction::Continue),
    }
}
