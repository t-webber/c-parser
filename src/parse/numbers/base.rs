use core::{
    num::{IntErrorKind, ParseIntError},
    str::FromStr,
};
use std::num::ParseFloatError;

use crate::{
    errors::{compile::CompileError, location::Location},
    to_error, to_warning,
};

use super::types::{
    Double, DoubleIntPart, Float, FloatIntPart, Int, Long, LongDouble, LongDoubleIntPart, LongLong,
    Number, NumberType, UInt, ULong, ULongLong, ERR_PREFIX,
};

macro_rules! parse_int_from_radix {
    ($location:ident, $nb_type:ident, $literal:tt, $reason:expr, $radix:expr, $($t:ident)*) => {
        match $nb_type {
            _ if !$nb_type.is_int() => Err(to_error!($location, "{ERR_PREFIX}{}, but found a `{}`", $reason, $nb_type)),
            $(NumberType::$t => Ok(Number::$t($t::from_str_radix($literal, $radix).expect("2 <= radix <= 36"))),)*
            _ => panic!("this is unreachable")
        }
    };
}

fn parse_int_and_error<T>(literal: &str, location: &Location) -> Result<T, CompileError>
where
    T: FromStr,
    <T as FromStr>::Err: Into<ParseIntError>,
{
    let parsed: Result<T, ParseIntError> = literal.parse::<T>().map_err(Into::into);
    match parsed {
        Ok(nb) => Ok(nb),
        Err(err) =>  match *err.kind() {
            IntErrorKind::Empty => panic!("Never happens. Checks for non empty."),
            IntErrorKind::InvalidDigit => Err(to_error!(
                location,
                "{ERR_PREFIX}invalid decimal number: must contain only ascii digits and at most one '.', one 'e' followed by at most a sign."
            )),
            IntErrorKind::PosOverflow => Err(to_error!(
                location,
                "{ERR_PREFIX}postive overflow on decimal number: number is too large to fit in attributed type. Add a suffix or reduce value."
            )),
            IntErrorKind::NegOverflow => Err(to_error!(
                location,
                "{ERR_PREFIX}negative overflow on decimal number: number is too large to fit in attributed type. Add a suffix or reduce value."
            )),
            IntErrorKind::Zero | _ => panic!("I don't know what this is"),
            },
    }
}

fn parse_and_error<T>(literal: &str, location: &Location) -> Result<T, CompileError>
where
    T: FromStr,
    <T as FromStr>::Err: Into<ParseFloatError>,
{
    literal
        .parse::<T>()
        .map_err(|_err| to_error!(location, "{ERR_PREFIX}invalid decimal float number."))
}

macro_rules! parse_number {
    ($location:ident, $nb_type:ident, $literal:tt, $($int:ident)*, $($float:ident)*) => {
        match $nb_type {
            NumberType::LongDouble => Err(to_error!($location, "{ERR_PREFIX}`long double` not supported yet.")), //TODO: f128 not implemented
            $(NumberType::$int => Ok(Number::$int(parse_int_and_error::<$int>($literal, $location)?)),)*
            $(NumberType::$float => Ok(Number::$float(parse_and_error::<$float>($literal, $location)?)),)*
        }
    };
}

type Return = Result<Number, CompileError>;

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
    type Unsigned;
    fn from_unsigned(val: T, location: &Location, warning: &mut Option<CompileError>) -> Self;
    fn from_usize(val: usize, location: &Location, warning: &mut Option<CompileError>) -> Self;
}

macro_rules! impl_floating_point {
    ($ftype:ident, $x:expr) => {
        #[allow(clippy::as_conversions, clippy::cast_precision_loss)]
        impl FloatingPoint<concat_idents!($ftype, IntPart)> for $ftype {
            type Unsigned = concat_idents!($ftype, IntPart);

            const MANTISSA_SIZE: u32 = $x;

            fn from_unsigned(
                val: Self::Unsigned,
                location: &Location,
                warning: &mut Option<CompileError>,
            ) -> Self {
                if val >= (2 as Self::Unsigned).pow(Self::MANTISSA_SIZE) {
                    *warning = Some(to_warning!(
                        location,
                        "value overflow, given number will be crapped"
                    ));
                }
                val as Self
            }

            fn from_usize(
                val: usize,
                location: &Location,
                warning: &mut Option<CompileError>,
            ) -> Self {
                if val >= 2usize.pow(Self::MANTISSA_SIZE) {
                    *warning = Some(to_warning!(
                        location,
                        "value overflow, given number will be crapped"
                    ));
                }
                val as Self
            }
        }
    };
}

impl_floating_point!(Float, 23);
impl_floating_point!(Double, 53);
impl_floating_point!(LongDouble, 113);

macro_rules! parse_hexadecimal_float {
    ($warning:expr, $location:ident, $nb_type:ident, $float_parse:ident, $($t:ident)*) => {{
        match $nb_type {
            $(NumberType::$t => {
                let int_part = $t::from_unsigned(
                    <concat_idents!($t, IntPart)>::from_str_radix(&$float_parse.int_part, 16).expect("2 <= <= 36"),
                    $location, $warning);
                #[allow(clippy::as_conversions)]
                let exponent = $t::from_unsigned((2 as concat_idents!($t, IntPart)).pow(
                    $float_parse
                        .exponent
                        .parse()
                        .expect("never fails: contains only ascii digits"),
                ), $location, $warning);
                let mut decimal_part: $t = 0.;
                for (idx, ch) in $float_parse.decimal_part.chars().enumerate() {
                    decimal_part += $t::from_unsigned(hex_char_to_int(ch).into(), $location, $warning)
                        / ($t::from(16.).powf($t::from_usize(idx, $location, $warning)));
                }
                Number::$t(int_part + exponent + decimal_part)
            },)*
            _ => panic!("Never happens: nb_type is float"),
        }
    }};
}

#[allow(clippy::panic_in_result_fn)]
pub fn to_hex_value(literal: &str, nb_type: &NumberType, location: &Location) -> Return {
    let mut float_parse = FloatParse::default();
    for ch in literal.chars() {
        match ch {
            _ if float_parse.state == FloatParseState::Exponent && ch.is_ascii_digit() => float_parse.push(ch),
            _ if float_parse.state == FloatParseState::Exponent => return Err(to_error!(location, "{ERR_PREFIX}invalid character for exponent. Expected an ascii digit, but found '{ch}'")),
            _ if ch.is_ascii_hexdigit() => float_parse.push(ch),
            '.' if float_parse.state == FloatParseState::Int => float_parse.state = FloatParseState::Decimal,
            '.' if float_parse.state == FloatParseState::Decimal  => return Err(to_error!(location, "{ERR_PREFIX}maximum one '.' in number constant, but 2 were found.")), 
            '.' if float_parse.state == FloatParseState::Exponent  => return Err(to_error!(location, "{ERR_PREFIX}exponent must be an integer, but found a period.")), 
            'p' | 'P' if float_parse.state == FloatParseState::Exponent => return Err(to_error!(location, "{ERR_PREFIX}maximum one 'p' in number constant, but 2 were found.")), 
            'p' | 'P'  => float_parse.push(ch),
            _ => return Err(to_error!(location, "{ERR_PREFIX}invalid character '{ch}' found in number constant")), 
        }
    }
    if nb_type.is_int() {
        parse_int_from_radix!(location,
           nb_type, literal, "never fails", 16, Int Long LongLong UInt ULong ULongLong
        )
    } else {
        let mut warning: Option<CompileError> = None;
        #[allow(clippy::float_arithmetic, clippy::wildcard_enum_match_arm)]
        Ok(
            parse_hexadecimal_float!(&mut warning, location, nb_type, float_parse, Float Double LongDouble),
        )
    }
}

pub fn to_decimal_value(literal: &str, nb_type: &NumberType, location: &Location) -> Return {
    parse_number!(location,  nb_type, literal, Int Long LongLong UInt ULong ULongLong, Float Double )
}

pub fn to_oct_value(literal: &str, nb_type: &NumberType, location: &Location) -> Return {
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

pub fn to_bin_value(literal: &str, nb_type: &NumberType, location: &Location) -> Return {
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
