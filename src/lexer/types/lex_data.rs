//! Module to define the [`LexingData`] type.

use super::super::types::api::{Token, TokenValue};
use super::symbols::Symbol;
use crate::Res;
use crate::errors::api::CompileError;

/// Lexing data
///
/// Contains the data needed will lexing. It contains buffers and information
/// needed to be stored.
#[derive(Debug, Default)]
pub struct LexingData {
    /// Boolean to indicate if the lexer needs to fail this line and try the
    /// next.
    ///
    /// This can be useful when adding an error. After an error, the state is
    /// wrong so we can't continue to lex like nothing happened, so we need to
    /// start a new line.
    ///
    /// It is also used when reading `//` to skip the rest of the line.
    end_line: bool,
    /// Errors that have occurred while lexing.
    errors: Vec<CompileError>,
    /// Tokens that have been lexed
    tokens: Vec<Token>,
}

impl LexingData {
    /// Makes a [`Res`] from the lexing data.
    pub fn into_res(self) -> Res<Vec<Token>> {
        Res::from((self.tokens, self.errors))
    }

    /// Checks if the lexer must terminate or note.
    ///
    /// # Returns
    ///
    /// This method returns `self.end_line`, which means that an error just
    /// occurred.
    pub const fn is_end_line(&self) -> bool {
        self.end_line
    }

    /// Checks if the last parsed token was a minus sign.
    ///
    /// This is useful to know wether the last accepted token was `-`. This is
    /// used when trying to con
    pub fn last_is_minus(&self) -> bool {
        self.tokens.last().map_or_else(
            || false,
            |tok| *tok.get_value() == TokenValue::Symbol(Symbol::Minus),
        )
    }

    /// Resets the lexing data for a new line.
    pub const fn newline(&mut self) {
        self.end_line = false;
    }

    /// Pushes an error to the lexing data.
    pub fn push_err(&mut self, err: CompileError) {
        let is_error = err.is_failure();
        self.errors.push(err);
        if is_error {
            self.end_line = true;
        }
    }

    /// Pushes a token to the lexing data.
    pub fn push_token(&mut self, token: Token) {
        if let TokenValue::Str(val) = token.get_value()
            && let Some(TokenValue::Str(old)) = self.tokens.last_mut().map(Token::get_value_mut)
        {
            old.push_str(val);
        } else {
            self.tokens.push(token);
        }
    }

    /// Sets the lexing data in end-of-line
    pub const fn set_end_line(&mut self) {
        self.end_line = true;
    }
}

/// Function to display tokens in a user-readable format.
///
/// # Examples
///
/// ```
/// use c_parser::*;
///
/// let tokens = lex_file("int x = 3", &mut Location::from("")).unwrap_or_display(&[], "");
/// let displayed = display_tokens(&tokens);
/// assert!(
///     &displayed == "[Keyword(int), Ident(x), Assign, 3]",
///     "!{displayed}!"
/// );
/// ```
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
