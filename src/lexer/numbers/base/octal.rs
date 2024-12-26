use super::super::macros::parse_int_from_radix;
use super::super::parse::OverParseRes;
#[allow(clippy::wildcard_imports)]
use super::super::types::arch_types::*;
use super::super::types::{Number, NumberType, ERR_PREFIX};
use crate::errors::api::Location;

pub fn to_oct_value(
    literal: &str,
    nb_type: &NumberType,
    location: &Location,
) -> OverParseRes<Number> {
    if literal.chars().all(|ch| matches!(ch, '0'..='7')) {
        parse_int_from_radix!(
            location,
           nb_type, literal, "an octal must be an integer", 8, Int Long LongLong UInt ULong ULongLong
        )
    } else {
        let first = literal
            .chars()
            .find(|ch| matches!(ch, '0'..='7'))
            .expect("Exists according to line above");
        OverParseRes::from(location.to_error(format!("{ERR_PREFIX}a octal constant must only contain digits between '0' and '7'. Found invalid character '{first}'.")))
    }
}
