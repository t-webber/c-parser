use super::parse_int_from_radix;
use crate::errors::compile::to_error;
use crate::errors::location::Location;
#[allow(clippy::wildcard_imports)]
use crate::lexer::numbers::types::arch_types::*;
use crate::lexer::numbers::types::{Number, NumberType, ERR_PREFIX};
use crate::lexer::numbers::ParseResult;

pub fn to_bin_value(
    literal: &str,
    nb_type: &NumberType,
    location: &Location,
) -> ParseResult<Number> {
    if literal.chars().all(|ch| matches!(ch, '0' | '1')) {
        parse_int_from_radix!(location,
           nb_type, literal, "a binary must be an integer", 2, Int Long LongLong UInt ULong ULongLong
        )
    } else {
        let first = literal
            .chars()
            .find(|ch| matches!(ch, '0' | '1'))
            .expect("Exists according to line above");
        ParseResult::from(to_error!(location, "{ERR_PREFIX}a binary constant must only contain '0's and '1's. Found invalid character '{first}'."))
    }
}
