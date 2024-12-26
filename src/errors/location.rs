use core::fmt;

use super::compile::{CompileError, ErrorLevel};

#[derive(Debug, Clone)]
pub struct Location {
    col: usize,
    file: String,
    line: usize,
}

impl Location {
    pub(crate) fn get(self) -> (String, usize, usize) {
        (self.file, self.line, self.col)
    }

    pub(crate) fn incr_col(&mut self) {
        self.col += 1;
    }

    pub(crate) fn incr_line(&mut self) {
        self.line += 1;
    }

    pub(crate) fn into_error(self, msg: String) -> CompileError {
        CompileError::from((self, msg, ErrorLevel::Error))
    }

    pub(crate) fn into_past(self, offset: usize) -> Self {
        Self {
            col: self.col.checked_sub(offset).unwrap_or(1),
            ..self
        }
    }

    pub(crate) fn new_line(&mut self) {
        self.line += 1;
        self.col = 1;
    }

    pub(crate) fn to_error(&self, msg: String) -> CompileError {
        CompileError::from((self.to_owned(), msg, ErrorLevel::Error))
    }

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
