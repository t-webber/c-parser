pub mod binary;
pub mod decimal;
pub mod hexadecimal;
pub mod octal;

#[macro_export]
macro_rules! safe_parse_int {
    ($dest_type:ident, $location:ident, $function_call:expr) => {{
        let parsed: Result<$dest_type, core::num::ParseIntError> = $function_call.map_err(|err| err.into());
        match parsed {
            Ok(nb) => Ok(nb),
            Err(err) => match *err.kind() {
                core::num::IntErrorKind::Empty => panic!("Never happens. Checks for non empty."),
                core::num::IntErrorKind::InvalidDigit => Err(to_error!(
                    $location,
                    "{ERR_PREFIX}invalid decimal number: must contain only ascii digits and at most one '.', one 'e' followed by at most a sign."
                )),
                core::num::IntErrorKind::PosOverflow => Err(to_error!(
                    $location,
                    "{ERR_PREFIX}postive overflow on decimal number: number is too large to fit in attributed type. Add a suffix or reduce value."
                )),
                core::num::IntErrorKind::NegOverflow => Err(to_error!(
                    $location,
                    "{ERR_PREFIX}negative overflow on decimal number: number is too large to fit in attributed type. Add a suffix or reduce value."
                )),
                core::num::IntErrorKind::Zero | _ => panic!("Unexpected error"),

            },
        }
    }};
}

#[macro_export]
macro_rules! parse_int_from_radix {
    ($location:ident, $nb_type:ident, $literal:tt, $reason:expr, $radix:expr, $($t:ident)*) => {
        match $nb_type {
            _ if !$nb_type.is_int() => Err(to_error!($location, "{ERR_PREFIX}{}, but found a `{}`", $reason, $nb_type)),
            $(NumberType::$t => Ok(Number::$t( $crate::safe_parse_int!($t, $location, $t::from_str_radix($literal, $radix))? )),)*
            _ => panic!("this is unreachable")
        }
    };
}
