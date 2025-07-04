//! Module to parse octal-represented number constants

use crate::errors::api::ErrorLocation;
use crate::lexer::numbers::macros::parse_int_from_radix;
use crate::lexer::numbers::parse::OverParseRes;
use crate::lexer::numbers::types::arch_types::{Int, Long, LongLong, UInt, ULong, ULongLong};
use crate::lexer::numbers::types::{ERR_PREFIX, Number, NumberType};

/// Parses an octal value.
///
/// The input doesn't contain the prefix ('0') or the suffix (e.g. 'ULL').
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
///     to_oct_value("123", &NumberType::Int, &LocationPointer::from(String::new()))
///         == OverParseRes::Value(Number::Int(83))
/// );
/// assert!(
///     to_oct_value(
///         "377",
///         &NumberType::Int,
///         &LocationPointer::from(String::new())
///     ) == OverParseRes::ValueOverflow(2i32.pow(31) - 1)
/// );
/// assert!(matches!(
///     to_oct_value("1f3", &NumberType::Int, &LocationPointer::from(String::new())),
///     OverParseRes::Err(_)
/// ));
/// ```
pub fn to_oct_value(
    literal: &str,
    nb_type: NumberType,
    location: &ErrorLocation,
) -> OverParseRes<Number> {
    debug_assert!(literal.chars().all(|ch| matches!(ch, '0'..='7')), "checked when creating base");
    parse_int_from_radix!(
        location,
       nb_type, literal, "an octal must be an integer", 8, Int Long LongLong UInt ULong ULongLong
    )
}
