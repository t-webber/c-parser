use super::tokens_types::Token;
use crate::errors::compile::CompileError;
use std::mem;

#[derive(Debug, Default)]
pub struct LexingData {
    errors: Vec<CompileError>,
    tokens: Vec<Token>,
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

    pub fn push_err(&mut self, error: CompileError) -> Result<(), ()> {
        let is_error = error.is_error();
        self.errors.push(error);
        if is_error {
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn push_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}
