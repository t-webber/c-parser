use super::lexing_state::{CharStatus, EscapeSequence, EscapeStatus, ParsingState};
use super::numbers::literal_to_number;
use super::{Token, TokenValue};
use crate::errors::location::Location;
use crate::to_error;
use core::mem;

pub fn end_both(lex_state: &mut ParsingState, location: &Location) {
    end_operator(lex_state, location);
    end_literal(lex_state, location);
}

pub fn end_escape_sequence(lex_state: &mut ParsingState, location: &Location) {
    match lex_state.escape.get_unsafe_sequence() {
        EscapeSequence::Unicode(value) => {
            assert!(value.len() <= 4, "Never should have pushed here");
            if value.len() < 4 {
                lex_state.push_err(to_error!(
                    location,
                    "An escaped unicode must contain 4 hexadecimal digits, found only {}.",
                    value.len()
                ));
            } else if let Some(ch) = char::from_u32(
                value
                    .parse()
                    .expect("this is not possible because value is n"),
            ) {
                lex_state.literal.push(ch);
            } else {
                lex_state.push_err(to_error!(
                    location,
                    "Invalid unicode number, {} is not a valid unicode character.",
                    value
                ));
            }
        }
        EscapeSequence::Hexadecimal(value) => {
            assert!(value.len() <= 2, "Never should have pushed here");
            if value.len() < 2 {
                lex_state.push_err(to_error!(
                    location,
                    "An escaped hexadecimal must contain 2 hexadecimal digits, found only {}.",
                    value.len()
                ));
            } else {
                let int = u8::from_str_radix(&value, 16)
                    .expect("We push only numeric so this doesn't happen");
                lex_state.literal.push(int.into());
            }
        }
        #[allow(clippy::unwrap_used, reason = "radix valid")]
        #[allow(
            clippy::as_conversions,
            clippy::cast_possible_truncation,
            reason = "int < 255"
        )]
        EscapeSequence::Octal(value) => {
            assert!(value.len() <= 3, "Never should have pushed here");
            assert!(!value.is_empty(), "");
            let int = u32::from_str_radix(&value, 8).unwrap();
            if value.len() < 3 || int <= 0o377 {
                lex_state.literal.push(char::from(int as u8));
            } else {
                #[allow(clippy::string_slice, reason = "len = 3")]
                let int2 = u8::from_str_radix(&value[0..2], 8).unwrap();
                lex_state.literal.push(char::from(int2));
                #[allow(clippy::indexing_slicing, reason = "len = 3")]
                lex_state.literal.push(char::from(value.as_bytes()[2]));
            }
        }
    };
    lex_state.escape = EscapeStatus::Trivial(false);
}

#[allow(clippy::needless_pass_by_ref_mut, clippy::todo)]
fn end_literal(lex_state: &mut ParsingState, location: &Location) {
    if !lex_state.literal.is_empty() {
        let possible_number = literal_to_number(lex_state, location);
        match possible_number {
            None => {
                let token = Token::from_identifier(mem::take(&mut lex_state.literal), location);
                lex_state.push_token(token);
            }
            Some(nb) => {
                let token = Token::from_number(nb, location);
                lex_state.push_token(token);
            }
        }
    }
}

pub fn end_operator(lex_state: &mut ParsingState, location: &Location) {
    let mut idx: usize = 0;
    while !lex_state.is_empty() && idx <= 2 {
        idx += 1;
        if let Some((size, symbol)) = lex_state.try_to_operator() {
            let token = Token::from_symbol(symbol, size, location);
            lex_state.push_token(token);
        } else {
            panic!(
                "This can't happen, as lex_state is not empty! ParsingState: {:?}",
                &lex_state
            );
        }
    }
    assert!(lex_state.is_empty(), "Not possible: executing 3 times the conversion, with stritcly decreasing number of non empty elements! This can't happen. ParsingState: {:?}", &lex_state);
}

fn end_string(lex_state: &mut ParsingState, location: &Location) {
    if !lex_state.literal.is_empty() {
        if let Some(last_token) = lex_state.pop_token() {
            if let TokenValue::Str(last_str) = last_token.value {
                let new_token =
                    Token::from_str(last_str + &mem::take(&mut lex_state.literal), location);
                lex_state.push_token(new_token);
                return;
            }
        }
        let token = Token::from_str(mem::take(&mut lex_state.literal), location);
        lex_state.push_token(token);
    }
    assert!(lex_state.literal.is_empty(), "Not possible: The string was just cleared, except if i am stupid and take doesn't clear ??!! ParsingState:{:?}", &lex_state);
}

pub fn handle_double_quotes(lex_state: &mut ParsingState, location: &Location) {
    if lex_state.double_quote {
        end_string(lex_state, location);
        lex_state.double_quote = false;
    } else {
        end_both(lex_state, location);
        lex_state.double_quote = true;
    }
}

pub fn handle_escaped(ch: char, lex_state: &mut ParsingState, location: &Location) {
    match &lex_state.escape {
        EscapeStatus::Sequence(_) => handle_escaped_sequence(ch, lex_state, location),
        EscapeStatus::Trivial(_) => handle_one_escaped_char(ch, lex_state, location),
    }
}

fn handle_escaped_sequence(ch: char, lex_state: &mut ParsingState, location: &Location) {
    let escape_sequence = lex_state.escape.get_unsafe_sequence();
    if !ch.is_ascii_hexdigit() || (escape_sequence.is_octal() && !ch.is_ascii_octdigit()) {
        end_escape_sequence(lex_state, location);
    } else {
        let value = lex_state.escape.get_unsafe_sequence_value_mut();
        value.push(ch);
        if value.len() == escape_sequence.max_len() {
            end_escape_sequence(lex_state, location);
        }
    }
}

fn handle_one_escaped_char(ch: char, lex_state: &mut ParsingState, location: &Location) {
    lex_state.escape = EscapeStatus::Trivial(false);
    if lex_state.double_quote || lex_state.single_quote == CharStatus::Opened {
        match ch {
            'n' => lex_state.literal.push('\n'),
            't' => lex_state.literal.push('\t'),
            'r' => lex_state.literal.push('\r'),
            '"' => lex_state.literal.push('\"'),
            '\'' => lex_state.literal.push('\''),
            'u' => {
                lex_state.escape = EscapeStatus::Sequence(EscapeSequence::Unicode(String::new()));
            }
            'x' => {
                lex_state.escape =
                    EscapeStatus::Sequence(EscapeSequence::Hexadecimal(String::new()));
            }
            _ if ch.is_numeric() => {
                lex_state.escape = EscapeStatus::Sequence(EscapeSequence::Octal(ch.to_string()));
            }
            '\\' => lex_state.literal.push('\\'),
            '\0' => lex_state.literal.push('\0'),
            _ => lex_state.push_err(to_error!(
                location,
                "Character '{ch}' can not be escaped inside a string or a char."
            )),
        }
    } else {
        lex_state.push_err(to_error!(
            location,
            "\\ escape character can only be used inside a string or char to espace a character."
        ));
    }
}

pub fn handle_single_quotes(lex_state: &mut ParsingState, location: &Location) {
    match lex_state.single_quote {
        CharStatus::Closed => lex_state.single_quote = CharStatus::Opened,
        CharStatus::Written => lex_state.single_quote = CharStatus::Closed,
        CharStatus::Opened => lex_state.push_err(to_error!(
            location,
            "A char must contain exactly one element, but none where found. Did you mean '\\''?"
        )),
    }
}

pub fn handle_symbol(ch: char, lex_state: &mut ParsingState, location: &Location) {
    end_literal(lex_state, location);
    if let Some((size, symbol)) = lex_state.push(ch) {
        let token = Token::from_symbol(symbol, size, location);
        lex_state.push_token(token);
    }
}
