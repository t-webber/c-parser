mod base;
mod types;

use core::str;

#[allow(clippy::useless_attribute)]
#[allow(clippy::pub_use)]
pub use types::Number;

use base::{to_bin_value, to_decimal_value, to_hex_value, to_oct_value};
use types::{Base, Int, NumberType, ERR_PREFIX};

use crate::{
    errors::{compile::FailRes, location::Location},
    to_error,
};

use super::parsing_state::ParsingState;

pub fn literal_to_number(p_state: &mut ParsingState) -> Option<Number> {
    let literal = &p_state.literal;

    if literal.is_empty()
        || !literal
            .as_bytes()
            .first()
            .expect("not empty")
            .is_ascii_digit()
    {
        return None;
    }
    if literal.len() == 1 {
        return literal
            .parse::<Int>()
            .map_or_else(|_| None, |x| Some(Number::Int(x)));
    }

    match literal_to_number_err(literal, &p_state.initial_location) {
        Ok(nb) => Some(nb),
        Err(error) => {
            p_state.push_err(error);
            None
        }
    }
}

fn literal_to_number_err(literal: &str, location: &Location) -> FailRes<Number> {
    let nb_type = get_number_type(literal, location)?;
    let base = get_base(literal, &nb_type, location)?;
    let value = str::from_utf8(
        literal
            .as_bytes()
            .get(base.prefix_size()..literal.len() - nb_type.suffix_size())
            .expect("never happens as suffix size + prefix size <= len, as 'x' and 'b' can't be used as suffix"),
    )
    .expect("never happens: all rust chars are valid utf8");

    match base {
        Base::Binary => to_bin_value(value, &nb_type, location),
        Base::Decimal => to_decimal_value(value, &nb_type, location),
        Base::Hexadecimal => to_hex_value(value, &nb_type, location),
        Base::Octal => to_oct_value(value, &nb_type, location),
    }
}

fn get_base(literal: &str, nb_type: &NumberType, location: &Location) -> FailRes<Base> {
    let mut chars = literal.chars();
    let first = chars.next().expect("len >= 1");
    let second = chars.next().expect("len >= 2");

    let one_char = literal.len() - nb_type.suffix_size() == 1;

    match (first, second) {
        ('0', 'x') if one_char => Err(to_error!(
            location,
            "{ERR_PREFIX}no digits found after 0x prefix"
        )),
        ('0', 'b') if one_char => Err(to_error!(
            location,
            "{ERR_PREFIX}no digits found after 0b prefix"
        )),
        ('0', 'x') => Ok(Base::Hexadecimal),
        ('0', 'b') if nb_type.is_int() => Ok(Base::Binary),
        ('0', 'b') if matches!(nb_type, NumberType::Float) => Err(to_error!(
            location,
            "{ERR_PREFIX}a binary can't be a `float`"
        )),
        ('0', 'b') => Err(to_error!(
            location,
            "{ERR_PREFIX}a binary can't be a `double`"
        )),
        ('0', '0'..='9') if nb_type.is_int() => Ok(Base::Octal),
        ('0', _) if nb_type.is_int() && one_char => Ok(Base::Decimal),
        ('0', ch) if nb_type.is_int() => Err(to_error!(
            location,
            "{ERR_PREFIX}found illegal character '{ch}' in octal representation."
        )),
        _ => Ok(Base::Decimal),
    }
}

fn get_number_type(literal: &str, location: &Location) -> FailRes<NumberType> {
    /* literal characteristics */
    let double_or_float = literal.contains('.')
        || (literal.starts_with("0x") && (literal.contains(['p', 'P'])))
        || (!literal.starts_with("0x") && (literal.contains(['e', 'E'])));

    // will be computed below
    let chars = literal.chars().rev();
    let mut l_count: u32 = 0;
    let mut unsigned = false;
    let mut float = false;

    for ch in chars {
        match ch {
            'u' | 'U' if unsigned => return Err(to_error!(location, "found 2 'u' characters.")),
            'u' | 'U' => unsigned = true,
            'l' | 'L' if l_count == 2 => {
                return Err(to_error!(
                    location,
                    "found 3 'l' characters, but max is 2 (`long long`)."
                ))
            }
            'l' | 'L' => l_count += 1,
            'f' | 'F' => float = true,
            'i' | 'I' => {
                return Err(to_error!(
                    location,
                    "imaginary constants are a GCC extension."
                ))
            }
            _ => break,
        }
    }

    // get the type from the characteristics
    match (float, double_or_float, unsigned, l_count) {
        (false, false, false, 0) => Ok(NumberType::Int),
        (false, false, false, 1) => Ok(NumberType::Long),
        (false, false, false, 2) => Ok(NumberType::LongLong),
        (_, _, _, l_c) if l_c >= 3  => {
            Err(to_error!(location, "`long long double` doesn't exist."))
        }
        (false, false, true, 0) => Ok(NumberType::UInt),
        (false, false, true, 1) => Ok(NumberType::ULong),
        (false, false, true, 2) => Ok(NumberType::ULongLong),
        (false, true, false, 0) => Ok(NumberType::Double),
        (false, true, false, 1) => Ok(NumberType::LongDouble),
        (false, true, false, l_c) if l_c >= 2 => {
            Err(to_error!(location, "`long long double` doesn't exist."))
        }
        (true, _, true, _) => Err(to_error!(location, "a `float` can't be `unsigned`.")), // moved up not to be shadowed
        (_, true, true, _) => {
            Err(to_error!(location, "a `double` can't be `unsigned`."))
        },
        (true, false, _, _) =>  Err(to_error!(location, "a 'f' suffix only works on `double` constants. Please insert a period or an exponent character before the 'f'.")),
        (true, true, false, 0)  => Ok(NumberType::Float),
        (true, true, false, l_c) if l_c > 0  => Err(to_error!(location, "a `float` can't be `long`. Did you mean `long double`? Remove the leading 'f' if that is the case.")),
        #[allow(clippy::unreachable)]
        (_, _, _, 3..=u32::MAX) | (false, true, false, 2..=u32::MAX) | (true, true, false, 1..=2) => unreachable!()
    }
}
