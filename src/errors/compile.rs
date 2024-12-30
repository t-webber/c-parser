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
/// let error = location.to_error("Something bad happened here.".to_owned());
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
    /// Length of the erroneous token or expression
    length: usize,
    /// Location of the error in the C source file
    location: Location,
    /// Error message to be displayed to the user
    message: String,
}

impl CompileError {
    /// Returns the owned data of a `CompileError`.
    pub(super) fn into_values(self) -> (Location, String, String, usize) {
        (
            self.location,
            self.message,
            self.err_lvl.to_string(),
            self.length,
        )
    }

    /// Checks if the error is of severity [`ErrorLevel::Error`].
    pub(crate) fn is_error(&self) -> bool {
        self.err_lvl == ErrorLevel::Error
    }

    // Replaces length of the token or expression concerned by the `CompileError`.
    pub(crate) fn specify_length(&mut self, length: usize) {
        self.length = length;
    }
}

impl From<(Location, String, ErrorLevel, usize)> for CompileError {
    #[inline]
    fn from((location, message, err_lvl, length): (Location, String, ErrorLevel, usize)) -> Self {
        Self {
            err_lvl,
            length,
            location,
            message,
        }
    }
}

impl From<(Location, String, ErrorLevel)> for CompileError {
    #[inline]
    fn from((location, message, err_lvl): (Location, String, ErrorLevel)) -> Self {
        Self {
            message,
            length: 0,
            location,
            err_lvl,
        }
    }
}

/// Different levels of errors
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorLevel {
    /// The compiler stops compiling the current block and fails.
    ///
    /// The level is only `Error` when the compiler can't fix the error and
    /// panics.
    ///
    /// The compiler will continue if it manages to do so safely on parts that
    /// are independent from the original location of the error. Not all of the
    /// independent parts are compiled though.
    Error,
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
            Self::Error => "error".fmt(f),
            Self::Suggestion => "suggestion".fmt(f),
            Self::Warning => "warning".fmt(f),
        }
    }
}
