//! Module to define the state and handlers for escaped characters and
//! sequences.

use super::api::LexingState;
use crate::errors::api::Location;
use crate::lexer::types::api::{EscapeSequence, LexingData};

/// Used to store the current escape state and the escape sequence values if
/// needed.
#[derive(Debug, PartialEq, Eq)]
pub enum EscapeState {
    /// Escape opened, but no characters were found yet.
    False,
    /// Reading an escape sequence.
    Sequence(EscapeSequence),
    /// Escape opened and found 1 character after it.
    Single,
}

/// Converts the full escape sequence into a char.
///
/// This function also checks that the right number of digits were given after
/// the prefix, and that the value is correct.
fn end_escape_sequence(
    lex_data: &mut LexingData,
    location: &Location,
    sequence: &EscapeSequence,
) -> Result<char, ()> {
    match sequence {
        EscapeSequence::ShortUnicode(value) => {
            expect_max_length(4, value);
            expect_min_length(lex_data, 4, value, location, sequence)?;
            char::from_u32(u32::from_str_radix(value, 16).expect("max 8 chars and radix valid"))
                .ok_or_else(|| {
                    //TODO: this should be a warning, and push the characters as raw
                    lex_data
                        .push_err(location.to_failure("Invalid escape character code".to_owned()));
                })
        }
        EscapeSequence::Unicode(value) => {
            if value.len() <= 4 {
                lex_data.push_err(location.to_failure(format!(
                    "Invalid escaped unicode number: An escaped big unicode must contain 8 hexadecimal digits, found only {}. Did you mean to use lowercase \\u?",
                    value.len()
                )));
                return Err(());
            }
            expect_max_length(8, value);
            expect_min_length(lex_data, 8, value, location, sequence)?;
            Ok(
                char::from_u32(
                    u32::from_str_radix(value, 16).expect("max 8 chars and radix valid"),
                )
                .expect("never invalid"),
            )
        }
        EscapeSequence::Hexadecimal(value) => {
            expect_max_length(3, value);
            expect_min_length(lex_data, 2, value, location, sequence)?;
            let int =
                u8::from_str_radix(value, 16).expect("We push only numeric so this doesn't happen");
            Ok(int.into())
        }
        EscapeSequence::Octal(value) => {
            expect_max_length(3, value);
            expect_min_length(lex_data, 1, value, location, sequence)?;
            let int =
                u32::from_str_radix(value, 8).expect("Max 3 digits, so value <= 0777 & radix < 32");
            assert!(int <= 0o377, "unreachable: should never have pushed");
            #[expect(clippy::as_conversions, clippy::cast_possible_truncation)]
            Ok(char::from(int as u8))
        }
    }
}

/// Returns the maximum number of characters expected after the escape sequence
/// prefix.
fn expect_max_length(size: usize, value: &str) {
    assert!(value.len() <= size, "Never should have pushed here");
}

/// Returns the minimum number of characters expected after the escape sequence
/// prefix.
fn expect_min_length(
    lex_data: &mut LexingData,
    size: usize,
    value: &str,
    location: &Location,
    sequence: &EscapeSequence,
) -> Result<(), ()> {
    let len = value.len();
    if len < size {
        lex_data.push_err(location.to_failure(format!(
            "Invalid escaped {} number: must contain 4 digits, but found only {}",
            sequence.repr(),
            len,
        )));
        return Err(());
    }
    Ok(())
}

/// Handle character in a escape context.
///
/// This function pushes the characters in escaped sequences or characters, and
/// pushes the character resulting of the escaped sequence into the string or
/// char.
///
/// # Note
///
/// When a escape sequence overflows, the last characters are considered normal
/// characters. For instance \776 is an escape sequence with octal code 77, and
/// a normal character 6. The last is also pushed in this function, with an
/// error
pub fn handle_escape(
    ch: char,
    lex_state: &mut LexingState,
    lex_data: &mut LexingData,
    escape_state: &mut EscapeState,
    location: &Location,
) {
    let (processed, failed) = push_char_in_escape(ch, lex_data, escape_state, location);
    if let Some(escaped) = processed {
        *escape_state = EscapeState::False;
        match lex_state {
            LexingState::Char(None) => *lex_state = LexingState::Char(Some(escaped)),
            LexingState::Str((val, _)) => val.push(escaped),
            LexingState::Char(_)
            | LexingState::Comment(_)
            | LexingState::Ident(_)
            | LexingState::StartOfLine
            | LexingState::Symbols(_)
            | LexingState::Unset => panic!("this can't happen, see match above"),
        }
    }
    if let Some(last) = failed {
        assert!(*escape_state == EscapeState::False, "");
        match lex_state {
                    LexingState::Char(None) => panic!(),
                    LexingState::Char(Some(_)) => lex_data.push_err_without_fail(location.to_failure("Escape sequence was too long. Only first 2 digits were taken, thus doesn't fit into a char.".to_owned())),
                    LexingState::Str((val, _)) => val.push(last),
                    LexingState::Comment(_) | LexingState::Ident(_) | LexingState::StartOfLine | LexingState::Symbols(_) | LexingState::Unset => panic!("this can't happen, see match above"),
                }
    }
}

/// Parses the token following the escape character. It determines whether it is
/// a escape sequence (in which case waiting for the next characters is
/// necessary) or a one character escape (in which case this function returns
/// the appropriate character).
fn handle_escape_one_char(
    ch: char,
    lex_data: &mut LexingData,
    escape_state: &mut EscapeState,
    location: &Location,
) -> Option<char> {
    *escape_state = EscapeState::False;
    match ch {
        '\0' => Some('\0'),
        'a' => Some('\u{0007}'),  // alert (beep, bell)
        'b' => Some('\u{0008}'),  // backspace
        't' => Some('\u{0009}'),  // horizontal tab
        'n' => Some('\u{000A}'),  // newline (line feed)
        'v' => Some('\u{000B}'),  // vertical tab
        'f' => Some('\u{000C}'),  // formfeed page break
        'r' => Some('\u{000D}'),  // carriage return
        'e' => Some('\u{001B}'),  // escape character
        '"' => Some('\u{0022}'),  // double quotation mark
        '\'' => Some('\u{0027}'), // apostrophe or single quotation mark
        '?' => Some('\u{003F}'),  // question mark (used to avoid trigraphs)
        '\\' => Some('\u{005C}'), // backslash
        'u' => {
            *escape_state = EscapeState::Sequence(EscapeSequence::ShortUnicode(String::new()));
            None
        }
        'U' => {
            *escape_state = EscapeState::Sequence(EscapeSequence::Unicode(String::new()));
            None
        }
        'x' => {
            *escape_state = EscapeState::Sequence(EscapeSequence::Hexadecimal(String::new()));
            None
        }
        _ if ch.is_numeric() => {
            *escape_state = EscapeState::Sequence(EscapeSequence::Octal(ch.to_string()));
            None
        }
        _ => {
            lex_data.push_err(location.to_failure(format!(
                "Character '{ch}' can not be escaped, even inside a string or a char.",
            )));
            None
        }
    }
}

/// See [`end_escape_sequence`].
fn handle_escaped_sequence(
    ch: char,
    escape_sequence: &mut EscapeSequence,
    lex_data: &mut LexingData,
    location: &Location,
) -> (Option<char>, Option<char>) {
    let octal = escape_sequence.is_octal();
    if !ch.is_ascii_hexdigit() || (octal && !ch.is_ascii_octdigit()) {
        (
            end_escape_sequence(lex_data, location, escape_sequence).ok(),
            None,
        )
    } else {
        let value = escape_sequence.value_mut();
        value.push(ch);
        let second = if octal && value.parse().map_or_else(|_| true, |nb: u32| nb >= 256) {
            value.pop()
        } else {
            None
        };
        if value.len() == escape_sequence.max_len() || second.is_some() {
            (
                end_escape_sequence(lex_data, location, escape_sequence).ok(),
                second,
            )
        } else {
            (None, second)
        }
    }
}

/// Pushed a character into an escape state, whatever the escape state.
fn push_char_in_escape(
    ch: char,
    lex_data: &mut LexingData,
    escape_state: &mut EscapeState,
    location: &Location,
) -> (Option<char>, Option<char>) {
    match escape_state {
        EscapeState::Sequence(escape_sequence) => {
            handle_escaped_sequence(ch, escape_sequence, lex_data, location)
        }
        EscapeState::Single => (
            handle_escape_one_char(ch, lex_data, escape_state, location),
            None,
        ),
        EscapeState::False => panic!("never called"),
    }
}
