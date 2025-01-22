//! Module to parse decimal-represented number constants

use crate::errors::api::Location;
use crate::lexer::numbers::parse::OverParseRes;
use crate::lexer::numbers::types::arch_types::{
    Double, Float, Int, Long, LongLong, UInt, ULong, ULongLong
};
use crate::lexer::numbers::types::{ERR_PREFIX, Number, NumberType};

/// Parses the stringifies version of a decimal number in a specific integer
/// or floating point type.
///
/// # Panics
///
/// If the float parsing fails, i.e. contains an invalid character.
macro_rules! parse_number {
    ($location:ident, $nb_type:ident, $literal:tt, $($int:ident)*, $($float:ident)*) => {
        match $nb_type {
            NumberType::LongDouble => OverParseRes::from($location.to_fault(format!("{ERR_PREFIX}`long double` not supported yet."))), //TODO: f128 not implemented
            $(NumberType::$int => $crate::lexer::numbers::macros::safe_parse_int!(ERR_PREFIX, $int, $location, $literal.parse::<$int>()).map(|nb| Number::$int(nb)),)*
            $(NumberType::$float => OverParseRes::from(Number::$float($literal.parse::<$float>().expect("only crashes with invalid character"))),)*
        }
    };
}

/// Parses a binary value.
///
/// The input doesn't contain the suffix (e.g. 'ULL').
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
///     to_decimal_value("123", &NumberType::Int, &Location::from(String::new()))
///         == OverParseRes::Value(Number::Int(123))
/// );
/// assert!(
///     to_decimal_value(
///         "1e33",
///         &NumberType::Int,
///         &Location::from(String::new())
///     ) == OverParseRes::ValueOverflow(2i32.pow(31) - 1)
/// );
/// assert!(matches!(
///     to_decimal_value("1fe3", &NumberType::Int, &Location::from(String::new())),
///     OverParseRes::Err(_)
/// ));
/// ```
pub fn to_decimal_value(
    literal: &str,
    nb_type: NumberType,
    location: &Location,
) -> OverParseRes<Number> {
    parse_number!(location,  nb_type, literal, Int Long LongLong UInt ULong ULongLong, Float Double )
}
