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

use core::{convert, fmt, ops};

pub(crate) use safe_parse_int;

use super::Number;
use crate::errors::compile::CompileError;
use crate::errors::result::Res;
use crate::prelude::Location;

/// Number parse result with overflow
pub enum OverParseRes<T> {
    Value(T),
    Err(CompileError),
    ValueErr(T, CompileError),
    ValueOverflow(T),
    Overflow,
}

// Number parse result without overflow
pub enum ParseRes<T> {
    Value(T),
    Err(CompileError),
    ValueErr(T, CompileError),
}

impl<T> From<CompileError> for OverParseRes<T> {
    fn from(value: CompileError) -> Self {
        Self::Err(value)
    }
}

impl<T: fmt::Display> From<T> for OverParseRes<T> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}

impl<T> OverParseRes<T> {
    pub fn ignore_overflow(self, value: &str, location: &Location) -> ParseRes<T> {
        match self {
            // OverParseRes::Value(_) | OverParseRes::Err(_) | OverParseRes::ValueErr(..) => self,
            Self::ValueOverflow(val) => ParseRes::ValueErr(
                val,
                location.to_warning(format!(
                    "Overflow: {value} is too big in traditional number"
                )),
            ),
            Self::Overflow => ParseRes::Err(location.to_error(format!(
                "Overflow: {value} is too big in traditional number"
            ))),
            Self::Value(val) => ParseRes::Value(val),
            Self::Err(compile_error) => ParseRes::Err(compile_error),
            Self::ValueErr(val, compile_error) => ParseRes::ValueErr(val, compile_error),
        }
    }

    pub const fn overflowed(&self) -> bool {
        matches!(self, Self::ValueOverflow(_) | Self::Overflow)
    }

    /// Adds an overflow warning to the current result
    ///
    /// The warning is not added if the result is already an error and doesn't
    /// contain any value.
    pub fn add_overflow(self) -> Self {
        match self {
            Self::Value(val) => Self::ValueOverflow(val),
            Self::Err(_) | Self::ValueErr(..) | Self::ValueOverflow(..) | Self::Overflow => self,
        }
    }

    #[allow(clippy::min_ident_chars)]
    pub fn map<F, U>(self, f: F) -> OverParseRes<U>
    where
        F: Fn(T) -> U,
    {
        match self {
            Self::Value(val) => OverParseRes::Value(f(val)),
            Self::Overflow => OverParseRes::Overflow,
            Self::Err(err) => OverParseRes::Err(err),
            Self::ValueOverflow(val) => OverParseRes::ValueOverflow(f(val)),
            Self::ValueErr(val, err) => OverParseRes::ValueErr(f(val), err),
        }
    }

    pub const fn from_pos_overflow() -> Self {
        Self::Overflow
    }

    pub const fn from_neg_overflow() -> Self {
        Self::Overflow
    }
}

impl<T> ParseRes<T> {
    #[allow(clippy::min_ident_chars)]
    pub fn edit_err<F: Fn(&mut CompileError)>(&mut self, f: F) {
        match self {
            Self::Value(_) => (),
            Self::Err(err) | Self::ValueErr(_, err) => f(err),
        }
    }

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

impl ops::FromResidual<Result<convert::Infallible, CompileError>> for OverParseRes<Number> {
    fn from_residual(residual: Result<convert::Infallible, CompileError>) -> Self {
        match residual {
            Ok(_) => unreachable!(/* Infallible = ! */),
            Err(err) => Self::Err(err),
        }
    }
}

impl<T> ops::FromResidual<Result<convert::Infallible, CompileError>> for ParseRes<T> {
    fn from_residual(residual: Result<convert::Infallible, CompileError>) -> Self {
        match residual {
            Ok(_) => unreachable!(/* Infallible = ! */),
            Err(err) => Self::Err(err),
        }
    }
}
