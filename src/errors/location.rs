//! Module to store the location and length of the error.
//!
//! This crate implements the [`ErrorLocation`] struct and its methods.

use super::compile::{CompileError, ErrorLevel};

/// Type to pinpoint a precise character in the C source file.
///
/// The locations are computed by the lexer that reads the C source file. Then,
/// the locations are stored inside the tokens to keep them for the rest of the
/// compiler.
///
/// # Note
///
/// In order to respect the click links from terminals, the line and column of
/// a file start at 1 and not 0.
#[derive(Debug, Clone, Default)]
pub enum ErrorLocation {
    /// Location a block of the source file
    ///
    /// # Fields
    ///
    /// file name, start line, start column, end line, end column
    Block(String, usize, usize, usize, usize),
    /// Location on one char of the source file
    ///
    /// # Fields
    ///
    /// file name, line, column
    Char(String, usize, usize),
    /// Never built, useful for taking
    #[default]
    None,
    /// Location a token of the source file
    ///
    /// # Fields
    ///
    /// file name, line, column, length
    Token(String, usize, usize, usize),
}

impl ExtendErrorBlock for ErrorLocation {
    fn extend_location(&mut self, extender: &Self) {
        if let Self::Block(.., end_line, end_col) = self
            && let Self::Block(.., extend_end_line, extend_end_col) = extender
        {
            *end_line = *extend_end_line;
            *end_col = *extend_end_col;
        } else {
            unreachable!("called on non block")
        }
    }
}

impl From<LocationPointer> for ErrorLocation {
    fn from(value: LocationPointer) -> Self {
        Self::Char(value.file, value.line, value.col)
    }
}

impl IntoError for ErrorLocation {
    fn into_crash(self, msg: String) -> CompileError {
        CompileError::from((self, msg, ErrorLevel::Crash))
    }

    fn into_fault(self, msg: String) -> CompileError {
        CompileError::from((self, msg, ErrorLevel::Fault))
    }

    fn into_suggestion(self, msg: String) -> CompileError {
        CompileError::from((self, msg, ErrorLevel::Suggestion))
    }

    fn into_warning(self, msg: String) -> CompileError {
        CompileError::from((self, msg, ErrorLevel::Warning))
    }
}

/// Trait for the [`ExtendErrorBlock::extend_location`] method.
pub trait ExtendErrorBlock {
    /// Extends a current [`ErrorLocation`] by changing the end of the location.
    ///
    /// # Panics
    ///
    /// If called on a non-block [`ErrorLocation`] or if the extender is a
    /// non-block [`ErrorLocation`].
    fn extend_location(&mut self, extender: &ErrorLocation);
}

/// Trait the implements methods to convert a [`ErrorLocation`] into a
/// [`CompileError`]
pub trait IntoError: Clone {
    /// Creates a [`CompileError`] of level [`ErrorLevel::Crash`] without
    /// cloning
    fn into_crash(self, msg: String) -> CompileError;
    /// Creates a [`CompileError`] of level [`ErrorLevel::Fault`] without
    /// cloning
    fn into_fault(self, msg: String) -> CompileError;
    /// Creates a [`CompileError`] of level [`ErrorLevel::Suggestion`] without
    /// cloning
    fn into_suggestion(self, msg: String) -> CompileError;
    /// Creates a [`CompileError`] of level [`ErrorLevel::Warning`] without
    /// cloning
    fn into_warning(self, msg: String) -> CompileError;
    /// Creates a [`CompileError`] of level [`ErrorLevel::Crash`] by cloning the
    /// original
    fn to_crash(&self, msg: String) -> CompileError {
        self.to_owned().into_crash(msg)
    }
    /// Creates a [`CompileError`] of level [`ErrorLevel::Fault`] by cloning the
    /// original
    fn to_fault(&self, msg: String) -> CompileError {
        self.to_owned().into_fault(msg)
    }
    /// Creates a [`CompileError`] of level [`ErrorLevel::Suggestion`] by
    /// cloning the original
    fn to_suggestion(&self, msg: String) -> CompileError {
        self.to_owned().into_suggestion(msg)
    }
    /// Creates a [`CompileError`] of level [`ErrorLevel::Warning`] by cloning
    /// the original
    fn to_warning(&self, msg: String) -> CompileError {
        self.to_owned().into_warning(msg)
    }
}

/// Structure used to lex the items and move the pointer forward.
#[derive(Debug, Clone)]
pub struct LocationPointer {
    /// Abscissa of the location
    col: usize,
    /// File of the location
    file: String,
    /// Ordinate of the location
    line: usize,
}

impl LocationPointer {
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

    /// Converts the [`LocationPointer`] to an [`ErrorLocation::Block`]
    pub(crate) fn into_block(self, other: &Self) -> ErrorLocation {
        ErrorLocation::Block(self.file, self.line, self.line, other.line, other.col)
    }

    /// Converts the [`LocationPointer`] to an [`ErrorLocation`]
    pub(crate) fn to_error_location(&self) -> ErrorLocation {
        ErrorLocation::from(self.to_owned())
    }

    /// Moves the location back a few character on the current line
    ///
    /// If the offset is too big, the column is set to minimal (1) without any
    /// warnings or errors.
    pub(crate) fn to_past(&self, len: usize, offset: usize) -> ErrorLocation {
        ErrorLocation::Token(
            self.file.clone(),
            self.line,
            self.col.checked_sub(offset).expect("never happens"),
            len,
        )
    }
}

impl<T: ToString> From<T> for LocationPointer {
    fn from(file: T) -> Self {
        Self { col: 0, file: file.to_string(), line: 0 }
    }
}

impl IntoError for LocationPointer {
    fn into_crash(self, msg: String) -> CompileError {
        CompileError::from((ErrorLocation::from(self), msg, ErrorLevel::Crash))
    }

    fn into_fault(self, msg: String) -> CompileError {
        CompileError::from((ErrorLocation::from(self), msg, ErrorLevel::Fault))
    }

    fn into_suggestion(self, msg: String) -> CompileError {
        CompileError::from((ErrorLocation::from(self), msg, ErrorLevel::Suggestion))
    }

    fn into_warning(self, msg: String) -> CompileError {
        CompileError::from((ErrorLocation::from(self), msg, ErrorLevel::Warning))
    }
}
