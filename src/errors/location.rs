//! Module to store the location and length of the error.
//!
//! This crate implements the [`Location`] struct and its methods.

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
    /// Returns the referenced data of a `Location`.
    pub(super) fn get_values(&self) -> (&str, usize, usize, usize) {
        (self.file.as_ref(), self.line, self.col, self.length)
    }

    /// Increments column of location by 1
    ///
    /// This is used by lexer when parsing every character of the C file.
    #[coverage(off)]
    pub(crate) fn incr_col<F: FnMut(CompileError)>(&mut self, store: &mut F) {
        match self.col.checked_add(1) {
            Some(col) => self.col = col,
            None => store(self.to_warning(format!(
                "This line of code exceeds the maximum numbers of columns ({}). Consider refactoring your code.",
                usize::MAX
            )))
        }
    }

    /// Increments line of location by 1
    ///
    /// This is used by lexer when parsing every line of the C file.
    #[coverage(off)]
    pub(crate) fn incr_line<F: FnMut(CompileError)>(&mut self, store: &mut F) {
        self.col = 0;
        match self.line.checked_add(1) {
            Some(line) => self.line = line,
            None => store(self.to_warning(format!(
                "This line of code exceeds the maximum numbers of lines ({}). Consider refactoring your code.",
                usize::MAX
            )))
        }
    }

    /// Creates an error from a location without cloning
    pub(crate) fn into_failure(self, msg: String) -> CompileError {
        CompileError::from((self, msg, ErrorLevel::Failure))
    }

    /// Creates an error by cloning the location.
    pub(crate) fn to_failure(&self, msg: String) -> CompileError {
        CompileError::from((self.to_owned(), msg, ErrorLevel::Failure))
    }

    /// Moves the location back a few character on the current line
    ///
    /// If the offset is too big, the column is set to minimal (1) without any
    /// warnings or errors.
    pub(crate) fn to_past(&self, len: usize, offset: usize) -> Self {
        Self {
            col: self.col.checked_sub(offset).expect("never happens"),
            length: len,
            ..self.to_owned()
        }
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
        Self::from(value.to_owned())
    }
}

impl From<String> for Location {
    #[inline]
    fn from(value: String) -> Self {
        Self {
            file: value,
            line: 0,
            col: 0,
            length: 1,
        }
    }
}
