use core::num::{IntErrorKind, ParseIntError};

use crate::errors::api::{CompileRes, ErrorLocation};
use crate::lexer::numbers::types::ERR_PREFIX;

/// Stores the data of an hexadecimal constant
#[derive(Default, Debug)]
pub struct HexFloatData {
    /// Decimal part of the constant, between the '.' and the 'p'
    pub decimal_part: String,
    /// Exponent part of the constant, after the 'p'
    pub exponent: String,
    /// Sign if found of the exponent
    ///
    /// - If a '+' is found after the 'p', ``exponent_neg = Some(false)``;
    /// - If a '-' is found after the 'p', ``exponent_neg = Some(true)``;
    /// - If a digit is found after the 'p', ``exponent_neg = None``.
    pub exponent_neg: Option<bool>,
    /// Integer part of the constant, before the '.'
    pub int_part: String,
    /// State of the parsing
    ///
    /// All the fields are set to default at the beginning, and when state
    /// changes, the fields begin receiving data, one by one.
    pub state: HexFloatParseState,
}

impl HexFloatData {
    /// Returns the exponent of the number constant.
    pub fn as_exp(&self) -> Result<u32, &'static str> {
        debug_assert!(
            !self.exponent.is_empty(),
            "Exponent not empty because exponent compulsory for float hexadecimals"
        );
        self.exponent.parse().map_err(|err: ParseIntError| {
            debug_assert!(matches!(err.kind(), IntErrorKind::PosOverflow), "none others possible");
            "Failed to parse exponent: too large"
        })
    }

    /// Pushes a character to the current state.
    fn push(&mut self, ch: char) {
        match self.state {
            HexFloatParseState::Int => self.int_part.push(ch),
            HexFloatParseState::Decimal => self.decimal_part.push(ch),
            HexFloatParseState::Exponent => self.exponent.push(ch),
        }
    }
}

/// Parsing state of the hexadecimal constant
///
/// The first part is the integer part, then the decimal part after a full stop,
/// and a exponent part after an exponent character ('p').
#[derive(Default, PartialEq, Eq, Debug)]
pub enum HexFloatParseState {
    /// Decimal part
    ///
    /// The part between the full stop and the exponent character 'p' (if they
    /// exist).
    Decimal,
    /// Exponent part
    ///
    /// Last part of the string, after the 'p' character.
    Exponent,
    /// Integer part
    ///
    /// First part of the string, before the full stop and the 'p' character.
    #[default]
    Int,
}

/// Parses an hexadecimal string by hand
///
/// # Returns
///
/// This function returns an [`HexFloatData`], that contains the different parts
/// of the number: the integer part, the decimal part and the exponent part.
///
/// For an hexadecimal C constant, the decimal part is prefix with the character
/// '.' and the exponent is prefixed with the letter `p`.
///
/// # Errors
///
/// This functions returns an error if
/// - multiple signs or full stops were found in the string,
/// - a non decimal digit was found in the exponent part,
///
/// # Examples
///
/// ```ignore
/// use crate::errors::location::LocationPointer;
///
/// assert!(
///     as_hex_float_data("fd.ep2", &LocationPointer::from(String::new()))
///         == Ok(HexFloatData {
///             int_part: "fd".to_owned(),
///             decimal_part: "e".to_owned(),
///             exponent: "2".to_owned(),
///             exponent_neg: None,
///             state: HexFloatParseState::Exponent
///         })
/// );
///
/// matches!(
///     as_hex_float_data("fd.ep++2", &LocationPointer::from(String::new())),
///     Err(_)
/// );
/// ```
///
/// # Note
///
/// There is never more than one sign symbol in a number constant, because the
/// second will always be interpreted as character: `1e+7+7` is read `(1e7)+7` .
pub fn as_hex_float_data(literal: &str, location: ErrorLocation) -> CompileRes<HexFloatData> {
    let mut float_parse = HexFloatData::default();
    for ch in literal.chars() {
        debug_assert!(
            !matches!(ch, '+' | '-') || float_parse.state == HexFloatParseState::Exponent,
            "+ or - always are after a p character in hex literal"
        );
        match ch {
            '-' => float_parse.exponent_neg = Some(true),
            '+' => float_parse.exponent_neg = Some(false),
            _ if float_parse.state == HexFloatParseState::Exponent && ch.is_ascii_digit() => float_parse.push(ch),
            _ if float_parse.state == HexFloatParseState::Exponent => {
                return Err(location.fail(format!(
                    "{ERR_PREFIX}invalid character for exponent. Expected an ascii digit, but found '{ch}'"
                )))
            }
            _ if ch.is_ascii_hexdigit() => float_parse.push(ch),
            '.' if float_parse.state == HexFloatParseState::Int => float_parse.state = HexFloatParseState::Decimal,
            'p' | 'P' => float_parse.state = HexFloatParseState::Exponent,
            _ => unreachable!("never happens: characters are all valid"),
        }
    }
    Ok(float_parse)
}
