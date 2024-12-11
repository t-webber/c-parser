use crate::errors::compile::CompileError;
use crate::errors::location::Location;
#[allow(clippy::wildcard_imports)]
use crate::parse::numbers::types::arch_types::*;
use crate::parse::numbers::types::{Number, NumberType, ERR_PREFIX};
use crate::to_error;
use core::num::{IntErrorKind, ParseFloatError, ParseIntError};
use core::str::FromStr;

macro_rules! safe_parse_int {
    ($dest_type:ident, $location:ident, $function_call:expr) => {{
        let parsed: Result<$dest_type, ParseIntError> = $function_call.map_err(|err| err.into());
        match parsed {
            Ok(nb) => Ok(nb),
            Err(err) => match *err.kind() {
                IntErrorKind::Empty => panic!("Never happens. Checks for non empty."),
                IntErrorKind::InvalidDigit => Err(to_error!(
                    $location,
                    "{ERR_PREFIX}invalid decimal number: must contain only ascii digits and at most one '.', one 'e' followed by at most a sign."
                )),
                IntErrorKind::PosOverflow => Err(to_error!(
                    $location,
                    "{ERR_PREFIX}postive overflow on decimal number: number is too large to fit in attributed type. Add a suffix or reduce value."
                )),
                IntErrorKind::NegOverflow => Err(to_error!(
                    $location,
                    "{ERR_PREFIX}negative overflow on decimal number: number is too large to fit in attributed type. Add a suffix or reduce value."
                )),
                IntErrorKind::Zero | _ => panic!("Unexpected error"),

            },
        }
    }};
}

fn parse_and_error<T>(literal: &str, location: &Location) -> Result<T, CompileError>
where
    T: FromStr,
    <T as FromStr>::Err: Into<ParseFloatError>,
{
    literal
        .parse::<T>()
        .map_err(|_err| to_error!(location, "{ERR_PREFIX}invalid decimal float number."))
}

macro_rules! parse_number {
    ($location:ident, $nb_type:ident, $literal:tt, $($int:ident)*, $($float:ident)*) => {
        match $nb_type {
            NumberType::LongDouble => Err(to_error!($location, "{ERR_PREFIX}`long double` not supported yet.")), //TODO: f128 not implemented
            $(NumberType::$int => Ok(Number::$int(safe_parse_int!($int, $location, $literal.parse::<$int>())?)),)*
            $(NumberType::$float => Ok(Number::$float(parse_and_error::<$float>($literal, $location)?)),)*
        }
    };
}

pub fn to_decimal_value(
    literal: &str,
    nb_type: &NumberType,
    location: &Location,
) -> Result<Number, CompileError> {
    parse_number!(location,  nb_type, literal, Int Long LongLong UInt ULong ULongLong, Float Double )
}
