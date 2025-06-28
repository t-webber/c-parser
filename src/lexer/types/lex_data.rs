//! Module to define the [`LexingData`] type.

use super::api::{Token, TokenValue};
use super::symbols::Symbol;
use crate::Res;
use crate::errors::api::{CompileError, ExtendErrorBlock as _};

/// Lexing data
///
/// Contains the data needed will lexing. It contains buffers and information
/// needed to be stored.
#[derive(Debug, Default)]
pub struct LexingData {
    /// Boolean to indicate if the lexer needs to fail this line and try the
    /// next.
    ///
    /// It is used when reading `//` to skip the rest of the line.
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
            |tok| matches!(tok.as_value(), &TokenValue::Symbol(Symbol::Minus)),
        )
    }

    /// Resets the lexing data for a new line.
    pub const fn newline(&mut self) {
        self.end_line = false;
    }

    /// Pushes an error to the lexing data.
    ///
    /// # Note
    ///
    /// This method doesn't crash the program, as the lexer errors are
    /// considered recoverable: the maximum level is fault not crash. If an
    /// error occurs, the lexer knows how to continue and parse the rest of the
    /// program.
    pub fn push_err(&mut self, err: CompileError) {
        self.errors.push(err);
    }

    /// Pushes a token to the lexing data.
    ///
    /// # Note
    ///
    /// If two successive constant strings are found, they are merged.
    pub fn push_token(&mut self, token: Token) {
        if let (TokenValue::Str(val), end_location) = token.as_value_location()
            && let Some(last) = self.tokens.last_mut()
            && let TokenValue::Str(last_str) = last.as_value_mut()
        {
            last_str.push_str(val);
            last.extend_location(end_location);
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
/// let tokens = lex_file("int x = 3", &mut LocationPointer::from("")).unwrap_or_display(&[], "");
/// let displayed = display_tokens(&tokens);
/// assert!(&displayed == "[Keyword(int), Ident(x), Assign, 3]", "!{displayed}!");
/// ```
#[must_use]
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
