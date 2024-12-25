use super::numbers::parse::safe_parse_int;
use super::types::escape_state::{EscapeSequence, EscapeStatus};
use super::types::lexing_data::LexingData;
use crate::errors::location::Location;

fn end_unicode_sequence(
    lex_data: &mut LexingData,
    value: &str,
    location: &Location,
) -> Result<char, ()> {
    safe_parse_int!(
        "Invalid escaped unicode number: ",
        u32,
        location,
        u32::from_str_radix(value, 16)
    )
    .map(char::from_u32)
    .ignore_overflow(value, location)
    .map_or_else(
        |err| {
            lex_data.push_err(err);
        },
        |val| val,
    )?
    .map_or_else(
        || {
            lex_data.push_err(location.to_error(format!(
                "Invalid escaped unicode number: {value} is not a valid unicode character.",
            )));
            Err(())
        },
        Ok,
    )
}

fn expect_min_length(
    lex_data: &mut LexingData,
    size: usize,
    value: &str,
    location: &Location,
    sequence: &EscapeSequence,
) -> Result<(), ()> {
    let len = value.len();
    if len < size {
        lex_data.push_err(location.to_error(format!(
            "Invalid escaped {} number: must contain 4 digits, but found only {}",
            sequence.repr(),
            len,
        )));
        return Err(());
    }
    Ok(())
}

fn expect_max_length(size: usize, value: &str) {
    assert!(value.len() <= size, "Never should have pushed here");
}

pub fn end_escape_sequence(
    lex_data: &mut LexingData,
    location: &Location,
    sequence: &EscapeSequence,
) -> Result<char, ()> {
    match &sequence {
        EscapeSequence::ShortUnicode(ref value) => {
            expect_max_length(4, value);
            expect_min_length(lex_data, 4, value, location, sequence)?;
            end_unicode_sequence(lex_data, value, location)
        }
        EscapeSequence::Unicode(ref value) => {
            if value.len() <= 4 {
                lex_data.push_err(location.to_error(format!(
                    "Invalid escaped unicode number: An escaped big unicode must contain 8 hexadecimal digits, found only {}. Did you mean to use lowercase \\u?",
                    value.len()
                )));
                Err(())?;
            }
            expect_max_length(8, value);
            expect_min_length(lex_data, 8, value, location, sequence)?;
            end_unicode_sequence(lex_data, value, location)
        }
        EscapeSequence::Hexadecimal(ref value) => {
            expect_max_length(3, value);
            expect_min_length(lex_data, 2, value, location, sequence)?;
            let int =
                u8::from_str_radix(value, 16).expect("We push only numeric so this doesn't happen");
            Ok(int.into())
        }
        EscapeSequence::Octal(ref value) => {
            expect_max_length(3, value);
            expect_min_length(lex_data, 1, value, location, sequence)?;
            let (int, small) = safe_parse_int!(
                "Invalid octal escape sequence :",
                u32,
                location,
                u32::from_str_radix(value, 8)
            )
            .ignore_overflow(value, location)
            .map_or_else(
                |err| lex_data.push_err(err),
                |int| (int, value.len() < 3 || int <= 0o377),
            )?;
            if small {
                #[allow(
                    clippy::as_conversions,
                    clippy::cast_possible_truncation,
                    reason = "int <= 255"
                )]
                Ok(char::from(int as u8))
            } else {
                #[allow(clippy::string_slice, reason = "len = 3")]
                safe_parse_int!(
                    "Invalid octal escape sequence: ",
                    u8,
                    location,
                    u8::from_str_radix(&value[0..2], 8)
                )
                .ignore_overflow(&value[0..2], location)
                .map_or_else(|err| lex_data.push_err(err), char::from)
            }
        }
    }
}

fn handle_escaped_sequence(
    ch: char,
    escape_sequence: &mut EscapeSequence,
    lex_data: &mut LexingData,
    location: &Location,
) -> Option<char> {
    if !ch.is_ascii_hexdigit() || (escape_sequence.is_octal() && !ch.is_ascii_octdigit()) {
        end_escape_sequence(lex_data, location, escape_sequence).ok()
    } else {
        let value = escape_sequence.value_mut();
        value.push(ch);
        if value.len() == escape_sequence.max_len() {
            end_escape_sequence(lex_data, location, escape_sequence).ok()
        } else {
            None
        }
    }
}

fn handle_escape_one_char(
    ch: char,
    lex_data: &mut LexingData,
    escape_status: &mut EscapeStatus,
    location: &Location,
) -> Option<char> {
    *escape_status = EscapeStatus::False;
    match ch {
        '\0' => Some('\0'),
        'a' => Some('\u{0007}'),  // alert (bepp, bell)
        'b' => Some('\u{0008}'),  // backspace
        't' => Some('\u{0009}'),  // horizontal tab
        'n' => Some('\u{000A}'),  // newline (line feed)
        'v' => Some('\u{000B}'),  // vertical tab
        'f' => Some('\u{000C}'),  // formfeed page break
        'r' => Some('\u{000D}'),  // carriage return
        'e' => Some('\u{001B}'),  // escape character
        '"' => Some('\u{0022}'),  // double quotation mark
        '\'' => Some('\u{0027}'), // apostrophe or single quotiation mark
        '?' => Some('\u{003F}'),  // question mark (used to avoid tigraphs)
        '\\' => Some('\u{005C}'), // backslash
        'u' => {
            *escape_status = EscapeStatus::Sequence(EscapeSequence::ShortUnicode(String::new()));
            None
        }
        'U' => {
            *escape_status = EscapeStatus::Sequence(EscapeSequence::Unicode(String::new()));
            None
        }
        'x' => {
            *escape_status = EscapeStatus::Sequence(EscapeSequence::Hexadecimal(String::new()));
            None
        }
        _ if ch.is_numeric() => {
            *escape_status = EscapeStatus::Sequence(EscapeSequence::Octal(ch.to_string()));
            None
        }
        _ => {
            lex_data.push_err(location.to_error(format!(
                "Character '{ch}' can not be escaped, even inside a string or a char.",
            )));
            None
        }
    }
}

pub fn handle_escape(
    ch: char,
    lex_data: &mut LexingData,
    escape_status: &mut EscapeStatus,
    location: &Location,
) -> Option<char> {
    match escape_status {
        EscapeStatus::Sequence(escape_sequence) => {
            handle_escaped_sequence(ch, escape_sequence, lex_data, location)
        }
        EscapeStatus::Single => handle_escape_one_char(ch, lex_data, escape_status, location),
        EscapeStatus::False => panic!("never called"),
    }
}
