//! Module that defines the result and error types used for parsing a number
//! constant.

use core::fmt;

use crate::errors::api::{CompileError, Location, SingleRes};

/// Number parse result with overflow
///
/// It can contain errors and values at the same time.
///
/// # Note
///
/// If an error occurs, the overflows are ignored (overflows are only warnings
/// not errors.)
#[derive(Debug)]
pub enum OverParseRes<T> {
    /// Number parsing failed
    Err(CompileError),
    /// Number parsing overflowed
    Overflow,
    /// Number parsing succeeded
    Value(T),
    /// Number parsing succeeded; but with a warning
    /// Number parsing succeeded; but with an overflow
    ValueOverflow(T),
}

impl<T> OverParseRes<T> {
    /// Creates a [`OverParseRes`] from an overflow parsing error.
    pub const fn from_overflow() -> Self {
        Self::Overflow
    }

    /// Creates a [`OverParseRes`] from a crapped value and an overflow parsing
    /// error.
    pub const fn from_value_overflow(value: T) -> Self {
        Self::ValueOverflow(value)
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
            Self::Overflow => SingleRes::from(location.to_fault(format!(
                "Overflow: {value} is too big in traditional number"
            ))),
            Self::Value(val) => SingleRes::from(Some(val)),
            Self::Err(compile_error) => SingleRes::from(compile_error),
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
