#![allow(clippy::arbitrary_source_item_ordering)]
#![allow(clippy::pub_use)]

macro_rules! parse_int_from_radix {
    ($location:ident, $nb_type:ident, $literal:tt, $reason:expr, $radix:expr, $($t:ident)*) => {{
        use $crate::lexer::numbers::{macros::safe_parse_int, parse::OverParseRes};
        match $nb_type {
            _ if !$nb_type.is_int() => OverParseRes::Err($location.to_error(format!("{ERR_PREFIX}{}, but found a `{}`", $reason, $nb_type))),
            $(NumberType::$t => safe_parse_int!(ERR_PREFIX, $t, $location, $t::from_str_radix($literal, $radix)).map(|nb| Number::$t(nb)),)*
            _ => panic!("this is unreachable")
        }
    }};
}

macro_rules! safe_parse_int {
    ($err_prefix:expr, $dest_type:ident, $location:ident, $function_call:expr) => {{
        use $crate::lexer::numbers::parse::OverParseRes;
        let parsed: Result<$dest_type, core::num::ParseIntError> = $function_call.map_err(|err| err.into());
        match parsed {
            Ok(nb) => OverParseRes::from(nb),
            Err(err) => match *err.kind() {
                core::num::IntErrorKind::Empty => panic!("Never happens. Checks for non empty."),
                core::num::IntErrorKind::InvalidDigit => OverParseRes::from($location.to_error(format!(
                    "{}invalid decimal number: must contain only ascii digits and at most one '.', one 'e' followed by at most a sign."
                , $err_prefix))),
                core::num::IntErrorKind::PosOverflow => OverParseRes::from_pos_overflow(),
                core::num::IntErrorKind::NegOverflow => OverParseRes::from_neg_overflow(),
                core::num::IntErrorKind::Zero | _ => panic!("Unexpected error"),
            },
        }
    }};
}

pub(super) use parse_int_from_radix;
pub(crate) use safe_parse_int;
