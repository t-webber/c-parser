use super::super::macros::parse_int_from_radix;
use super::super::parse::OverParseRes;
#[allow(clippy::wildcard_imports)]
use super::super::types::arch_types::*;
use super::super::types::{Number, NumberType, ERR_PREFIX};
use crate::errors::api::Location;

pub fn to_bin_value(
    literal: &str,
    nb_type: &NumberType,
    location: &Location,
) -> OverParseRes<Number> {
    if literal.chars().all(|ch| matches!(ch, '0' | '1')) {
        parse_int_from_radix!(location,
           nb_type, literal, "a binary must be an integer", 2, Int Long LongLong UInt ULong ULongLong
        )
    } else {
        let first = literal
            .chars()
            .find(|ch| matches!(ch, '0' | '1'))
            .expect("Exists according to line above");
        OverParseRes::from(location.to_error(format!(
            "{ERR_PREFIX}a binary constant must only contain '0's and '1's. Found invalid character '{first}'."
        )))
    }
}
