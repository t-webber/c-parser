//! Module to store a compilation error
//!
//! This crate implements the [`CompileError`] struct and its methods.

use core::fmt;

use crate::errors::api::Location;

/// Struct to store the error information
///
/// # Creation
///
/// To create an error, you need to have the [`Location`] of the error. Then,
/// use the methods on that location, for example:
///
/// ```ignore
/// let location = Location::from("filename.c");
/// let error = location.to_failure("Something bad happened here.".to_owned());
/// ```
///
/// To see the others methods to create errors see [`Location`].
///
/// # Usage
///
/// The [`CompileError`] is mainly used as part of a
/// [`Res`](super::result::Res).
#[derive(Debug)]
pub struct CompileError {
    /// Severity of the error
    err_lvl: ErrorLevel,
    /// Location of the error in the C source file
    location: Location,
    /// Error message to be displayed to the user
    message: String,
}

impl CompileError {
    /// Returns the referenced data of a `CompileError`.
    pub(super) fn get_values(&self) -> (&Location, &str, String) {
        (&self.location, &self.message, self.err_lvl.to_string())
    }

    /// Checks if the error is of severity [`ErrorLevel::Failure`].
    pub(crate) fn is_failure(&self) -> bool {
        self.err_lvl == ErrorLevel::Failure
    }
}

impl From<(Location, String, ErrorLevel)> for CompileError {
    #[inline]
    fn from((location, message, err_lvl): (Location, String, ErrorLevel)) -> Self {
        Self {
            err_lvl,
            location,
            message,
        }
    }
}

/// Different levels of errors
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorLevel {
    /// The compiler stops compiling the current block and fails.
    ///
    /// The level is only `Failure` when the compiler can't fix the error and
    /// panics.
    ///
    /// The compiler will continue if it manages to do so safely on parts that
    /// are independent from the original location of the error. Not all of the
    /// independent parts are compiled though.
    Failure,
    /// Found a bad practice.
    ///
    /// # Examples
    ///
    /// - a leading space after `\` at end of line
    Suggestion,
    /// The compiler manages to fix the code and continue.
    ///
    /// A warning is displayed to the user, but the compiler continues as
    /// nothing happened.
    ///
    /// # Examples
    ///
    /// - an overflow on a integer constant: the value is crapped and the
    ///   compiler continues
    /// - deprecated behaviours (e.g. using `_Bool` instead of `bool` in C23).
    Warning,
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for ErrorLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failure => "error".fmt(f),
            Self::Suggestion => "suggestion".fmt(f),
            Self::Warning => "warning".fmt(f),
        }
    }
}
