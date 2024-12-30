use core::mem;

use super::super::numbers::api::literal_to_number;
use super::super::state::api::{LexingState, SymbolState};
use super::super::types::api::{Ident, LexingData, Token};
use crate::errors::api::Location;

pub fn end_current(state: &mut LexingState, lex_data: &mut LexingData, location: &Location) {
    match state {
        LexingState::Comment(_) | LexingState::Unset | LexingState::StartOfLine => return,
        LexingState::Symbols(symbol_state) => end_symbols(symbol_state, lex_data, location),
        LexingState::Identifier(ident) => end_ident(ident, lex_data, location),
        LexingState::Char(None) => {
            lex_data.push_err(
                location.to_error(
                    "Found an empty char, but chars must contain one character. Did you mean '\\''?".to_owned(),
                ),
            );
        }
        LexingState::Char(Some(ch)) => lex_data.push_token(Token::from_char(*ch, location)),
        LexingState::Str(val) => {
            lex_data.push_token(Token::from_str(mem::take(val), location));
        }
    };
    *state = LexingState::Unset;
}

fn end_ident(literal: &mut Ident, lex_data: &mut LexingData, location: &Location) {
    if !literal.is_empty() {
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
}

pub fn end_symbols(symbols: &mut SymbolState, lex_data: &mut LexingData, location: &Location) {
    let mut idx: usize = 0;
    while !symbols.is_empty() && idx <= 2 {
        idx += 1;
        if let Some((size, symbol)) = symbols.try_to_operator(lex_data, location) {
            let token = Token::from_symbol(symbol, size, location);
            lex_data.push_token(token);
        } else {
            /* This happens when the 3 characters formed a trigraph. If this
             * is the case, they were ignored. */
            //TODO: the characters are meant to be printed as they are
            //TODO: it is only for a case not yet implemented: trigraphs inside
            // string literals.
        }
    }
}
