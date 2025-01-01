//! Module to follow the opening and closing blocks status.

use crate::Location;
use crate::errors::api::CompileError;

/// Type used to try match opening and closing braces, brackets and parenthesis.
#[derive(PartialEq, Eq, Debug)]
pub enum BlockState {
    /// `{` & `}`
    Brace,
    /// `[` & `]`
    Bracket,
    /// `(` & `)`
    Parenthesis,
}

impl BlockState {
    /// Returns the characters that correspond.
    const fn get_delimiters(&self) -> (char, char) {
        match self {
            Self::Brace => ('{', '}'),
            Self::Bracket => ('[', ']'),
            Self::Parenthesis => ('(', ')'),
        }
    }

    /// Creates a mismatched error for a mismatched closing block.
    ///
    /// This is called when more opening characters were found than closing
    /// ones.
    pub fn mismatched_err_begin(&self, location: Location) -> CompileError {
        let (open, close) = self.get_delimiters();
        location.into_error(format!(
            "Mismatched '{close}'. Perhaps you forgot an opening '{open}'?"
        ))
    }

    /// Creates a mismatched error for a mismatched opening block.
    ///
    /// This is called when more closing characters were found than opening
    /// ones.
    pub fn mismatched_err_end(&self, location: Location) -> CompileError {
        let (open, close) = self.get_delimiters();
        location.into_error(format!(
            "Mismatched '{open}': reached end of block. Perhaps you forgot a closing '{close}'?"
        ))
    }
}

/// Stores data for the parsing state.
#[derive(Default, Debug)]
pub struct ParsingState {
    /// History of the opened and unclosed blocks.
    ///
    /// This is pushed and popped on recursion calls to check that the block
    /// ended with the right character.
    pub opened_blocks: Vec<BlockState>,
}
