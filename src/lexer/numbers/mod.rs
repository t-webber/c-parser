mod base;
pub mod macros;
pub mod parse;
pub mod types;
use core::str;

use base::{binary, decimal, hexadecimal, octal};
use parse::{OverParseRes, ParseRes};
#[allow(clippy::wildcard_imports)]
use types::arch_types::*;
#[allow(clippy::useless_attribute)]
#[allow(clippy::pub_use)]
pub use types::Number;
use types::{Base, NumberType, ERR_PREFIX};

use super::types::lexing_data::LexingData;
use super::types::lexing_state::Ident;
use crate::errors::compile::CompileError;
use crate::errors::location::Location;

fn check_with_base(literal: &str, base: &Base) -> Option<char> {
    let mut chars = literal.chars();
    match base {
        Base::Binary => chars.find(|ch| !matches!(ch, '0' | '1')),
        Base::Decimal => chars.find(|ch| !matches!(ch, '0'..='9' | '.' | 'e' | 'E' | '+' | '-')),
        Base::Hexadecimal => {
            chars.find(|ch| !ch.is_ascii_hexdigit() && !matches!(ch, '.' | 'p' | 'P' | '+' | '-'))
        }
        Base::Octal => chars.find(|ch| !ch.is_ascii_octdigit()),
    }
}

fn get_base(
    literal: &str,
    nb_type: &NumberType,
    location: &Location,
) -> Result<Base, CompileError> {
    let mut chars = literal.chars();
    let first = chars.next().expect("len >= 1");
    let second = chars.next().expect("len >= 2");

    let one_char = literal.len() - nb_type.suffix_size() == 1;

    match (first, second) {
        ('0', 'x') if one_char => {
            Err(location.to_error(format!("{ERR_PREFIX}no digits found after 0x prefix")))
        }
        ('0', 'b') if one_char => {
            Err(location.to_error(format!("{ERR_PREFIX}no digits found after 0b prefix")))
        }
        ('0', 'x') => Ok(Base::Hexadecimal),
        ('0', 'b') if nb_type.is_int() => Ok(Base::Binary),
        ('0', 'b') if matches!(nb_type, NumberType::Float) => {
            Err(location.to_error(format!("{ERR_PREFIX}a binary can't be a `float`")))
        }
        ('0', 'b') => Err(location.to_error(format!("{ERR_PREFIX}a binary can't be a `double`"))),
        ('0', '0'..='9') if nb_type.is_int() => Ok(Base::Octal),
        ('0', _) if nb_type.is_int() && one_char => Ok(Base::Decimal),
        ('0', ch) if nb_type.is_int() => Err(location.to_error(format!(
            "{ERR_PREFIX}found illegal character '{ch}' in octal representation."
        ))),
        _ => Ok(Base::Decimal),
    }
}

fn get_number_type(literal: &str, location: &Location) -> Result<NumberType, CompileError> {
    let is_hex = literal.starts_with("0x");
    /* literal characteristics */
    let double_or_float = literal.contains('.')
        || (is_hex && (literal.contains(['p', 'P'])))
        || (!is_hex && (literal.contains(['e', 'E'])));

    // will be computed below
    let chars = literal.chars().rev();
    let mut l_count: u32 = 0;
    let mut unsigned = false;
    let mut float = false;

    for ch in chars {
        match ch {
            'u' | 'U' if unsigned => {
                return Err(location.to_error("found 2 'u' characters.".to_owned()))
            }
            'u' | 'U' => unsigned = true,
            'l' | 'L' if l_count == 2 => {
                return Err(location
                    .to_error("found 3 'l' characters, but max is 2 (`long long`).".to_owned()))
            }
            'l' | 'L' => l_count += 1,
            'f' | 'F' if is_hex && !double_or_float => break,
            'f' | 'F' => float = true,
            'i' | 'I' => {
                return Err(location.to_error("imaginary constants are a GCC extension.".to_owned()))
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
            Err(location.to_error(format!("{ERR_PREFIX}`long long double` doesn't exist.")))
        }
        (false, false, true, 0) => Ok(NumberType::UInt),
        (false, false, true, 1) => Ok(NumberType::ULong),
        (false, false, true, 2) => Ok(NumberType::ULongLong),
        (false, true, false, 0) => Ok(NumberType::Double),
        (false, true, false, 1) => Ok(NumberType::LongDouble),
        (false, true, false, l_c) if l_c >= 2 => {
            Err(location.to_error(format!("{ERR_PREFIX}`long long double` doesn't exist.")))
        }
        (true, _, true, _) => Err(location.to_error(format!("{ERR_PREFIX}a `float` can't be `unsigned`."))), // moved up not to be shadowed
        (_, true, true, _) => {
            Err(location.to_error(format!("{ERR_PREFIX}a `double` can't be `unsigned`.")))
        },
        (true, false, _, _) if is_hex =>  Err(location.to_error(format!("{ERR_PREFIX}a 'f' suffix only works on `double` constants. Please insert a 'p' exponent character before the 'f'."))),
        (true, false, _, _) =>  Err(location.to_error(format!("{ERR_PREFIX}a 'f' suffix only works on `double` constants. Please insert a period or a 'e' exponent character before the 'f'."))),
        (true, true, false, 0)  => Ok(NumberType::Float),
        (true, true, false, l_c) if l_c > 0  => Err(location.to_error(format!("{ERR_PREFIX}a `float` can't be `long`. Did you mean `long double`? Remove the leading 'f' if that is the case."))),
        (_, _, _, 3..=u32::MAX) | (false, true, false, 2..=u32::MAX) | (true, true, false, 1..=2) => panic!("never happens normally")
    }
}

pub fn literal_to_number(
    lex_data: &mut LexingData,
    literal: &Ident,
    location: &Location,
) -> Option<Number> {
    if literal.is_empty() || !literal.is_number() {
        return None;
    }

    if literal.len() == 1 {
        return literal
            .value()
            .parse::<Int>()
            .map_or_else(|_| None, |x| Some(Number::Int(x)));
    };

    let mut res = literal_to_number_err(literal.value(), location, lex_data.last_is_minus());
    res.edit_err(|err| err.specify_length(literal.len() - 1));
    match res {
        ParseRes::Value(val) => Some(val),
        ParseRes::Err(err) => {
            lex_data.push_err(err);
            None
        }
        ParseRes::ValueErr(val, err) => {
            lex_data.push_err(err);
            Some(val)
        }
    }
}

fn literal_to_number_err(literal: &str, location: &Location, signed: bool) -> ParseRes<Number> {
    let mut nb_type = get_number_type(literal, location)?;
    let base = get_base(literal, &nb_type, location)?;
    let value = literal
                .get(base.prefix_size()..literal.len() - nb_type.suffix_size())
            .expect("never happens as suffix size + prefix size <= len, as 'x' and 'b' can't be used as suffix");

    if value.is_empty() {
        return ParseRes::Err(location.to_error(format!(
            "{ERR_PREFIX}found no digits between prefix and suffix. Please add at least one digit.",
        )));
    }

    if let Some(ch) = check_with_base(value, &base) {
        return ParseRes::Err(location.to_error(format!(
            "{ERR_PREFIX}found invalid character '{ch}' in {} base.",
            base.repr(),
        )));
    }

    loop {
        let parse_res = match base {
            Base::Binary => binary::to_bin_value(value, &nb_type, location),
            Base::Decimal => decimal::to_decimal_value(value, &nb_type, location),
            Base::Hexadecimal => hexadecimal::to_hex_value(value, &nb_type, location),
            Base::Octal => octal::to_oct_value(value, &nb_type, location),
        };
        if parse_res.overflowed()
            && let Some(new_type) = nb_type.incr_size(signed)
        {
            nb_type = new_type;
        } else {
            return parse_res.ignore_overflow(literal, location);
        }
    }
}
