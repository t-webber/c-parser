//! Module that defines the result and error types used for parsing a number
//! constant.

use core::{convert, fmt, ops};

use super::types::Number;
use crate::errors::api::{CompileError, CompileRes, Location, SingleRes};

/// Number parse result with overflow
///
/// It can contain errors and values at the same time.
///
/// # Note
///
/// If an error occurs, the overflows are ignored (overflows are only warnings
/// not errors.)
pub enum OverParseRes<T> {
    /// Number parsing failed
    Err(CompileError),
    /// Number parsing overflowed
    Overflow,
    /// Number parsing succeeded
    Value(T),
    /// Number parsing succeeded; but with a warning
    ValueErr(T, CompileError),
    /// Number parsing succeeded; but with an overflow
    ValueOverflow(T),
}

impl<T> OverParseRes<T> {
    /// Adds an overflow warning to the current result
    ///
    /// # Note
    ///
    /// The warning is not added if the result is already an error and doesn't
    /// contain any value.
    pub fn add_overflow(self) -> Self {
        match self {
            Self::Value(val) => Self::ValueOverflow(val),
            Self::Err(_) | Self::ValueErr(..) | Self::ValueOverflow(..) | Self::Overflow => self,
        }
    }

    /// Creates a [`OverParseRes`] from a negative overflow parsing error.
    ///
    /// # Note
    ///
    /// The sign is not implemented yet. The user-error will only display
    /// 'overflow error' and not wether it is a positive or negative overflow
    pub const fn from_neg_overflow() -> Self {
        Self::Overflow
    }

    /// Creates a [`OverParseRes`] from a positive overflow parsing error.
    ///
    /// # Note
    ///
    /// The sign is not implemented yet. The user-error will only display
    /// 'overflow error' and not wether it is a positive or negative overflow
    pub const fn from_pos_overflow() -> Self {
        Self::Overflow
    }

    /// Clamps to value if there is an overflow.
    pub fn ignore_overflow(self, value: &str, location: &Location) -> SingleRes<Option<T>> {
        match self {
            Self::ValueOverflow(val) => SingleRes::from((
                Some(val),
                location.to_warning(format!(
                    "Overflow: {value} is too big in traditional number"
                )),
            )),
            Self::Overflow => SingleRes::from(location.to_failure(format!(
                "Overflow: {value} is too big in traditional number"
            ))),
            Self::Value(val) => SingleRes::from(Some(val)),
            Self::Err(compile_error) => SingleRes::from(compile_error),
            Self::ValueErr(val, compile_error) => SingleRes::from((Some(val), compile_error)),
        }
    }

    /// Applies a function to the value, if it exists.
    #[expect(clippy::min_ident_chars)]
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

    /// Checks if an overflow has occurred.
    pub const fn overflowed(&self) -> bool {
        matches!(self, Self::ValueOverflow(_) | Self::Overflow)
    }
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

impl ops::FromResidual<CompileRes<convert::Infallible>> for OverParseRes<Number> {
    fn from_residual(residual: CompileRes<convert::Infallible>) -> Self {
        match residual {
            Ok(_) => panic!(/* Infallible = ! */),
            Err(err) => Self::Err(err),
        }
    }
}
