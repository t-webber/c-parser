//! Module to parse binary-represented number constants

use crate::errors::api::Location;
use crate::lexer::numbers::macros::parse_int_from_radix;
use crate::lexer::numbers::parse::OverParseRes;
use crate::lexer::numbers::types::arch_types::{Int, Long, LongLong, UInt, ULong, ULongLong};
use crate::lexer::numbers::types::{ERR_PREFIX, Number, NumberType};

/// Parses a binary value.
///
/// The input doesn't contain the prefix ('0b') or the suffix (e.g. 'ULL').
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
///     to_bin_value("1010", &NumberType::Int, &Location::from(String::new()))
///         == OverParseRes::Value(Number::Int(10))
/// );
/// assert!(
///     to_bin_value(
///         "11111111111111111111111111111111",
///         &NumberType::Int,
///         &Location::from(String::new())
///     ) == OverParseRes::ValueOverflow(2i32.pow(31) - 1)
/// );
/// assert!(matches!(
///     to_bin_value("123", &NumberType::Int, &Location::from(String::new())),
///     OverParseRes::Err(_)
/// ));
/// ```
pub fn to_bin_value(
    literal: &str,
    nb_type: NumberType,
    location: &Location,
) -> OverParseRes<Number> {
    debug_assert!(
        literal.chars().all(|ch| matches!(ch, '0' | '1')),
        "checked for invalid characters when finding base."
    );
    parse_int_from_radix!(location,
       nb_type, literal, "a binary must be an integer", 2, Int Long LongLong UInt ULong ULongLong
    )
}
