use super::tokens_types::Token;
use crate::errors::compile::CompileError;
use core::mem;

#[derive(Debug, Default)]
pub struct LexingData {
    errors: Vec<CompileError>,
    tokens: Vec<Token>,
    failed: bool,
    end_line: bool,
}

impl LexingData {
    pub fn pop_token(&mut self) -> Option<Token> {
        self.tokens.pop()
    }

    pub fn take_errors(&mut self) -> Vec<CompileError> {
        mem::take(&mut self.errors)
    }

    pub fn take_tokens(&mut self) -> Vec<Token> {
        mem::take(&mut self.tokens)
    }

    pub fn push_err(&mut self, error: CompileError) {
        let is_error = error.is_error();
        self.errors.push(error);
        if is_error {
            self.failed = true;
        }
    }

    pub fn push_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub const fn set_end_line(&mut self) {
        self.end_line = true;
    }

    pub const fn newline(&mut self) {
        self.failed = false;
        self.end_line = false;
    }

    pub const fn is_end_line(&self) -> bool {
        self.failed || self.end_line
    }
}
