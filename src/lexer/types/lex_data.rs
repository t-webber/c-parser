//! Module to define the [`LexingData`] type.

use super::api::{Token, TokenValue};
use super::symbols::Symbol;
use crate::Res;
use crate::errors::api::CompileError;

/// Scope of the next block to skip
///
/// This is useful after comments or crashes, as we want to continue lexing but
/// by ignoring everything in a specific piece of code.
#[derive(Debug, Default, PartialEq, Eq)]
enum EndScope {
    /// End file
    ///
    /// Found a fault: the compiler must crash straight away.
    EndFile,
    /// End line
    ///
    /// Found a comment or a fault that makes the lexer ignore the rest of the
    /// line
    EndLine,
    /// Nothing to end
    ///
    /// Lexing may continue
    #[default]
    None,
}

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
    end: EndScope,
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
        !matches!(self.end, EndScope::None)
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
        if matches!(self.end, EndScope::EndLine) {
            self.end = EndScope::None;
        }
    }

    /// Pushes an error to the lexing data.
    pub fn push_err(&mut self, err: CompileError) {
        let is_crash = err.is_crash();
        self.errors.push(err);
        if is_crash {
            self.end = EndScope::EndFile;
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
        if matches!(self.end, EndScope::None) {
            self.end = EndScope::EndLine;
        }
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
