mod base;
mod types;

use base::{to_bin_value, to_decimal_value, to_hex_value, to_oct_value};
use types::{Base, Int, Number, NumberType, OptionalReturn, ERR_PREFIX};
pub fn literal_to_number(literal: &str) -> OptionalReturn {
    if literal.is_empty() {
        return Ok(None);
    }
    if literal.len() == 1 {
        return Ok(literal
            .parse::<Int>()
            .map_or_else(|_| None, |x| Some(Number::Int(x))));
    }
    let nb_type = get_number_type(literal)?;
    let base = get_base(literal, &nb_type)?;

    Ok(Some(match base {
        Base::Binary => to_bin_value(literal, &nb_type),
        Base::Decimal => to_decimal_value(literal, &nb_type),
        Base::Hexadecimal => to_hex_value(literal, &nb_type),
        Base::Octal => to_oct_value(literal, &nb_type),
    }?))
}

fn get_base(literal: &str, nb_type: &NumberType) -> Result<Base, String> {
    let err_prefix = ERR_PREFIX.to_owned();

    let mut chars = literal.chars();
    let first = chars.next().expect("len >= 1");
    let second = chars.next().expect("len >= 2");

    match (first, second) {
        ('0', 'x') => Ok::<Base, String>(Base::Hexadecimal),
        ('0', 'b') if nb_type.is_int() => Ok(Base::Binary),
        ('0', 'b') if matches!(nb_type, NumberType::Float) => {
            Err(err_prefix + "a binary can't be a `float`")
        }
        ('0', 'b') => Err(err_prefix + "a binary can't be a `double`"),
        ('0', ch) if nb_type.is_int() && ch.is_ascii_digit() => Ok(Base::Octal),
        ('0', ch) if nb_type.is_int() => {
            //TODO: 0l
            Err(format!(
                "{ERR_PREFIX}found illegal character '{ch}' in octal representation."
            ))
        }
        _ => Ok(Base::Decimal),
    }
}

fn get_number_type(literal: &str) -> Result<NumberType, String> {
    let err_prefix = ERR_PREFIX.to_owned();

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
            'u' | 'U' if unsigned => return Err(err_prefix + "found 2 'u' characters."),
            'u' | 'U' => unsigned = true,
            'l' | 'L' if l_count == 2 => {
                return Err(err_prefix + "found 3 'l' characters, but max is 2 (`long long`).")
            }
            'l' | 'L' => l_count += 1,
            'f' | 'F' => float = true,
            'i' | 'I' => return Err(err_prefix + "imaginary constants are a GCC extension."),
            _ => break,
        }
    }

    // get the type from the characteristics
    match (float, double_or_float, unsigned, l_count) {
        (false, false, false, 0) => Ok(NumberType::Int),
        (false, false, false, 1) => Ok(NumberType::Long),
        (false, false, false, 2) => Ok(NumberType::LongLong),
        (_, _, _, l_c) if l_c >= 3  => {
            Err(err_prefix + "`long long double` doesn't exist.")
        }
        (false, false, true, 0) => Ok(NumberType::UInt),
        (false, false, true, 1) => Ok(NumberType::ULong),
        (false, false, true, 2) => Ok(NumberType::ULongLong),
        (false, true, false, 0) => Ok(NumberType::Double),
        (false, true, false, 1) => Ok(NumberType::LongDouble),
        (false, true, false, l_c) if l_c >= 2 => {
            Err(err_prefix + "`long long double` doesn't exist.")
        }
        (true, _, true, _) => Err(err_prefix + "a `float` can't be `unsigned`."), // moved up not to be shadowed
        (_, true, true, _) => {
            Err(err_prefix + "a `double` can't be `unsigned`.")
        },
        (true, false, _, _) =>  Err(err_prefix + "a 'f' suffix only works on `double` constants. Please insert a period or an exponent character before the 'f'."),
        (true, true, false, 0)  => Ok(NumberType::Float),
        (true, true, false, l_c) if l_c > 0  => Err(err_prefix + "a `float` can't be `long`. Did you mean `long double`? Remove the leading 'f' if that is the case."),
        #[allow(clippy::unreachable)]
        (_, _, _, 3..=u32::MAX) | (false, true, false, 2..=u32::MAX) | (true, true, false, 1..=2) => unreachable!()
    }
}
