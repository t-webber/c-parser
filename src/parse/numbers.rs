use core::fmt;

type Int = i32;
#[cfg(target_pointer_width = "32")]
type Long = Int;
#[cfg(target_pointer_width = "64")]
type Long = LongLong;
type LongLong = i64;
type Float = f32;
type Double = f64;
type LongDouble = f128;
type UInt = u32;
#[cfg(target_pointer_width = "32")]
type ULong = UiIt;
#[cfg(target_pointer_width = "64")]
type ULong = ULongLong;
type ULongLong = u64;

macro_rules! define_nb_types {
    ($($t:ident)*) => {
        pub enum Number {
            $($t($t),)*
        }

        pub enum NumberType {
            $($t,)*
        }

    };
}

define_nb_types!(Int Long LongLong Float Double LongDouble UInt ULong ULongLong);

macro_rules! parse_from_radix {
    ($nb_type:ident, $literal:tt, $radix:expr, $($t:ident)*) => {
        match $nb_type {
            _ if !$nb_type.is_int() => Err(format!("{ERR_PREFIX}a binary must be an integer, but found a `{}`", $nb_type)),
            $(NumberType::$t => Ok(Number::$t($t::from_str_radix($literal, $radix).expect("2 <= radix <= 36"))),)*
            _ => unreachable!()
        }
    };
}

impl NumberType {
    const fn is_int(&self) -> bool {
        !matches!(self, Self::Double | Self::Float | Self::LongDouble)
    }

    const fn suffix_size(&self) -> usize {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Int => 0,
            Self::Long => 1,
            Self::LongLong => 2,
            Self::Float => 1,
            Self::Double => 0,
            Self::LongDouble => 1,
            Self::UInt => 1,
            Self::ULong => 2,
            Self::ULongLong => 3,
        }
    }
}

impl fmt::Display for NumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NumberType::Int => "int",
                NumberType::Long => "long",
                NumberType::LongLong => "long long",
                NumberType::Float => "float",
                NumberType::Double => "double",
                NumberType::LongDouble => "long double",
                NumberType::UInt => "unsigned int",
                NumberType::ULong => "unsigned long",
                NumberType::ULongLong => "unsigned long long",
            }
        )
    }
}

enum Base {
    Binary,
    Decimal,
    Hexadecimal,
    Octal,
}

impl Base {
    const fn prefix_size(&self) -> usize {
        match self {
            Self::Binary | Self::Hexadecimal => 2,
            Self::Decimal => 0,
            Self::Octal => 1,
        }
    }
}

type OptionalReturn = Result<Option<Number>, String>;
type Return = Result<Number, String>;
static ERR_PREFIX: &str = "Invalid number constant type: ";

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

fn to_hex_value(literal: &str, nb_type: &NumberType) -> Return {}
fn to_decimal_value(literal: &str, nb_type: &NumberType) -> Return {}

fn to_oct_value(literal: &str, nb_type: &NumberType) -> Return {
    if !literal.chars().all(|ch| matches!(ch, '0'..='7')) {
        let first = literal
            .chars()
            .find(|ch| matches!(ch, '0'..='7'))
            .expect("Exists according to line above");
        Err(format!("{ERR_PREFIX}a octal constant must only contain digits between '0' and '7'. Found invalid character '{first}'."))
    } else {
        parse_from_radix!(
           nb_type, literal, 8, Int Long LongLong UInt ULong ULongLong
        )
    }
}
fn to_bin_value(literal: &str, nb_type: &NumberType) -> Return {
    if !literal.chars().all(|ch| matches!(ch, '0' | '1')) {
        let first = literal
            .chars()
            .find(|ch| matches!(ch, '0' | '1'))
            .expect("Exists according to line above");
        Err(format!("{ERR_PREFIX}a binary constant must only contain '0's and '1's. Found invalid character '{first}'."))
    } else {
        parse_from_radix!(
           nb_type, literal, 2, Int Long LongLong UInt ULong ULongLong
        )
    }
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
