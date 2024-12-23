use core::mem;

use super::tokens_types::{Symbol, Token, TokenValue};
use crate::errors::compile::CompileError;

#[derive(Debug, Default)]
pub struct LexingData {
    errors: Vec<CompileError>,
    tokens: Vec<Token>,
    failed: bool,
    end_line: bool,
}

impl LexingData {
    pub fn last_is_minus(&self) -> bool {
        self.tokens.last().map_or_else(
            || false,
            |tok| *tok.get_value() == TokenValue::Symbol(Symbol::Minus),
        )
    }

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

    pub fn extend_err(&mut self, errors: Vec<CompileError>) {
        let is_error = errors.iter().any(CompileError::is_error);
        self.errors.extend(errors);
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
