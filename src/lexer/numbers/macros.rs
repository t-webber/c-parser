//! Module that defines useful macros to parse integer and get useful errors.

#![allow(clippy::arbitrary_source_item_ordering)]
#![allow(clippy::pub_use)]

/// Parses an integer from a given base
///
/// Specify the base with the radix integer, that is 2, 8 or 16.
///
/// For base 10, please use [`safe_parse_int`] for better errors.
macro_rules! parse_int_from_radix {
    ($location:ident, $nb_type:ident, $literal:tt, $reason:expr, $radix:expr, $($t:ident)*) => {{
        use $crate::lexer::numbers::{macros::safe_parse_int, parse::OverParseRes};
        match $nb_type {
            _ if !$nb_type.is_int() => OverParseRes::Err($location.to_failure(format!("{ERR_PREFIX}{}, but found a `{}`", $reason, $nb_type))),
            $(NumberType::$t => safe_parse_int!(ERR_PREFIX, $t, $location, $t::from_str_radix($literal, $radix)).map(|nb| Number::$t(nb)),)*
            _ => panic!("this is unreachable")
        }
    }};
}

/// Parses an decimal integer from a string
macro_rules! safe_parse_int {
    ($err_prefix:expr, $dest_type:ident, $location:ident, $function_call:expr) => {{
        use $crate::lexer::numbers::api::OverParseRes;
        let parsed: Result<$dest_type, core::num::ParseIntError> = $function_call.map_err(|err| err.into());
        match parsed {
            Ok(nb) => OverParseRes::from(nb),
            Err(err) => match *err.kind() {
                core::num::IntErrorKind::Empty => panic!("Never happens. Checks for non empty."),
                core::num::IntErrorKind::InvalidDigit => OverParseRes::from($location.to_failure(format!(
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
