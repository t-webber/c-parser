//! Module to store the location and length of the error.
//!
//! This crate implements the [`ErrorLocation`] struct and its methods.

use core::fmt;
use core::mem::take;

use super::compile::{CompileError, ErrorLevel};
use crate::errors::api::Located;

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

    /// Creates a new [`ErrorLocation`] of type char at the given position
    pub const fn new_char(file: u32, line: u32, col: u32) -> Self {
        Self::Char(file, line, col)
    }

    /// Adds a value to the error location to make a [`Located`].
    pub fn wrap<T>(self, value: T) -> Located<T> {
        Located::from((value, self))
    }
}

impl fmt::Debug for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "".fmt(f)
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
