use core::fmt;

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
    col: usize,
    file: String,
    line: usize,
}

impl Location {
    /// Increments column of location by 1
    ///
    /// This is used by lexer when parsing every character of the C file.
    pub(crate) fn incr_col(&mut self) {
        self.col += 1;
    }

    /// Increments line of location by 1
    ///
    /// This is used by lexer when parsing every line of the C file.
    pub(crate) fn incr_line(&mut self) {
        self.line += 1;
        self.col = 1;
    }

    /// Creates an error from a location without cloning
    pub(crate) fn into_error(self, msg: String) -> CompileError {
        CompileError::from((self, msg, ErrorLevel::Error))
    }

    /// Moves the location back a few character on the current line
    ///
    /// If the offset is too big, the column is set to minimal (1) without any
    /// warnings or errors.
    pub(crate) fn into_past(self, offset: usize) -> Self {
        Self {
            col: self.col.checked_sub(offset).unwrap_or(1),
            ..self
        }
    }

    /// Returns the owned data of a `Location`.
    pub(crate) fn into_values(self) -> (String, usize, usize) {
        (self.file, self.line, self.col)
    }

    /// Creates an error by cloning the location.
    pub(crate) fn to_error(&self, msg: String) -> CompileError {
        CompileError::from((self.to_owned(), msg, ErrorLevel::Error))
    }

    /// Returns a clone of the current file name.
    pub(crate) fn to_filename(&self) -> String {
        self.file.clone()
    }

    /// Creates an warning by cloning the location.
    pub(crate) fn to_suggestion(&self, msg: String) -> CompileError {
        CompileError::from((self.to_owned(), msg, ErrorLevel::Warning))
    }

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
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Location {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}
