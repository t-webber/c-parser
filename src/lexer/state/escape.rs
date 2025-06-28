//! Module to define the state and handlers for escaped characters and
//! sequences.

use super::api::LexingState;
use crate::errors::api::{IntoError as _, LocationPointer};
use crate::lexer::types::api::{EscapeSequence, LexingData};

/// Used to describe the return status after lexing an [`EscapeSequence`];
#[derive(Debug)]
enum EscapeSequenceReturnState {
    /// Lexing tried to lex the whole sequence but failed
    Error,
    /// Lexing failed and an extra character was found
    ErrorOverflow(char, usize),
    /// Lexing still in progress: nothing to do
    None,
    /// Lexing was successful and ended
    Value(char),
    /// An additional character was found
    ValueOverflow(char, char, usize),
}

impl EscapeSequenceReturnState {
    /// Returns the overflowed additional [`char`] if possible
    const fn as_overflow(&self) -> Option<(char, usize)> {
        match self {
            Self::Error | Self::None | Self::Value(_) => None,
            Self::ErrorOverflow(ch, len) | Self::ValueOverflow(_, ch, len) => Some((*ch, *len)),
        }
    }
    /// Returns the value if possible
    const fn as_value(&self) -> Option<char> {
        match self {
            Self::Error | Self::None | Self::ErrorOverflow(..) => None,
            Self::Value(value) | Self::ValueOverflow(value, ..) => Some(*value),
        }
    }
}

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
///
/// # Note
///
/// `char::from_u32(i)` returns an error iff `i >= 0x110000 || (i >= 0xD800 && i
/// < 0xE000)`.
fn end_escape_sequence(
    lex_data: &mut LexingData,
    location: &LocationPointer,
    sequence: &EscapeSequence,
    last: bool,
) -> Result<char, ()> {
    match sequence {
        EscapeSequence::ShortUnicode(value) => {
            expect_max_length(4, value);
            expect_min_length(lex_data, 4, value, location, sequence)?;
            debug_assert!(last, "len = 4");
            char::from_u32(u32::from_str_radix(value, 16).expect("max 8 chars and radix valid"))
                .ok_or_else(|| {
                    lex_data.push_err(
                        location
                            // value.len() == 4 and we add 2 for the prefix
                            .to_past(6, 5)
                            .into_fault("Invalid escape character code".to_owned()),
                    );
                })
        }
        EscapeSequence::Unicode(value) => {
            if value.len() <= 4 {
                let len2 = value.len().checked_add(2).expect("len <= 8");
                lex_data.push_err(location
                    .to_past(len2, len2)
                    .to_fault(format!(
                        "Invalid escaped unicode number: An escaped big unicode must contain 8 hexadecimal digits, found only {}. Did you mean to use lowercase \\u?",
                        value.len()
                    )));
                return Err(());
            }
            expect_max_length(8, value);
            expect_min_length(lex_data, 8, value, location, sequence)?;
            debug_assert!(last, "len = 4");
            char::from_u32(u32::from_str_radix(value, 16).expect("max 4 chars and radix valid"))
                .ok_or_else(|| {
                    lex_data.push_err(
                        location
                            // value.len() == 8 and we add 2 for the prefix
                            .to_past(10, 9)
                            .to_fault("Invalid escape character code".to_owned()),
                    );
                })
        }
        EscapeSequence::Hexadecimal(value) => {
            expect_max_length(2, value);
            expect_min_length(lex_data, 1, value, location, sequence)?;
            let int =
                u8::from_str_radix(value, 16).expect("We push only numeric so this doesn't happen");
            Ok(int.into())
        }
        EscapeSequence::Octal(value) => {
            expect_max_length(3, value);
            debug_assert!(!value.is_empty(), "initialise with len 1");
            let int =
                u32::from_str_radix(value, 8).expect("Max 3 digits, so value <= 0777 & radix < 32");
            debug_assert!(int <= 0o377, "unreachable: should never have pushed");
            #[expect(
                clippy::as_conversions,
                clippy::cast_possible_truncation,
                reason = "manually checked"
            )]
            Ok(char::from(int as u8))
        }
    }
}

/// Returns the maximum number of characters expected after the escape sequence
/// prefix.
fn expect_max_length(size: usize, value: &str) {
    debug_assert!(value.len() <= size, "Never should have pushed here");
}

/// Returns the minimum number of characters expected after the escape sequence
/// prefix.
fn expect_min_length(
    lex_data: &mut LexingData,
    size: usize,
    value: &str,
    location: &LocationPointer,
    sequence: &EscapeSequence,
) -> Result<(), ()> {
    let len = value.len();
    if len < size {
        let len2 = len.checked_add(2).expect("len <= 8");
        lex_data.push_err(location.to_past(len2, len2).to_fault(format!(
            "Invalid escaped {sequence} number: must contain at least {size} digits, but found only {len}"
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
    location: &LocationPointer,
) {
    debug_assert!(
        matches!(lex_state, LexingState::Char(None) | LexingState::Str(_)),
        "Can't happen because error raised on escape creation if wrong state."
    );
    let escape_return = push_char_in_escape(ch, lex_data, escape_state, location);
    if matches!(escape_return, EscapeSequenceReturnState::Error) {
        *escape_state = EscapeState::False;
        if let LexingState::Char(old @ None) = lex_state {
            *old = Some('\0');
        }
        return;
    }
    if let Some(escaped) = escape_return.as_value() {
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
    if let Some((last, len)) = escape_return.as_overflow() {
        *escape_state = EscapeState::False;
        match lex_state {
                    LexingState::Char(None) => {/* error raised above */ *lex_state = LexingState::Unset},
                    LexingState::Char(Some(_)) => lex_data.push_err(location.to_past(len, len).to_fault("Escape sequence was too long, creating more than one character, but it doesn't fit into a char.".to_owned())),
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
    location: &LocationPointer,
) -> Option<char> {
    *escape_state = EscapeState::False;
    match ch {
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
            lex_data.push_err(location.to_past(2, 1).to_warning(format!(
                "Escape ignored. Escaping character '{ch}' has no effect. Please remove the '\\'.",
            )));
            Some(ch)
        }
    }
}

/// See [`end_escape_sequence`].
fn handle_escaped_sequence(
    ch: char,
    escape_sequence: &mut EscapeSequence,
    lex_data: &mut LexingData,
    location: &LocationPointer,
) -> EscapeSequenceReturnState {
    let len = escape_sequence.len();
    let octal = escape_sequence.is_octal();
    if !ch.is_ascii_hexdigit() || (octal && !ch.is_ascii_octdigit()) {
        match end_escape_sequence(lex_data, location, escape_sequence, false) {
            Ok(escaped) => EscapeSequenceReturnState::ValueOverflow(escaped, ch, len),
            Err(()) => EscapeSequenceReturnState::ErrorOverflow(ch, len),
        }
    } else {
        let value = escape_sequence.value_mut();
        value.push(ch);
        let second =
            if octal && u32::from_str_radix(value, 8).expect("valid octal with len <= 3") >= 256 {
                value.pop()
            } else {
                None
            };
        let escaped_res = if value.len() == escape_sequence.max_len() || second.is_some() {
            end_escape_sequence(lex_data, location, escape_sequence, true).map_err(|()| false)
        } else {
            Err(true)
        };
        match (escaped_res, second) {
            (Ok(escaped), None) => EscapeSequenceReturnState::Value(escaped),
            (Ok(escaped), Some(failed)) =>
                EscapeSequenceReturnState::ValueOverflow(escaped, failed, len),
            (Err(true), None) => EscapeSequenceReturnState::None,
            (Err(false), None) => EscapeSequenceReturnState::Error,
            (_, Some(_)) => panic!("Octal can't have len < 1 and len = 3"),
        }
    }
}

/// Pushed a character into an escape state, whatever the escape state.
fn push_char_in_escape(
    ch: char,
    lex_data: &mut LexingData,
    escape_state: &mut EscapeState,
    location: &LocationPointer,
) -> EscapeSequenceReturnState {
    match escape_state {
        EscapeState::Sequence(escape_sequence) =>
            handle_escaped_sequence(ch, escape_sequence, lex_data, location),
        EscapeState::Single => handle_escape_one_char(ch, lex_data, escape_state, location)
            .map_or(EscapeSequenceReturnState::None, |escaped| {
                EscapeSequenceReturnState::Value(escaped)
            }),
        EscapeState::False => panic!("never called"),
    }
}
