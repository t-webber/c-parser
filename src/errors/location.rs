//! Module to store the location and length of the error.
//!
//! This crate implements the [`ErrorLocation`] struct and its methods.

use core::fmt;
use core::mem::take;

use super::compile::{CompileError, ErrorLevel};
use crate::utils::usize_to_u32;

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
#[derive(Clone, Default, Copy)]
pub enum ErrorLocation {
    /// Location a block of the source file
    ///
    /// # Fields
    ///
    /// file name, start line, start column, end line, end column
    Block(u32, u32, u32, u32, u32),
    /// Location on one char of the source file
    ///
    /// # Fields
    ///
    /// file name, line, column
    Char(u32, u32, u32),
    /// Never built, useful for taking
    #[default]
    None,
    /// Location a token of the source file
    ///
    /// # Fields
    ///
    /// file name, line, column, length
    Token(u32, u32, u32, u32),
}

impl ErrorLocation {
    /// Returns the filename of the current [`ErrorLocation`]
    fn as_filename(self) -> u32 {
        match self {
            Self::None => unreachable!("never built"),
            Self::Block(filename, _, _, _, _)
            | Self::Char(filename, _, _)
            | Self::Token(filename, _, _, _) => filename,
        }
    }

    /// Returns the start and end of the [`ErrorLocation`]
    #[expect(clippy::arithmetic_side_effects, reason = "in range of tokens")]
    fn as_pos(self) -> (u32, u32, u32, u32) {
        match self {
            Self::Block(_, line_s, col_s, line_e, col_e) => (line_s, col_s, line_e, col_e),
            Self::Char(_, line, col) => (line, col, line, col),
            Self::Token(_, line, col, len) => (line, col, line, col + len),
            Self::None => unreachable!("never built"),
        }
    }

    /// Extends a current [`ErrorLocation`] by changing the end of the location.
    pub fn extend(&mut self, other: Self) {
        *self = take(self).into_extended(other);
    }

    /// Returns a [`ErrorLocation`] that is the combination of the span covered
    /// by the 2 inputs.
    ///
    /// # Panics
    ///
    /// If the second provided [`ErrorLocation`] is before or overlaps the
    /// first.
    pub fn into_extended(self, other: Self) -> Self {
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
        if min.0 == max.2 {
            Self::Token(file, min.0, min.1, max.3.saturating_sub(min.1))
        } else {
            Self::Block(file, min.0, min.1, max.2, max.3)
        }
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

impl ErrorLocation {
    /// Creates a [`CompileError`] of level [`ErrorLevel::Crash`].
    pub fn crash(self, msg: String) -> CompileError {
        CompileError::from((self, msg, ErrorLevel::Crash))
    }

    /// Creates a [`CompileError`] of level [`ErrorLevel::Fault`].
    pub fn fail(self, msg: String) -> CompileError {
        CompileError::from((self, msg, ErrorLevel::Fault))
    }

    /// Creates a [`CompileError`] of level [`ErrorLevel::Suggestion`].
    pub fn suggest(self, msg: String) -> CompileError {
        CompileError::from((self, msg, ErrorLevel::Suggestion))
    }

    /// Creates a [`CompileError`] of level [`ErrorLevel::Warning`].
    pub fn warn(self, msg: String) -> CompileError {
        CompileError::from((self, msg, ErrorLevel::Warning))
    }
}

/// Structure used to lex the items and move the pointer forward.
#[derive(Debug, Clone, Copy)]
pub struct LocationPointer {
    /// Abscissa of the location
    col: u32,
    /// File of the location
    file: u32,
    /// Ordinate of the location
    line: u32,
}

impl LocationPointer {
    /// Increments column of location by 1
    ///
    /// This is used by lexer when parsing every character of the C file.
    pub(crate) fn incr_col<F: FnOnce(CompileError)>(&mut self, store: F) {
        match self.col.checked_add(1) {
            Some(col) => self.col = col,
            None => store(
                self.warn(format!("More than ({}) chars in this line, please refactor.", u32::MAX)),
            ),
        }
    }

    /// Increments line of location by 1
    ///
    /// This is used by lexer when parsing every line of the C file.
    pub(crate) fn incr_line<F: FnOnce(CompileError)>(&mut self, store: F) {
        self.col = 0;
        match self.line.checked_add(1) {
            Some(line) => self.line = line,
            None => store(
                self.warn(format!("More than ({}) lines in this file, please refactor.", u32::MAX)),
            ),
        }
    }

    /// Converts the [`LocationPointer`] to an [`ErrorLocation::Block`]
    pub(crate) const fn into_block(self, other: &Self) -> ErrorLocation {
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

    /// Creates a new file with the given file index.
    pub(crate) const fn start_file(file: u32) -> Self {
        Self { col: 0, file, line: 0 }
    }

    /// Moves the location back a few character on the current line
    ///
    /// If the offset is too big, the column is set to minimal (1) without any
    /// warnings or errors.
    pub(crate) fn to_past(self, len: usize, offset: usize) -> ErrorLocation {
        ErrorLocation::Token(
            self.file,
            self.line,
            self.col
                .checked_sub(usize_to_u32(offset))
                .expect("never happens"),
            usize_to_u32(len),
        )
    }
}

impl LocationPointer {
    /// Creates a [`CompileError`] of level [`ErrorLevel::Fault`].
    pub fn fail(self, msg: String) -> CompileError {
        CompileError::from((ErrorLocation::from(self), msg, ErrorLevel::Fault))
    }

    /// Creates a [`CompileError`] of level [`ErrorLevel::Suggestion`].
    pub fn suggest(self, msg: String) -> CompileError {
        CompileError::from((ErrorLocation::from(self), msg, ErrorLevel::Suggestion))
    }

    /// Creates a [`CompileError`] of level [`ErrorLevel::Warning`].
    pub fn warn(self, msg: String) -> CompileError {
        CompileError::from((ErrorLocation::from(self), msg, ErrorLevel::Warning))
    }
}

/// Adds an error location to a value.
#[derive(Default, Clone)]
pub struct Located<T>(T, ErrorLocation);

impl<T> Located<T> {
    /// References the location.
    pub const fn as_location(&self) -> ErrorLocation {
        self.1
    }

    /// Transfers the mutable reference to the value.
    pub const fn as_ref(&self) -> Located<&T> {
        Located(&self.0, self.1)
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

#[cfg(test)]
mod test {
    use crate::errors::api::LocationPointer;

    #[test]
    fn too_many_cols() {
        let mut location = LocationPointer { col: u32::MAX, file: 0, line: 0 };
        let mut has_err = false;
        location.incr_col(|err| {
            let got = err.as_values().1;
            has_err =
                got == format!("More than ({}) chars in this line, please refactor.", u32::MAX);
        });
        assert!(has_err);
    }

    #[test]
    fn too_many_lines() {
        let mut location = LocationPointer { col: 0, file: 0, line: u32::MAX };
        let mut has_err = false;
        location.incr_line(|err| {
            let got = err.as_values().1;
            has_err =
                got == format!("More than ({}) lines in this file, please refactor.", u32::MAX);
        });
        assert!(has_err);
    }
}
