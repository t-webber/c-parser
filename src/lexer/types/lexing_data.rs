use core::mem;

use super::tokens_types::{Symbol, Token, TokenValue};
use crate::errors::api::CompileError;

#[derive(Debug, Default)]
pub struct LexingData {
    end_line: bool,
    errors: Vec<CompileError>,
    failed: bool,
    tokens: Vec<Token>,
}

impl LexingData {
    pub const fn is_end_line(&self) -> bool {
        self.failed || self.end_line
    }

    pub fn last_is_minus(&self) -> bool {
        self.tokens.last().map_or_else(
            || false,
            |tok| *tok.get_value() == TokenValue::Symbol(Symbol::Minus),
        )
    }

    pub const fn newline(&mut self) {
        self.failed = false;
        self.end_line = false;
    }

    pub fn push_err(&mut self, err: CompileError) {
        let is_error = err.is_error();
        self.errors.push(err);
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

    pub fn take_errors(&mut self) -> Vec<CompileError> {
        mem::take(&mut self.errors)
    }

    pub fn take_tokens(&mut self) -> Vec<Token> {
        mem::take(&mut self.tokens)
    }
}

#[must_use]
#[inline]
pub fn display_tokens(tokens: &[Token]) -> String {
    format!(
        "[{}]",
        tokens
            .iter()
            .map(|x| format!("{x}"))
            .collect::<Vec<_>>()
            .join(", ")
    )
}
