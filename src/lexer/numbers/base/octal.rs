use super::parse_int_from_radix;
use crate::errors::compile::to_error;
use crate::errors::location::Location;
#[allow(clippy::wildcard_imports)]
use crate::lexer::numbers::types::arch_types::*;
use crate::lexer::numbers::types::NumberType;
use crate::lexer::numbers::{Number, ParseResult, ERR_PREFIX};

pub fn to_oct_value(
    literal: &str,
    nb_type: &NumberType,
    location: &Location,
) -> ParseResult<Number> {
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
        ParseResult::from(to_error!(location, "{ERR_PREFIX}a octal constant must only contain digits between '0' and '7'. Found invalid character '{first}'."))
    }
}
