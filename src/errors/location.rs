//! Module to store the location and length of the error.
//!
//! This crate implements the [`ErrorLocation`] struct and its methods.

use core::fmt;
use core::mem::take;

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
#[derive(Clone, Default)]
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

impl ErrorLocation {
    /// Returns the filename of the current [`ErrorLocation`]
    fn as_filename(&self) -> &str {
        match self {
            Self::None => unreachable!("never built"),
            Self::Block(filename, _, _, _, _)
            | Self::Char(filename, _, _)
            | Self::Token(filename, _, _, _) => filename,
        }
    }

    /// Returns the start and end of the [`ErrorLocation`]
    #[expect(clippy::arithmetic_side_effects, reason = "in range of tokens")]
    fn as_pos(&self) -> (usize, usize, usize, usize) {
        match self {
            Self::Block(_, line_s, col_s, line_e, col_e) => (*line_s, *col_s, *line_e, *col_e),
            Self::Char(_, line, col) => (*line, *col, *line, *col),
            Self::Token(_, line, col, len) => (*line, *col, *line, col + len),
            Self::None => unreachable!("never built"),
        }
    }

    /// Extends a current [`ErrorLocation`] by changing the end of the location.
    pub fn extend(&mut self, other: &Self) {
        *self = take(self).into_extended(other);
    }

    /// Returns a [`ErrorLocation`] that is the combination of the span covered
    /// by the 2 inputs.
    ///
    /// # Panics
    ///
    /// If the second provided [`ErrorLocation`] is before or overlaps the
    /// first.
    pub fn into_extended(self, other: &Self) -> Self {
        debug_assert_eq!(
            self.as_filename(),
            other.as_filename(),
            "can't merge 2 locations from different files"
        );
        let first = self.as_pos();
        let second = other.as_pos();
        let (min, max) = if first.0 < second.0 || (first.0 == second.0 && first.1 <= second.1) {
            (first, second)
        } else {
            (second, first)
        };
        let file = match self {
            Self::None => unreachable!("never built"),
            Self::Block(file, _, _, _, _) | Self::Char(file, _, _) | Self::Token(file, _, _, _) =>
                file,
        };
        Self::Block(file, min.0, min.1, max.2, max.3)
    }

    /// Adds a value to the error location to make a [`Located`].
    pub const fn wrap<T>(self, value: T) -> Located<T> {
        Located(value, self)
    }
}

impl fmt::Debug for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "".fmt(f)
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
        if self.line == other.line {
            ErrorLocation::Token(
                self.file,
                self.line,
                self.col,
                other.col.saturating_sub(self.col).saturating_add(1),
            )
        } else {
            ErrorLocation::Block(self.file, self.line, self.col, other.line, other.col)
        }
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

impl From<&str> for LocationPointer {
    fn from(file: &str) -> Self {
        Self { col: 0, file: file.to_owned(), line: 0 }
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

#[cfg(feature = "debug")]
impl core::fmt::Display for LocationPointer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}:{:02}:{:02}",
            self.file
                .rsplit('/')
                .next()
                .expect("split never returns empty"),
            self.line,
            self.col
        )
    }
}

/// Adds an error location to a value.
#[derive(Default, Clone)]
pub struct Located<T>(T, ErrorLocation);

impl<T> Located<T> {
    /// Applies a function to value and location.
    pub fn and<U, F: FnOnce(T, ErrorLocation) -> U>(self, apply: F) -> U {
        apply(self.0, self.1)
    }

    /// References the location.
    pub const fn as_location(&self) -> &ErrorLocation {
        &self.1
    }

    /// Transfers the mutable reference to the value.
    pub fn as_mut(&mut self) -> Located<&mut T> {
        Located(&mut self.0, self.1.clone())
    }

    /// Transfers the mutable reference to the value.
    pub fn as_ref(&self) -> Located<&T> {
        Located(&self.0, self.1.clone())
    }

    /// References the value.
    pub const fn as_value(&self) -> &T {
        &self.0
    }

    /// Drops the location and returns the value.
    pub fn drop_location(self) -> T {
        self.0
    }

    /// Returns inner value and location.
    pub fn into_inner(self) -> (T, ErrorLocation) {
        (self.0, self.1)
    }

    /// Applies a function to the value but keeping the same location.
    pub fn transfer<U, F: FnOnce(T) -> U>(self, apply: F) -> Located<U> {
        Located(apply(self.0), self.1)
    }
}

impl<T: fmt::Display> fmt::Display for Located<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Debug> fmt::Debug for Located<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
