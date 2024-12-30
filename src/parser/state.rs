use crate::Location;
use crate::errors::api::CompileError;

#[derive(PartialEq, Eq, Debug)]
pub enum BlockState {
    Brace,
    Bracket,
    Parenthesis,
}

impl BlockState {
    const fn get_delimiters(&self) -> (char, char) {
        match self {
            Self::Brace => ('{', '}'),
            Self::Bracket => ('[', ']'),
            Self::Parenthesis => ('(', ')'),
        }
    }

    pub fn mismatched_err_begin(&self, location: Location) -> CompileError {
        let (open, close) = self.get_delimiters();
        location.into_error(format!(
            "Mismatched '{close}'. Perhaps you forgot an opening '{open}'?"
        ))
    }

    pub fn mismatched_err_end(&self, location: Location) -> CompileError {
        let (open, close) = self.get_delimiters();
        location.into_error(format!(
            "Mismatched '{open}': reached end of block. Perhaps you forgot a closing '{close}'?"
        ))
    }
}

#[derive(Default, Debug)]
pub struct ParsingState {
    pub opened_blocks: Vec<BlockState>,
}
