macro_rules! safe_parse_int {
    ($err_prefix:expr, $dest_type:ident, $location:ident, $function_call:expr) => {{
        use $crate::lexer::numbers::parse::ParseResult;
        let parsed: Result<$dest_type, core::num::ParseIntError> = $function_call.map_err(|err| err.into());
        match parsed {
            Ok(nb) => ParseResult::from(nb),
            Err(err) => match *err.kind() {
                core::num::IntErrorKind::Empty => panic!("Never happens. Checks for non empty."),
                core::num::IntErrorKind::InvalidDigit => ParseResult::from(to_error!(
                    $location,
                    "{}invalid decimal number: must contain only ascii digits and at most one '.', one 'e' followed by at most a sign."
                , $err_prefix)),
                core::num::IntErrorKind::PosOverflow => ParseResult::from_pos_overflow(),
                core::num::IntErrorKind::NegOverflow => ParseResult::from_neg_overflow(),
                core::num::IntErrorKind::Zero | _ => panic!("Unexpected error"),
            },
        }
    }};
}

use crate::{
    errors::compile::{to_warning, CompileError},
    errors::result::Res,
    prelude::Location,
};
use core::{convert, fmt, ops};
pub(crate) use safe_parse_int;

use super::{to_error, Number};

pub enum ParseResult<T> {
    Value(T),
    Err(CompileError),
    ValueErr(T, CompileError),
    ValueOverflow(T),
    Overflow,
}
pub enum NoOverflowParseResult<T> {
    Value(T),
    Err(CompileError),
    ValueErr(T, CompileError),
}

impl<T> From<CompileError> for ParseResult<T> {
    fn from(value: CompileError) -> Self {
        Self::Err(value)
    }
}

impl<T: fmt::Display> From<T> for ParseResult<T> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}

impl<T> ParseResult<T> {
    pub fn ignore_overflow(self, value: &str, location: &Location) -> NoOverflowParseResult<T> {
        match self {
            // ParseResult::Value(_) | ParseResult::Err(_) | ParseResult::ValueErr(..) => self,
            Self::ValueOverflow(val) => NoOverflowParseResult::ValueErr(
                val,
                to_warning!(
                    location,
                    "Overflow: {value} is too big in traditional number"
                ),
            ),
            Self::Overflow => NoOverflowParseResult::Err(to_error!(
                location,
                "Overflow: {value} is too big in traditional number"
            )),
            Self::Value(val) => NoOverflowParseResult::Value(val),
            Self::Err(compile_error) => NoOverflowParseResult::Err(compile_error),
            Self::ValueErr(val, compile_error) => {
                NoOverflowParseResult::ValueErr(val, compile_error)
            }
        }
    }

    pub const fn overflowed(&self) -> bool {
        matches!(self, Self::ValueOverflow(_) | Self::Overflow)
    }

    pub fn add_overflow(self) -> Self {
        match self {
            Self::Value(val) => Self::ValueOverflow(val),
            Self::Err(_) | Self::ValueErr(..) | // TODO: not adding overflow !
            Self::ValueOverflow(..) | Self::Overflow => self,
        }
    }

    #[allow(clippy::min_ident_chars)]
    pub fn map<F, U>(self, f: F) -> ParseResult<U>
    where
        F: Fn(T) -> U,
    {
        match self {
            Self::Value(val) => ParseResult::Value(f(val)),
            Self::Overflow => ParseResult::Overflow,
            Self::Err(err) => ParseResult::Err(err),
            Self::ValueOverflow(val) => ParseResult::ValueOverflow(f(val)),
            Self::ValueErr(val, err) => ParseResult::ValueErr(f(val), err),
        }
    }

    pub const fn from_pos_overflow() -> Self {
        Self::Overflow
    }

    pub const fn from_neg_overflow() -> Self {
        Self::Overflow
    }
}

impl<T> NoOverflowParseResult<T> {
    fn into_elts(self) -> (Option<T>, Option<CompileError>) {
        match self {
            Self::Value(value) => (Some(value), None),
            Self::Err(error) => (None, Some(error)),
            Self::ValueErr(value, error) => (Some(value), Some(error)),
        }
    }

    #[allow(clippy::min_ident_chars)]
    pub fn map_or_else<U, D: FnMut(CompileError), F: Fn(T) -> U>(
        self,
        mut default: D,
        f: F,
    ) -> Result<U, ()> {
        let (value, error) = self.into_elts();
        if let Some(err) = error {
            default(err);
        };
        value.map(f).ok_or(())
    }

    pub fn into_res(self) -> Res<Option<T>> {
        let (value, error) = self.into_elts();
        Res::from((value, error.map_or_else(Vec::new, |err| vec![err])))
    }
}

impl ops::FromResidual<Result<convert::Infallible, CompileError>> for ParseResult<Number> {
    fn from_residual(residual: Result<convert::Infallible, CompileError>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(err) => Self::Err(err),
        }
    }
}
