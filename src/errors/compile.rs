//! Module to store a compilation error
//!
//! This crate implements the [`CompileError`] struct and its methods.

use crate::errors::api::ErrorLocation;
use crate::utils::display;

/// Struct to store the error information
///
/// # Creation
///
/// To create an error, you need to have the [`ErrorLocation`] of the error.
/// Then, use the methods on that location, for example:
///
/// ```ignore
/// let location = ErrorLocation::from("filename.c");
/// let error = location.to_failure("Something bad happened here.".to_owned());
/// ```
///
/// To see the others methods to create errors see [`ErrorLocation`].
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
    location: ErrorLocation,
    /// Error message to be displayed to the user
    message: String,
}

impl CompileError {
    /// Returns the referenced data of a `CompileError`.
    pub(super) fn as_values(&self) -> (&ErrorLocation, &str, String) {
        (&self.location, &self.message, self.err_lvl.to_string())
    }

    /// Checks if the error is a failure.
    ///
    /// A failure is when the compiler will stop at the end of the current step.
    /// It is represented either by a [`ErrorLevel::Crash`] or a
    /// [`ErrorLevel::Fault`].
    pub(crate) const fn is_failure(&self) -> bool {
        matches!(self.err_lvl, ErrorLevel::Crash | ErrorLevel::Fault)
    }
}

impl From<(ErrorLocation, String, ErrorLevel)> for CompileError {
    fn from((location, message, err_lvl): (ErrorLocation, String, ErrorLevel)) -> Self {
        Self { err_lvl, location, message }
    }
}

/// Different levels of errors
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ErrorLevel {
    /// The compiler stops compiling and fails.
    ///
    /// The level is only [`ErrorLevel::Crash`] when the compiler can't fix the
    /// error, and doesn't know what will happen next. The compile thus
    /// can't process the rest of the code properly without being influenced
    /// by the unrecognised pattern it just saw. Thus, a [`ErrorLevel::Crash`]
    /// leads to an immediate panic.
    ///
    /// # Examples
    ///
    /// - a missing brace in parser: if the brace is missing, the compiler can't
    ///   decide where it was supposed to be, and parsing is very different if
    ///   there is or not a brace.
    Crash,
    /// The compiler stops compiling the current block.
    ///
    /// The compiler will continue if it manages to do so safely on parts that
    /// are independent from the original location of the error. Not all of the
    /// independent parts are compiled though.
    ///
    /// # Examples
    ///
    /// - an invalid number: the compiler stills knows how to parse the rest
    ///   because it isolated the number token.
    Fault,
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

display!(
    ErrorLevel,
    self,
    f,
    match self {
        Self::Crash | Self::Fault => "error".fmt(f),
        Self::Suggestion => "suggestion".fmt(f),
        Self::Warning => "warning".fmt(f),
    }
);
