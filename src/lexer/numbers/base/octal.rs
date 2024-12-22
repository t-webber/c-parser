use super::parse_int_from_radix;
use crate::errors::compile::to_error;
use crate::errors::compile::CompileError;
use crate::errors::location::Location;
#[allow(clippy::wildcard_imports)]
use crate::lexer::numbers::types::arch_types::*;
use crate::lexer::numbers::types::NumberType;
use crate::lexer::numbers::Number;
use crate::lexer::numbers::ERR_PREFIX;

pub fn to_oct_value(
    literal: &str,
    nb_type: &NumberType,
    location: &Location,
) -> Result<Number, CompileError> {
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
        Err(to_error!(location, "{ERR_PREFIX}a octal constant must only contain digits between '0' and '7'. Found invalid character '{first}'."))
    }
}
