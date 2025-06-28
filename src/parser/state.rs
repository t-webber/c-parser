//! Module to follow the opening and closing blocks status.

use crate::errors::api::{CompileError, ErrorLocation, IntoError as _};
use crate::utils::display;

/// Type to save the closed blocks.
#[derive(Debug)]
pub struct BlockState {
    /// Type of the block
    pub block_type: BlockType,
    /// Location of the block
    pub location: ErrorLocation,
}

impl BlockState {
    /// Creates a mismatched error for a mismatched closing block.
    ///
    /// This is called when more opening characters were found than closing
    /// ones.
    pub fn mismatched_err_begin(self) -> CompileError {
        let (open, close) = self.block_type.as_delimiters();
        self.location
            .into_crash(format!("Mismatched '{close}'. Perhaps you forgot an opening '{open}'?"))
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
    const fn as_delimiters(&self) -> (char, char) {
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
    pub fn mismatched_err_end(&self, location: ErrorLocation) -> CompileError {
        let (open, close) = self.as_delimiters();
        location.into_crash(format!(
            "Mismatched '{open}': reached end of block. Perhaps you forgot a closing '{close}'?"
        ))
    }
}

/// State to know who was the parent control flow of the current block
#[derive(Debug, Default, PartialEq, Eq)]
pub enum CtrlFlowState {
    /// No interesting information
    #[default]
    None,
    /// Inside the block following a `switch`, but at the top most level
    Switch,
}

/// Stores data for the parsing state.
#[derive(Default, Debug)]
pub struct ParsingState {
    /// History of the closed blocks.
    ///
    /// This is pushed (when the recursion is broken) and popped (on the
    /// recursive call) on recursion calls to check that the block ended
    /// with the right character.
    closed_blocks: Vec<BlockState>,
    /// History of the opened control flow.
    ///
    /// This is pushed (on the recursive call) and popped (when the recursion is
    /// broken) to know
    opened_ctrl_flows: Vec<CtrlFlowState>,
}

impl ParsingState {
    /// Contains opening blocks that weren't closed
    pub const fn has_opening_blocks(&self) -> bool {
        !self.closed_blocks.is_empty()
    }

    /// Checks whether we are the top most block of a switch
    pub fn is_in_switch(&self) -> bool {
        self.opened_ctrl_flows
            .last()
            .is_some_and(|x| x == &CtrlFlowState::Switch)
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
        self.closed_blocks
            .pop()
            .map(|block| block.block_type)
            .as_ref()
            == Some(block_type)
    }

    /// Pops a control flow.
    pub fn pop_ctrl_flow(&mut self) -> Option<bool> {
        self.opened_ctrl_flows
            .pop()
            .map(|x| x == CtrlFlowState::Switch)
    }

    /// Pushes a block.
    pub fn push_closing_block(&mut self, block_type: BlockType, location: ErrorLocation) {
        self.closed_blocks.push(BlockState { block_type, location });
    }

    /// Pushed a control flow.
    pub fn push_ctrl_flow(&mut self, switch: bool) {
        self.opened_ctrl_flows.push(if switch {
            CtrlFlowState::Switch
        } else {
            CtrlFlowState::None
        });
    }
}

display!(ParsingState, self, f, {
    write!(
        f,
        "blks = [{}] & ctrls = [{}]",
        self.closed_blocks
            .iter()
            .map(|blk| blk.block_type.as_delimiters().1.to_string())
            .collect::<Vec<_>>()
            .join(", "),
        self.opened_ctrl_flows
            .iter()
            .map(|ctrl| format!("{ctrl:?}"))
            .collect::<Vec<_>>()
            .join(", ")
    )
});
