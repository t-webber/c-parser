//! Module to store the location and length of the error.
//!
//! This crate implements the [`Location`] struct and its methods.

use super::api::CompileRes;
use super::compile::{CompileError, ErrorLevel};

/// Struct to pinpoint a precise character in the C source file.
///
/// The locations are computed by the lexer that reads the C source file. Then,
/// the locations are stored inside the tokens to keep them for the rest of the
/// compiler.
///
/// # Note
///
/// In order to respect the click links from terminals, the line and column of
/// a file start at 1 and not 0.
#[derive(Debug, Clone)]
pub struct Location {
    /// Abscissa of the begging of the erroneous token.
    col: usize,
    /// Source file of the error.
    file: String,
    /// Horizontal length of the error.
    length: usize,
    /// Ordinate of the error.
    line: usize,
}

impl Location {
    /// Increments column of location by 1
    ///
    /// This is used by lexer when parsing every character of the C file.
    pub(crate) fn incr_col(&mut self) -> CompileRes<()> {
        self.col = self.col.checked_add(1).ok_or_else(|| {
            self.to_failure(format!(
                "This line of code exceeds the maximum numbers of columns ({}).
        Consider refactoring your code.",
                usize::MAX
            ))
        })?;
        Ok(())
    }

    /// Increments line of location by 1
    ///
    /// This is used by lexer when parsing every line of the C file.
    pub(crate) fn incr_line(&mut self) -> CompileRes<()> {
        self.line = self.line.checked_add(1).ok_or_else(|| {
            self.to_failure(format!(
                "The file exceeds the maximum number lines ({}). Consider refactoring
        your code.",
                usize::MAX
            ))
        })?;
        self.col = 1;
        Ok(())
    }

    /// Creates an error from a location without cloning
    pub(crate) fn into_failure(self, msg: String) -> CompileError {
        CompileError::from((self, msg, ErrorLevel::Failure))
    }

    /// Moves the location back a few character on the current line
    ///
    /// If the offset is too big, the column is set to minimal (1) without any
    /// warnings or errors.
    pub(crate) fn into_past_with_length(self, len: usize) -> Self {
        Self {
            col: self.col.checked_sub(len).unwrap_or(1),
            length: len,
            ..self
        }
    }

    /// Returns the owned data of a `Location`.
    pub(crate) fn into_values(self) -> (String, usize, usize, usize) {
        (self.file, self.line, self.col, self.length)
    }

    /// Creates an error by cloning the location.
    pub(crate) fn to_failure(&self, msg: String) -> CompileError {
        CompileError::from((self.to_owned(), msg, ErrorLevel::Failure))
    }

    /// Creates an suggestion by cloning the location.
    pub(crate) fn to_suggestion(&self, msg: String) -> CompileError {
        CompileError::from((self.to_owned(), msg, ErrorLevel::Suggestion))
    }

    /// Creates an warning by cloning the location.
    pub(crate) fn to_warning(&self, msg: String) -> CompileError {
        CompileError::from((self.to_owned(), msg, ErrorLevel::Warning))
    }
}

impl From<&str> for Location {
    #[inline]
    fn from(value: &str) -> Self {
        Self {
            file: value.to_owned(),
            line: 1,
            col: 1,
            length: 1,
        }
    }
}

impl From<String> for Location {
    #[inline]
    fn from(value: String) -> Self {
        Self {
            file: value,
            line: 1,
            col: 1,
            length: 1,
        }
    }
}
