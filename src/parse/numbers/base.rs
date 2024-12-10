use super::types::{
    Double, DoubleIntPart, Float, FloatIntPart, Int, Long, LongDouble, LongDoubleIntPart, LongLong,
    Number, NumberType, Return, UInt, ULong, ULongLong, ERR_PREFIX,
};

macro_rules! parse_int_from_radix {
    ($nb_type:ident, $literal:tt, $reason:expr, $radix:expr, $($t:ident)*) => {
        match $nb_type {
            _ if !$nb_type.is_int() => Err(format!("{ERR_PREFIX}{}, but found a `{}`", $reason, $nb_type)),
            $(NumberType::$t => Ok(Number::$t($t::from_str_radix($literal, $radix).expect("2 <= radix <= 36"))),)*
            _ => unreachable!()
        }
    };
}

// macro_rules! parse_number {
//     ($nb_type:ident, $literal:tt, $($t:ident)*) => {
//         match $nb_type {
//             $(NumberType::$t => Ok(Number::$t($literal.to_string().parse()?)),)*
//             _ => unreachable!()
//         }
//     };
// }

#[derive(Default, PartialEq, Eq)]
enum FloatParseState {
    #[default]
    Int,
    Decimal,
    Exponent,
}

#[derive(Default)]
struct FloatParse {
    int_part: String,
    decimal_part: String,
    exponent: String,
    state: FloatParseState,
}

impl FloatParse {
    fn push(&mut self, ch: char) {
        match self.state {
            FloatParseState::Int => self.int_part.push(ch),
            FloatParseState::Decimal => self.decimal_part.push(ch),
            FloatParseState::Exponent => self.exponent.push(ch),
        }
    }
}

fn hex_char_to_int(ch: char) -> u8 {
    match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' | 'A' => 10,
        'b' | 'B' => 11,
        'c' | 'C' => 12,
        'd' | 'D' => 13,
        'e' | 'E' => 14,
        'f' | 'F' => 15,
        _ => panic!("function called on non hex char"),
    }
}

trait FloatingPoint<T> {
    const MANTISSA_SIZE: u32;
    fn from_unsigned(val: T) -> Self;
    fn from_usize(val: usize) -> Self;
}

macro_rules! impl_floating_point {
    ($ftype:ident, $utype:ident, $x:expr) => {
        #[allow(clippy::as_conversions, clippy::cast_precision_loss)]
        impl FloatingPoint<$utype> for $ftype {
            const MANTISSA_SIZE: u32 = $x;

            fn from_unsigned(val: $utype) -> Self {
                if val >= (2 as $utype).pow(Self::MANTISSA_SIZE) {
                    //TODO: add a warning to show that the value as been crapped, adding a eprint! before that.
                    eprintln!("crapping float !! not implemented yet.");
                }
                val as Self
            }

            fn from_usize(val: usize) -> Self {
                if val >= 2usize.pow(Self::MANTISSA_SIZE) {
                    //TODO: add a warning to show that the value as been crapped, adding a eprint! before that.
                    eprintln!("crapping float !! not implemented yet.");
                }
                val as Self
            }
        }
    };
}

impl_floating_point!(Float, FloatIntPart, 23);
impl_floating_point!(Double, DoubleIntPart, 53);
impl_floating_point!(LongDouble, LongDoubleIntPart, 113);

#[allow(clippy::panic_in_result_fn)]
pub fn to_hex_value(literal: &str, nb_type: &NumberType) -> Return {
    let err_prefix = ERR_PREFIX.to_owned();
    let mut float_parse = FloatParse::default();
    for ch in literal.chars() {
        match ch {
            _ if float_parse.state == FloatParseState::Exponent && ch.is_ascii_digit() => float_parse.push(ch),
            _ if float_parse.state == FloatParseState::Exponent => return Err(format!("{ERR_PREFIX}invalid character for exponent. Expected an ascii digit, but found '{ch}'")),
            _ if ch.is_ascii_hexdigit() => float_parse.push(ch),
            '.' if float_parse.state == FloatParseState::Int => float_parse.state = FloatParseState::Decimal,
            '.' if float_parse.state == FloatParseState::Decimal  => return Err(err_prefix + "maximum one '.' in number constant, but 2 were found."), 
            '.' if float_parse.state == FloatParseState::Exponent  => return Err(err_prefix + "exponent must be an integer, but found a period."), 
            'p' | 'P' if float_parse.state == FloatParseState::Exponent => return Err(err_prefix + "maximum one 'p' in number constant, but 2 were found."), 
            'p' | 'P'  => float_parse.push(ch),
            _ => return Err(format!("{ERR_PREFIX}invalid character '{ch}' found in number constant")), 
        }
    }
    if nb_type.is_int() {
        parse_int_from_radix!(
           nb_type, literal, "never fails", 16, Int Long LongLong UInt ULong ULongLong
        )
    } else {
        #[allow(clippy::float_arithmetic, clippy::wildcard_enum_match_arm)]
        Ok(match nb_type {
            NumberType::Float => {
                let int_part = Float::from_unsigned(
                    FloatIntPart::from_str_radix(&float_parse.int_part, 16).expect("2 <= <= 36"),
                );
                let exponent = Float::from(2.).powf(
                    float_parse
                        .exponent
                        .parse()
                        .expect("never fails: contains only ascii digits"),
                );
                let mut decimal_part: Float = 0.;
                for (idx, ch) in float_parse.decimal_part.chars().enumerate() {
                    decimal_part += Float::from(hex_char_to_int(ch))
                        / (Float::from(16.).powf(Float::from_usize(idx)));
                }
                Number::Float(int_part + exponent + decimal_part)
            }
            NumberType::Double => {
                let int_part = Double::from_unsigned(
                    DoubleIntPart::from_str_radix(&float_parse.int_part, 16).expect("2 <= <= 36"),
                );
                let exponent = Double::from(2.).powf(
                    float_parse
                        .exponent
                        .parse()
                        .expect("never fails: contains only ascii digits"),
                );
                let mut decimal_part: Double = 0.;
                for (idx, ch) in float_parse.decimal_part.chars().enumerate() {
                    decimal_part += Double::from(hex_char_to_int(ch))
                        / (Double::from(16.).powf(Double::from_usize(idx)));
                }
                Number::Double(int_part + exponent + decimal_part)
            }
            NumberType::LongDouble => {
                let int_part = LongDouble::from_unsigned(
                    LongDoubleIntPart::from_str_radix(&float_parse.int_part, 16)
                        .expect("2 <= <= 36"),
                );
                let exponent = LongDouble::from_unsigned(
                    LongDoubleIntPart::from(2u32).pow(
                        float_parse
                            .exponent
                            .parse()
                            .expect("never fails: contains only ascii digits"),
                    ),
                );
                let mut decimal_part: LongDouble = 0.;
                for (idx, ch) in float_parse.decimal_part.chars().enumerate() {
                    decimal_part += LongDouble::from_unsigned(hex_char_to_int(ch).into())
                        / (LongDouble::from(16.).powf(LongDouble::from_usize(idx)));
                }
                Number::LongDouble(int_part + exponent + decimal_part)
            }

            _ => panic!("Never happens: nb_type is float"),
        })
    }
}

//TODO: some take_while might be erroneous: used as mut but doesn't mutate.

pub fn to_decimal_value(literal: &str, _nb_type: &NumberType) -> Return {
    //TODO: let x = parse_number!( nb_type, literal, Int Long LongLong Float Double LongDouble UInt ULong ULongLong );
    Err(literal.to_owned())
}

pub fn to_oct_value(literal: &str, nb_type: &NumberType) -> Return {
    if literal.chars().all(|ch| matches!(ch, '0'..='7')) {
        parse_int_from_radix!(
           nb_type, literal, "an octal must be an integer", 8, Int Long LongLong UInt ULong ULongLong
        )
    } else {
        let first = literal
            .chars()
            .find(|ch| matches!(ch, '0'..='7'))
            .expect("Exists according to line above");
        Err(format!("{ERR_PREFIX}a octal constant must only contain digits between '0' and '7'. Found invalid character '{first}'."))
    }
}

pub fn to_bin_value(literal: &str, nb_type: &NumberType) -> Return {
    if literal.chars().all(|ch| matches!(ch, '0' | '1')) {
        parse_int_from_radix!(
           nb_type, literal, "a binary must be an integer", 2, Int Long LongLong UInt ULong ULongLong
        )
    } else {
        let first = literal
            .chars()
            .find(|ch| matches!(ch, '0' | '1'))
            .expect("Exists according to line above");
        Err(format!("{ERR_PREFIX}a binary constant must only contain '0's and '1's. Found invalid character '{first}'."))
    }
}
