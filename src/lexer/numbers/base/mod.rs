pub mod binary;
pub mod decimal;
pub mod hexadecimal;
pub mod octal;

macro_rules! parse_int_from_radix {
    ($location:ident, $nb_type:ident, $literal:tt, $reason:expr, $radix:expr, $($t:ident)*) => {
        match $nb_type {
            _ if !$nb_type.is_int() => Err(to_error!($location, "{ERR_PREFIX}{}, but found a `{}`", $reason, $nb_type)),
            $(NumberType::$t => Ok(Number::$t( $crate::lexer::safe_parse_int!(ERR_PREFIX, $t, $location, $t::from_str_radix($literal, $radix))? )),)*
            _ => panic!("this is unreachable")
        }
    };
}

#[allow(clippy::pub_use)]
pub(super) use parse_int_from_radix;
