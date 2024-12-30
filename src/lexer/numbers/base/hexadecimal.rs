#![allow(clippy::arbitrary_source_item_ordering)]

use super::super::macros::parse_int_from_radix;
use super::super::parse::OverParseRes;
#[allow(clippy::wildcard_imports)]
use super::super::types::arch_types::*;
use super::super::types::{ERR_PREFIX, Number, NumberType};
use crate::errors::api::{CompileError, Location};

macro_rules! impl_floating_point {
    ($x:expr, $($ftype:ident)*) => {
        $(#[expect(clippy::as_conversions, clippy::cast_precision_loss)]
        impl FloatingPoint<concat_idents!($ftype, IntPart)> for $ftype {
            const MANTISSA_SIZE: u32 = $x;

            type Unsigned = concat_idents!($ftype, IntPart);


            fn from_unsigned(
                val: Self::Unsigned,
                overflow: &mut bool,
            ) -> Self {
                if val >= (2 as Self::Unsigned).pow(Self::MANTISSA_SIZE) {
                    *overflow = true;
                }
                val as Self
            }

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

macro_rules! parse_hexadecimal_float {
    ($overflow:expr, $nb_type:ident, $float_parse:ident, $($t:ident)*) => {{
        match $nb_type {
            $(NumberType::$t => {
                let int_part = $t::from_unsigned(
                    <concat_idents!($t, IntPart)>::from_str_radix(&$float_parse.int_part, 16).expect("2 <= <= 36"),
                    $overflow);
                #[expect(clippy::as_conversions)]
                let exponent = $t::from_unsigned((2 as concat_idents!($t, IntPart)).pow($float_parse.get_exp()), $overflow);
                let mut decimal_part: $t = 0.;
                for (idx, ch) in $float_parse.decimal_part.chars().enumerate() {
                    let digit_value = $t::from_unsigned(hex_char_to_int(ch).into(), $overflow);
                    let exponent_pow = $t::from(16.).powf($t::from_usize(idx, $overflow) + 1.);
                    decimal_part += digit_value / exponent_pow;
                }
                if $float_parse.exponent_neg.unwrap_or(false) {
                   OverParseRes::from(Number::$t((int_part + decimal_part) / exponent))
                } else {
                    OverParseRes::from(Number::$t((int_part + decimal_part) * exponent))
                }
            },)*
            _ => panic!("Never happens: nb_type is float"),
        }
    }};
}

impl_floating_point!(23, Double Float LongDouble);

/// Trait to try and convert the integer and decimal part inside the mantissa.
///
/// ``overflow`` is set to true if the value doesn't fix in the mantissa.
trait FloatingPoint<T> {
    const MANTISSA_SIZE: u32;
    type Unsigned;
    fn from_unsigned(val: T, overflow: &mut bool) -> Self;
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
    fn push(&mut self, ch: char) {
        match self.state {
            HexFloatParseState::Int => self.int_part.push(ch),
            HexFloatParseState::Decimal => self.decimal_part.push(ch),
            HexFloatParseState::Exponent => self.exponent.push(ch),
        }
    }

    fn get_exp(&self) -> u32 {
        if self.exponent.is_empty() {
            0
        } else {
            self.exponent
                .parse()
                .expect("Never fails: contains only ascii digits and not empty")
        }
    }
}

/// Parsing state of the hexadecimal constant
///
/// The first part is the integer part, then the decimal part after a full stop,
/// and a exponent part after an exponent character ('p').
#[derive(Default, PartialEq, Eq, Debug)]
enum HexFloatParseState {
    Decimal,
    Exponent,
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
/// use crate::errors::location::Location;
///
/// assert!(
///     get_hex_float_data("fd.ep2", &Location::from(String::new()))
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
///     get_hex_float_data("fd.ep++2", &Location::from(String::new())),
///     Err(_)
/// );
/// ```
fn get_hex_float_data(literal: &str, location: &Location) -> Result<HexFloatData, CompileError> {
    let mut float_parse = HexFloatData::default();
    for ch in literal.chars() {
        match ch {
            '+' | '-' if float_parse.state != HexFloatParseState::Exponent => {
                panic!("never happens: + or - always are after a p character in hex literal")
            }
            '+' | '-' if float_parse.exponent_neg.is_some() => {
                return Err(location.to_error(format!("{ERR_PREFIX}maximum one sign is allowed in a number literal.")))
            }
            '-' => float_parse.exponent_neg = Some(true),
            '+' => float_parse.exponent_neg = Some(false),
            _ if float_parse.state == HexFloatParseState::Exponent && ch.is_ascii_digit() => float_parse.push(ch),
            _ if float_parse.state == HexFloatParseState::Exponent => {
                return Err(location.to_error(format!(
                    "{ERR_PREFIX}invalid character for exponent. Expected an ascii digit, but found '{ch}'"
                )))
            }
            _ if ch.is_ascii_hexdigit() => float_parse.push(ch),
            '.' if float_parse.state == HexFloatParseState::Int => float_parse.state = HexFloatParseState::Decimal,
            '.' if float_parse.state == HexFloatParseState::Decimal => {
                return Err(location.to_error(format!(
                    "{ERR_PREFIX}maximum one '.' in number constant, but 2 were found."
                )))
            }
            '.' if float_parse.state == HexFloatParseState::Exponent => {
                return Err(location.to_error(format!("{ERR_PREFIX}exponent must be an integer, but found a full stop.")))
            }
            'p' | 'P' if float_parse.state == HexFloatParseState::Exponent => {
                return Err(location.to_error(format!(
                    "{ERR_PREFIX}maximum one 'p' in number constant, but 2 were found."
                )))
            }
            'p' | 'P' => float_parse.state = HexFloatParseState::Exponent,
            _ => {
                return Err(location.to_error(format!("{ERR_PREFIX}invalid character '{ch}' found in number constant")))
            }
        };
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
        _ => panic!("function called on non hex char"),
    }
}

/// Parses a binary value.
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
/// use crate::errors::location::Location;
/// use crate::lexer::numbers::parse::OverParseRes;
/// use crate::lexer::numbers::types::{Number, NumberType};
///
/// assert!(
///     to_hex_value("f20", &NumberType::Int, &Location::from(String::new()))
///         == OverParseRes::Value(Number::Int(3872))
/// );
/// assert!(
///     to_hex_value("ffffffff", &NumberType::Int, &Location::from(String::new()))
///         == OverParseRes::ValueOverflow(2i32.pow(31) - 1)
/// );
/// assert!(matches!(
///     to_hex_value("1o3", &NumberType::Int, &Location::from(String::new())),
///     OverParseRes::Err(_)
/// ));
/// ```
pub fn to_hex_value(
    literal: &str,
    nb_type: &NumberType,
    location: &Location,
) -> OverParseRes<Number> {
    let float_data = match get_hex_float_data(literal, location) {
        Err(err) => return OverParseRes::from(err),
        Ok(parsed) => parsed,
    };
    if float_data.exponent.is_empty()
        && (float_data.exponent_neg.is_some() || float_data.state == HexFloatParseState::Exponent)
    {
        return OverParseRes::from(
            location
                .to_error(format!("{ERR_PREFIX}Illegal floating point constant: found empty exponent, but at least one digit was expected.")),
        );
    }
    if nb_type.is_int() {
        parse_int_from_radix!(location,
           nb_type, literal, "never fails", 16, Int Long LongLong UInt ULong ULongLong
        )
    } else {
        let mut overflow = false;
        #[expect(clippy::float_arithmetic)]
        let res =
            parse_hexadecimal_float!(&mut overflow, nb_type, float_data, Float Double LongDouble);
        if overflow { res.add_overflow() } else { res }
    }
}
