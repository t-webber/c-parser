//! Module to follow the opening and closing blocks status.

use crate::Location;
use crate::errors::api::CompileError;

/// Type to save the closed blocks.
#[derive(Debug)]
pub struct BlockState {
    /// Type of the block
    pub block_type: BlockType,
    /// Location of the block
    pub location: Location,
}

impl BlockState {
    /// Creates a mismatched error for a mismatched closing block.
    ///
    /// This is called when more opening characters were found than closing
    /// ones.
    pub fn mismatched_err_begin(self) -> CompileError {
        let (open, close) = self.block_type.get_delimiters();
        self.location.into_failure(format!(
            "Mismatched '{close}'. Perhaps you forgot an opening '{open}'?"
        ))
    }
}

/// Enum of the different block delimiters.
#[derive(PartialEq, Eq, Debug)]
pub enum BlockType {
    /// `{` & `}`
    Brace,
    /// `[` & `]`
    Bracket,
    /// `(` & `)`
    Parenthesis,
}

impl BlockType {
    /// Returns the characters that correspond.
    const fn get_delimiters(&self) -> (char, char) {
        match self {
            Self::Brace => ('{', '}'),
            Self::Bracket => ('[', ']'),
            Self::Parenthesis => ('(', ')'),
        }
    }

    /// Creates a mismatched error for a mismatched opening block.
    ///
    /// This is called when more closing characters were found than opening
    /// ones.
    pub fn mismatched_err_end(&self, location: Location) -> CompileError {
        let (open, close) = self.get_delimiters();
        location.into_failure(format!(
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
    closed_blocks: Vec<BlockState>,
}

impl ParsingState {
    /// Contains opening blocks that weren't closed
    pub const fn has_opening_blocks(&self) -> bool {
        !self.closed_blocks.is_empty()
    }

    /// Returns errors for the unopened blocks (cf. [`BlockState`]).
    pub fn mismatched_error(&mut self) -> Vec<CompileError> {
        let mut errors = vec![];
        while let Some(block) = self.closed_blocks.pop() {
            errors.push(block.mismatched_err_begin());
        }
        errors
    }

    /// Pops the last opened block and compares it to a block type.
    pub fn pop_and_compare_block(&mut self, block_type: &BlockType) -> bool {
        let res = self
            .closed_blocks
            .pop()
            .map(|block| block.block_type)
            .as_ref()
            == Some(block_type);
        res
    }

    /// Pushes a block.
    pub fn push_closing_block(&mut self, block_type: BlockType, location: Location) {
        self.closed_blocks.push(BlockState {
            block_type,
            location,
        });
    }
}
