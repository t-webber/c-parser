//! Module to parse hexadecimal-represented number constants

#![allow(clippy::arbitrary_source_item_ordering, reason = "macro usage")]

use core::num::{IntErrorKind, ParseIntError};

use crate::errors::api::{CompileRes, ErrorLocation, IntoError as _};
use crate::lexer::numbers::macros::parse_int_from_radix;
use crate::lexer::numbers::parse::OverParseRes;
use crate::lexer::numbers::types::arch_types::{
    Double, DoubleIntPart, Float, FloatIntPart, Int, Long, LongDouble, LongDoubleIntPart, LongLong, UInt, ULong, ULongLong
};
use crate::lexer::numbers::types::{ERR_PREFIX, Number, NumberType};

/// Implements the [`FloatingPoint`] for the floating-point types.
macro_rules! impl_floating_point {
    ($x:expr, $($type:ident)*) => {
        $(#[allow(clippy::as_conversions, clippy::cast_precision_loss, clippy::allow_attributes, reason="todo")]
        impl FloatingPoint<${concat($type, IntPart)}> for $type {
            const MANTISSA_SIZE: u32 = $x;

            type Unsigned = ${concat($type, IntPart)};


            fn from_unsigned(
                val: Self::Unsigned,
                overflow: &mut bool,
            ) -> Self {
                if val >= (2 as Self::Unsigned).pow(Self::MANTISSA_SIZE) {
                    *overflow = true;
                }
                val as Self
            }

            #[coverage(off)]
            fn from_usize(
                val: usize,
                overflow: &mut bool,
            ) -> Self {
                if val >= 2usize.pow(Self::MANTISSA_SIZE) {
                    *overflow = true;
                }
                val as Self
            }
        })*
    };
}

/// Parses the stringified version of a number into a [`HexFloatData`].
macro_rules! parse_hexadecimal_float {
    ($overflow:expr, $nb_type:ident, $float_parse:ident, $($t:ident)*) => {{
        #[expect(clippy::float_arithmetic, clippy::arithmetic_side_effects, clippy::as_conversions, reason="todo")]
        match $nb_type {
            $(NumberType::$t => {
                let int_part = $t::from_unsigned(
                    ${concat($t, IntPart)}::from_str_radix(&$float_parse.int_part, 16).unwrap(),
                    $overflow);
                let exponent = $t::from_unsigned((2 as ${concat($t, IntPart)}).pow($float_parse.as_exp()?), $overflow);
                let mut decimal_part: $t = 0.;
                for (idx, ch) in $float_parse.decimal_part.chars().enumerate() {
                    let digit_value = $t::from_unsigned(hex_char_to_int(ch).into(), $overflow);
                    println!("> {idx}");
                    let exponent_pow = $t::from(16.).powf($t::from_usize(idx, $overflow) + 1.);
                    decimal_part += digit_value / exponent_pow;
                }
                if $float_parse.exponent_neg.unwrap_or(false) {
                   Number::$t((int_part + decimal_part) / exponent)
                } else {
                    Number::$t((int_part + decimal_part) * exponent)
                }
            },)*
            _ => unreachable!("Never happens: nb_type is float"),
        }
    }};
}

impl_floating_point!(23, Double Float LongDouble);

/// Trait to try and convert the integer and decimal part inside the mantissa.
///
/// ``overflow`` is set to true if the value doesn't fix in the mantissa.
trait FloatingPoint<T> {
    /// Size of the mantissa
    ///
    /// In the binary representation of the floating-point
    /// values, there is one part for the exponent, and one point for the
    /// digits, the latter is called 'mantissa'.
    const MANTISSA_SIZE: u32;
    /// The biggest unsigned integer type that can contain the mantissa.
    type Unsigned;
    /// Convert the integer-parsed value into the current floating-point type.
    fn from_unsigned(val: T, overflow: &mut bool) -> Self;
    /// Convert the usize-parsed value into the current floating-point type.
    fn from_usize(val: usize, overflow: &mut bool) -> Self;
}

/// Stores the data of an hexadecimal constant
#[derive(Default, Debug)]
struct HexFloatData {
    /// Decimal part of the constant, between the '.' and the 'p'
    decimal_part: String,
    /// Exponent part of the constant, after the 'p'
    exponent: String,
    /// Sign if found of the exponent
    ///
    /// - If a '+' is found after the 'p', ``exponent_neg = Some(false)``;
    /// - If a '-' is found after the 'p', ``exponent_neg = Some(true)``;
    /// - If a digit is found after the 'p', ``exponent_neg = None``.
    exponent_neg: Option<bool>,
    /// Integer part of the constant, before the '.'
    int_part: String,
    /// State of the parsing
    ///
    /// All the fields are set to default at the beginning, and when state
    /// changes, the fields begin receiving data, one by one.
    state: HexFloatParseState,
}

impl HexFloatData {
    /// Pushes a character to the current state.
    fn push(&mut self, ch: char) {
        match self.state {
            HexFloatParseState::Int => self.int_part.push(ch),
            HexFloatParseState::Decimal => self.decimal_part.push(ch),
            HexFloatParseState::Exponent => self.exponent.push(ch),
        }
    }

    /// Returns the exponent of the number constant.
    fn as_exp(&self) -> Result<u32, &'static str> {
        debug_assert!(
            !self.exponent.is_empty(),
            "Exponent not empty because exponent compulsory for float hexadecimals"
        );
        self.exponent.parse().map_err(|err: ParseIntError| {
            debug_assert!(matches!(err.kind(), IntErrorKind::PosOverflow), "none others possible");
            "Failed to parse exponent: too large"
        })
    }
}

/// Parsing state of the hexadecimal constant
///
/// The first part is the integer part, then the decimal part after a full stop,
/// and a exponent part after an exponent character ('p').
#[derive(Default, PartialEq, Eq, Debug)]
enum HexFloatParseState {
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
fn as_hex_float_data(literal: &str, location: &ErrorLocation) -> CompileRes<HexFloatData> {
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
                return Err(location.to_fault(format!(
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

/// Converts a hexadecimal digit to its value.
///
/// # Panics
///
/// This function panics if the char is not a valid hexadecimal digits.
///
/// # Examples
///
/// ```ignore
/// assert!(hex_char_to_int('f') == 15);
/// ```
///
/// ```ignore,should_panic
/// hex_char_to_int('p'); // this panics
/// ```
#[coverage(off)]
fn hex_char_to_int(ch: char) -> u8 {
    match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' | 'A' => 10,
        'b' | 'B' => 11,
        'c' | 'C' => 12,
        'd' | 'D' => 13,
        'e' | 'E' => 14,
        'f' | 'F' => 15,
        _ => unreachable!("function called on non hex char"),
    }
}

/// Parsed an hexadecimal float.
///
/// This is a wrapper for float handling. See [`to_hex_value`] for more detail.
fn to_hex_float_value(
    overflow: &mut bool,
    nb_type: NumberType,
    float_data: &HexFloatData,
) -> Result<Number, String> {
    Ok(parse_hexadecimal_float!(overflow, nb_type, float_data, Float Double LongDouble))
}

/// Parses an hexadecimal value.
///
/// The input doesn't contain the prefix ('0x') or the suffix (e.g. 'ULL').
///
/// # Returns
///
/// A [`OverParseRes`]. It contains one or more of the following:
///
///  - the value, if the parsing succeeded
///  - errors, if there are some
///  - overflow warning if a value was crapped to fit in the specified type.
///
///
/// # Examples
///
/// ```ignore
/// use crate::errors::location::LocationPointer;
/// use crate::lexer::numbers::parse::OverParseRes;
/// use crate::lexer::numbers::types::{Number, NumberType};
///
/// assert!(
///     to_hex_value("f20", &NumberType::Int, &LocationPointer::from(String::new()))
///         == OverParseRes::Value(Number::Int(3872))
/// );
/// assert!(
///     to_hex_value("ffffffff", &NumberType::Int, &LocationPointer::from(String::new()))
///         == OverParseRes::ValueOverflow(2i32.pow(31) - 1)
/// );
/// assert!(matches!(
///     to_hex_value("1o3", &NumberType::Int, &LocationPointer::from(String::new())),
///     OverParseRes::Err(_)
/// ));
/// ```
pub fn to_hex_value(
    literal: &str,
    nb_type: NumberType,
    location: &ErrorLocation,
) -> OverParseRes<Number> {
    let float_data = match as_hex_float_data(literal, location) {
        Err(err) => return OverParseRes::from(err),
        Ok(parsed) => parsed,
    };
    if float_data.exponent.is_empty()
        && (float_data.exponent_neg.is_some() || float_data.state == HexFloatParseState::Exponent)
    {
        return OverParseRes::from(
            location
                .to_fault(format!("{ERR_PREFIX}Illegal floating point constant: found empty exponent, but at least one digit was expected.")),
        );
    }
    if nb_type.is_int() {
        parse_int_from_radix!(location,
           nb_type, literal, "never fails", 16, Int Long LongLong UInt ULong ULongLong
        )
    } else {
        let mut overflow = false;
        match to_hex_float_value(&mut overflow, nb_type, &float_data) {
            Ok(number) =>
                if overflow {
                    OverParseRes::from_value_overflow(number)
                } else {
                    OverParseRes::from(number)
                },
            Err(msg) => OverParseRes::from(location.to_fault(msg)),
        }
    }
}
