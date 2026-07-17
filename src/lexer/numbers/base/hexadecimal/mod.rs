//! Module to parse decimal-represented number constants

/// Converts the parse hexadecimal constant to a [`Number`]
mod convert;
/// Parses the literal string into an hexadecimal value;
mod parse;

use crate::errors::api::ErrorLocation;
use crate::lexer::numbers::api::OverParseRes;
use crate::lexer::numbers::base::hexadecimal::convert::to_hex_float_value;
use crate::lexer::numbers::base::hexadecimal::parse::{HexFloatParseState, as_hex_float_data};
use crate::lexer::numbers::macros::parse_int_from_radix;
use crate::lexer::numbers::types::arch_types::{Int, Long, LongLong, UInt, ULong, ULongLong};
use crate::lexer::numbers::types::{ERR_PREFIX, Number, NumberType};

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
    location: ErrorLocation,
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
                .fail(format!("{ERR_PREFIX}Illegal floating point constant: found empty exponent, but at least one digit was expected.")),
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
            Err(msg) => OverParseRes::from(location.fail(msg)),
        }
    }
}
