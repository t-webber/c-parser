use crate::errors::compile::CompileError;
use crate::errors::location::Location;
#[allow(clippy::wildcard_imports)]
use crate::parse::numbers::types::arch_types::*;
use crate::parse::numbers::types::{Number, NumberType, ERR_PREFIX};
use crate::{parse_int_from_radix, to_error};

pub fn to_bin_value(
    literal: &str,
    nb_type: &NumberType,
    location: &Location,
) -> Result<Number, CompileError> {
    if literal.chars().all(|ch| matches!(ch, '0' | '1')) {
        parse_int_from_radix!(location,
           nb_type, literal, "a binary must be an integer", 2, Int Long LongLong UInt ULong ULongLong
        )
    } else {
        let first = literal
            .chars()
            .find(|ch| matches!(ch, '0' | '1'))
            .expect("Exists according to line above");
        Err(to_error!(location, "{ERR_PREFIX}a binary constant must only contain '0's and '1's. Found invalid character '{first}'."))
    }
}
