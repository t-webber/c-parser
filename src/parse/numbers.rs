type INT = i32;
#[cfg(target_pointer_width = "32")]
type Long = Int;
#[cfg(target_pointer_width = "64")]
type Long = Longlong;
type Longlong = i64;
type Float = f32;
type Double = f64;
type Longdouble = f128;
type Uint = u32;
#[cfg(target_pointer_width = "32")]
type Ulong = Uint;
#[cfg(target_pointer_width = "64")]
type Ulong = Ulonglong;
type Ulonglong = u64;

pub enum Number {
    Int(Int),
    Long(Long),
    LongLong(Longlong),
    Float(Float),
    Double(Double),
    LongDouble(Longdouble),
    UInt(Uint),
    ULong(Ulong),
    ULongLong(Ulonglong),
}

enum NumberTypes {
    Int,
    Long,
    LongLong,
    Float,
    Double,
    LongDouble,
    UInt,
    ULong,
    ULongLong,
}

impl NumberTypes {
    const fn is_int(&self) -> bool {
        !matches!(self, Self::Double | Self::Float | Self::LongDouble)
    }
}

static ERR_PREFIX: &str = "Invalid number constant type: ";

pub fn literal_to_number(literal: &str) -> Result<Option<Number>, String> {
    if literal.is_empty() {
        return Ok(None);
    }
    if literal.len() == 1 {
        return Ok(literal
            .parse::<INT>()
            .map_or_else(|_| None, |x| Some(Number::Int(x))));
    }

    let nb_type = get_number_type(literal)?;

    Ok(None)
}

fn get_number_type(literal: &str) -> Result<NumberTypes, String> {
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
        (false, false, false, 0) => Ok(NumberTypes::Int),
        (false, false, false, 1) => Ok(NumberTypes::Long),
        (false, false, false, 2) => Ok(NumberTypes::LongLong),
        (_, _, _, l_c) if l_c >= 3  => {
            Err(err_prefix + "`long long double` doesn't exist.")
        }
        (false, false, true, 0) => Ok(NumberTypes::UInt),
        (false, false, true, 1) => Ok(NumberTypes::ULong),
        (false, false, true, 2) => Ok(NumberTypes::ULongLong),
        (false, true, false, 0) => Ok(NumberTypes::Double),
        (false, true, false, 1) => Ok(NumberTypes::LongDouble),
        (false, true, false, l_c) if l_c >= 2 => {
            Err(err_prefix + "`long long double` doesn't exist.")
        }
        (true, _, true, _) => Err(err_prefix + "a `float` can't be `unsigned`."), // moved up not to be shadowed
        (_, true, true, _) => {
            Err(err_prefix + "a `double` can't be `unsigned`.")
        },
        (true, false, _, _) =>  Err(err_prefix + "a 'f' suffix only works on `double` constants. Please insert a period or an exponent character before the 'f'."),
        (true, true, false, 0)  => Ok(NumberTypes::Float),
        (true, true, false, l_c) if l_c > 0  => Err(err_prefix + "a `float` can't be `long`. Did you mean `long double`? Remove the leading 'f' if that is the case."),
        #[allow(clippy::unreachable)]
        (_, _, _, 3..=u32::MAX) | (false, true, false, 2..=u32::MAX) | (true, true, false, 1..=2) => unreachable!()
    }
}
