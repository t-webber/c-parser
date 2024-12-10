use super::numbers::literal_to_number;
use super::parsing_state::{CharStatus, EscapeSequence, EscapeStatus, ParsingState};
use super::Token;
use crate::errors::location::Location;
use crate::to_error;
use core::mem;

pub fn end_both(p_state: &mut ParsingState, tokens: &mut Vec<Token>, location: &Location) {
    end_operator(p_state, tokens, location);
    end_literal(p_state, tokens, location);
}

pub fn end_escape_sequence(p_state: &mut ParsingState, location: &Location) {
    match p_state.escape.get_unsafe_sequence() {
        EscapeSequence::Unicode(value) => {
            assert!(value.len() <= 4, "Never should have pushed here");
            if value.len() < 4 {
                p_state.push_err(to_error!(
                    location,
                    "An escaped unicode must contain 4 hexadecimal digits, found only {}.",
                    value.len()
                ));
            } else if let Some(ch) = char::from_u32(
                value
                    .parse()
                    .expect("this is not possible because value is n"),
            ) {
                p_state.literal.push(ch);
            } else {
                p_state.push_err(to_error!(
                    location,
                    "Invalid unicode number, {} is not a valid unicode character.",
                    value
                ));
            }
        }
        EscapeSequence::Hexadecimal(value) => {
            assert!(value.len() <= 2, "Never should have pushed here");
            if value.len() < 2 {
                p_state.push_err(to_error!(
                    location,
                    "An escaped hexadecimal must contain 2 hexadecimal digits, found only {}.",
                    value.len()
                ));
            } else {
                let int = u8::from_str_radix(&value, 16)
                    .expect("We push only numeric so this doesn't happen");
                p_state.literal.push(int.into());
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
                p_state.literal.push(char::from(int as u8));
            } else {
                #[allow(clippy::string_slice, reason = "len = 3")]
                let int2 = u8::from_str_radix(&value[0..2], 8).unwrap();
                p_state.literal.push(char::from(int2));
                #[allow(clippy::indexing_slicing, reason = "len = 3")]
                p_state.literal.push(char::from(value.as_bytes()[2]));
            }
        }
    };
    p_state.escape = EscapeStatus::Trivial(false);
}

#[allow(clippy::needless_pass_by_ref_mut, clippy::todo)]
fn end_literal(p_state: &mut ParsingState, tokens: &mut Vec<Token>, location: &Location) {
    if !p_state.literal.is_empty() {
        match literal_to_number(&p_state.literal) {
            Ok(None) => tokens.push(Token::from_identifier(
                mem::take(&mut p_state.literal),
                p_state,
                location,
            )),
            Ok(Some(nb)) => tokens.push(Token::from_number(nb, p_state, location)),
            Err(err) => p_state.push_err(to_error!(location, "{err}")),
        }
    }
}

pub fn end_operator(p_state: &mut ParsingState, tokens: &mut Vec<Token>, location: &Location) {
    let mut idx: usize = 0;
    while !p_state.is_empty() && idx <= 2 {
        idx += 1;
        if let Some((size, symbol)) = p_state.try_to_operator() {
            tokens.push(Token::from_symbol(symbol, size, p_state, location));
        } else {
            panic!(
                "This can't happen, as p_state is not empty! ParsingState: {:?}",
                &p_state
            );
        }
    }
    assert!(p_state.is_empty(), "Not possible: executing 3 times the conversion, with stritcly decreasing number of non empty elements! This can't happen. ParsingState: {:?}", &p_state);
}

fn end_string(p_state: &mut ParsingState, tokens: &mut Vec<Token>, location: &Location) {
    if !p_state.literal.is_empty() {
        tokens.push(Token::from_str(
            mem::take(&mut p_state.literal),
            p_state,
            location,
        ));
    }
    assert!(p_state.literal.is_empty(), "Not possible: The string was just cleared, except if i am stupid and take doesn't clear ??!! ParsingState:{:?}", &p_state);
}

pub fn handle_double_quotes(
    p_state: &mut ParsingState,
    tokens: &mut Vec<Token>,
    location: &Location,
) {
    if p_state.double_quote {
        end_string(p_state, tokens, location);
        p_state.double_quote = false;
    } else {
        end_both(p_state, tokens, location);
        p_state.double_quote = true;
    }
}

pub fn handle_escaped(ch: char, p_state: &mut ParsingState, location: &Location) {
    match &p_state.escape {
        EscapeStatus::Sequence(_) => handle_escaped_sequence(ch, p_state, location),
        EscapeStatus::Trivial(_) => handle_one_escaped_char(ch, p_state, location),
    }
}

fn handle_escaped_sequence(ch: char, p_state: &mut ParsingState, location: &Location) {
    let escape_sequence = p_state.escape.get_unsafe_sequence();
    if !ch.is_ascii_hexdigit() || (escape_sequence.is_octal() && !ch.is_ascii_octdigit()) {
        end_escape_sequence(p_state, location);
    } else {
        let value = p_state.escape.get_unsafe_sequence_value_mut();
        value.push(ch);
        if value.len() == escape_sequence.max_len() {
            end_escape_sequence(p_state, location);
        }
    }
}

fn handle_one_escaped_char(ch: char, p_state: &mut ParsingState, location: &Location) {
    p_state.escape = EscapeStatus::Trivial(false);
    if p_state.double_quote || p_state.single_quote == CharStatus::Opened {
        match ch {
            'n' => p_state.literal.push('\n'),
            't' => p_state.literal.push('\t'),
            'r' => p_state.literal.push('\r'),
            '"' => p_state.literal.push('\"'),
            '\'' => p_state.literal.push('\''),
            'u' => p_state.escape = EscapeStatus::Sequence(EscapeSequence::Unicode(String::new())),
            'x' => {
                p_state.escape = EscapeStatus::Sequence(EscapeSequence::Hexadecimal(String::new()));
            }
            _ if ch.is_numeric() => {
                p_state.escape = EscapeStatus::Sequence(EscapeSequence::Octal(ch.to_string()));
            }
            '\\' => p_state.literal.push('\\'),
            '\0' => p_state.literal.push('\0'),
            _ => p_state.push_err(to_error!(
                location,
                "Character '{ch}' can not be escaped inside a string or a char."
            )),
        }
    } else {
        p_state.push_err(to_error!(
            location,
            "\\ escape character can only be used inside a string or char to espace a character."
        ));
    }
}

pub fn handle_single_quotes(p_state: &mut ParsingState, location: &Location) {
    match p_state.single_quote {
        CharStatus::Closed => p_state.single_quote = CharStatus::Opened,
        CharStatus::Written => p_state.single_quote = CharStatus::Closed,
        CharStatus::Opened => p_state.push_err(to_error!(
            location,
            "A char must contain exactly one element, but none where found. Did you mean '\\''?"
        )),
    }
}

pub fn handle_symbol(
    ch: char,
    p_state: &mut ParsingState,
    location: &Location,
    tokens: &mut Vec<Token>,
) {
    end_literal(p_state, tokens, location);
    if let Some((size, symbol)) = p_state.push(ch) {
        tokens.push(Token::from_symbol(symbol, size, p_state, location));
    }
}
