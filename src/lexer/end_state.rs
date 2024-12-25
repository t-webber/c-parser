use core::mem;

use super::numbers::literal_to_number;
use super::types::lexing_data::LexingData;
use super::types::lexing_state::{Ident, LexingStatus, SymbolStatus};
use super::types::tokens_types::Token;
use crate::errors::location::Location;

pub fn end_symbols(symbols: &mut SymbolStatus, lex_data: &mut LexingData, location: &Location) {
    let mut idx: usize = 0;
    while !symbols.is_empty() && idx <= 2 {
        idx += 1;
        if let Some((size, symbol)) = symbols.try_to_operator() {
            let token = Token::from_symbol(symbol, size, location);
            lex_data.push_token(token);
        } else {
            panic!(
                "This can't happen, as lex_data is not empty! LexingData: {:?}",
                &lex_data
            );
        }
    }
}

fn end_ident(literal: &mut Ident, lex_data: &mut LexingData, location: &Location) {
    if !literal.is_empty() {
        let possible_number = literal_to_number(lex_data, literal, location);
        match possible_number {
            None => {
                let token = Token::from_identifier(literal, location);
                lex_data.push_token(token);
            }
            Some(nb) => {
                let token = Token::from_number(nb, location);
                lex_data.push_token(token);
            }
        }
    }
}

pub fn end_current(status: &mut LexingStatus, lex_data: &mut LexingData, location: &Location) {
    match status {
        LexingStatus::Comment(_) | LexingStatus::Unset | LexingStatus::StartOfLine => return,
        LexingStatus::Symbols(symbol_status) => end_symbols(symbol_status, lex_data, location),
        LexingStatus::Identifier(ident) => end_ident(ident, lex_data, location),
        LexingStatus::Char(None) => {
            lex_data.push_err(location.to_error(
                "Found an empty char, but chars must contain one character. Did you mean '\\''?".to_owned()
            ));
        }
        LexingStatus::Char(Some(ch)) => lex_data.push_token(Token::from_char(*ch, location)),
        LexingStatus::Str(val) => {
            lex_data.push_token(Token::from_str(mem::take(val), location));
        }
    };
    *status = LexingStatus::Unset;
}
