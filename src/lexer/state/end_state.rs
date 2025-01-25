//! Module to define the functions called when the state is changed
//!
//! When the [`LexingState`] is changed, the buffers need to be emptied,
//! transformed into [`Token`] and pushed into [`LexingData`]. This is the goal
//! of this module.

use core::mem;

use crate::errors::api::{IntoError as _, LocationPointer};
use crate::lexer::numbers::api::literal_to_number;
use crate::lexer::state::api::{LexingState, SymbolState};
use crate::lexer::types::api::{Ident, LexingData, Token};

/// Ends the current state, and set current state to unset.
pub fn end_current(
    lex_state: &mut LexingState,
    lex_data: &mut LexingData,
    location: &LocationPointer,
) {
    match lex_state {
        LexingState::Comment(_) | LexingState::Unset | LexingState::StartOfLine => return,
        LexingState::Symbols(symbol_state) => end_symbols(symbol_state, lex_data, location),
        LexingState::Ident(ident) => end_ident(ident, lex_data, location),
        LexingState::Char(None) => {
            lex_data.push_err(
                location.to_fault(
                    "Found an empty char, but chars must contain one character. Did you mean '\\''?".to_owned(),
                ),
            );
        }
        LexingState::Char(Some(ch)) => lex_data.push_token(Token::from_char(*ch, location)),
        LexingState::Str(_) => {
            if let LexingState::Str((string, initial_location)) = mem::take(lex_state) {
                lex_data.push_token(Token::from_str(string, initial_location, location));
            } else {
                panic!()
            }
        }
    };
    *lex_state = LexingState::Unset;
}

/// Parses and pushes `literal` to the list of tokens in [`LexingData`].
///
/// This functions works out if the literal is an identifier or a number
/// constant and pushes the corresponding token. If it is a number, it is parsed
/// into its value.
fn end_ident(literal: &mut Ident, lex_data: &mut LexingData, location: &LocationPointer) {
    debug_assert!(!literal.is_empty(), "initialised with one");
    let possible_number = literal_to_number(lex_data, literal, location);
    match possible_number {
        None => {
            if !literal.first().unwrap_or('0').is_ascii_digit() {
                let token = Token::from_identifier(lex_data, literal, location);
                lex_data.push_token(token);
            }
        }
        Some(nb) => {
            let token = Token::from_number(nb, location);
            lex_data.push_token(token);
        }
    }
}

/// Ends the state for symbols.
pub fn end_symbols(
    symbols: &mut SymbolState,
    lex_data: &mut LexingData,
    location: &LocationPointer,
) {
    for _ in 0u32..3u32 {
        if symbols.is_empty() {
            break;
        }
        let symbol_len = symbols.len();
        if let Some((tok_len, symbol)) = symbols.try_to_operator(lex_data, location) {
            let token = Token::from_symbol_with_offset(symbol, tok_len, symbol_len, location);
            lex_data.push_token(token);
        } else {
            /* This happens when the 3 characters formed a trigraph. If this
             * is the case, they were ignored. It only happens in this case
             * because symbols is not empty. */
            //TODO: the characters are meant to be printed as they are
            //TODO: it is only for a case not yet implemented: trigraphs inside
            // string literals.
        }
    }
}
