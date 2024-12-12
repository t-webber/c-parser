use crate::errors::compile::CompileError;
use crate::errors::location::Location;
#[allow(clippy::wildcard_imports)]
use crate::lexer::numbers::types::arch_types::*;
use crate::lexer::numbers::types::{Number, NumberType, ERR_PREFIX};
use crate::{safe_parse_int, to_error};
use core::num::ParseFloatError;
use core::str::FromStr;

macro_rules! parse_number {
    ($location:ident, $nb_type:ident, $literal:tt, $($int:ident)*, $($float:ident)*) => {
        match $nb_type {
            NumberType::LongDouble => Err(to_error!($location, "{ERR_PREFIX}`long double` not supported yet.")), //TODO: f128 not implemented
            $(NumberType::$int => Ok(Number::$int(safe_parse_int!($int, $location, $literal.parse::<$int>())?)),)*
            $(NumberType::$float => Ok(Number::$float(parse_and_error::<$float>($literal, $location)?)),)*
        }
    };
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

pub fn to_decimal_value(
    literal: &str,
    nb_type: &NumberType,
    location: &Location,
) -> Result<Number, CompileError> {
    parse_number!(location,  nb_type, literal, Int Long LongLong UInt ULong ULongLong, Float Double )
}
