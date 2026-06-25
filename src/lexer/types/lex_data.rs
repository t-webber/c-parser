//! Module to define the [`LexingData`] type.

use core::mem::replace;
use std::collections::HashMap;

use super::api::{Token, TokenValue};
use super::symbols::Symbol;
use crate::Res;
use crate::errors::api::CompileError;
use crate::utils::{StringResolver, display};

/// Holds an index instead of a full string.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringId(u32);

display!(StringId, self, f, self.0.fmt(f));

impl StringId {
    /// Increment and return the old value.
    const fn post_inc(&mut self) -> Self {
        replace(self, Self(self.0.saturating_add(1)))
    }
}

/// Lexing data
///
/// Contains the data needed will lexing. It contains buffers and information
/// needed to be stored.
#[derive(Debug)]
pub struct LexingData {
    /// Boolean to indicate if the lexer needs to fail this line and try the
    /// next.
    ///
    /// It is used when reading `//` to skip the rest of the line.
    end_line: bool,
    /// Errors that have occurred while lexing.
    errors: Vec<CompileError>,
    /// Next id to use to store the strings.
    string_id: StringId,
    /// Store strings elsewhere to make lexing and parsing lighter on memory.
    strings: HashMap<String, StringId>,
    /// Tokens that have been lexed
    tokens: Vec<Token>,
}

impl LexingData {
    /// Concatenate 2 strings, useful to create the `"abc" "def"` concatenation.
    // PERF: this is underperformant but HashMap was chosen to be kept as insertions
    // happen much more often than concatenations.
    pub fn concat_strs(&mut self, id0: StringId, id1: StringId) -> StringId {
        let mut found = None::<&str>;
        #[expect(clippy::iter_over_hash_type, reason = "order not meaningful")]
        for (str, id) in &self.strings {
            if *id == id0 {
                if let Some(prev) = found {
                    return self.push_str(str.to_owned() + prev);
                }
                found = Some(str);
            } else if *id == id1 {
                if let Some(prev) = found {
                    return self.push_str(prev.to_owned() + str);
                }
                found = Some(str);
            }
        }
        unreachable!("ids can't exist without the value being in the map")
    }

    /// Makes a [`Res`] from the lexing data.
    pub fn into_res(self) -> Res<StringResolver<Vec<Token>>> {
        Res::from((
            StringResolver::from((self.tokens, self.strings.into_iter().collect())),
            self.errors,
        ))
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
    /// This is useful to know whether the last accepted token was `-`. This is
    /// used when trying to con
    pub fn last_is_minus(&self) -> bool {
        self.tokens.last().map_or_else(
            || false,
            |tok| matches!(tok.as_value(), &TokenValue::Symbol(Symbol::Minus)),
        )
    }

    /// Returns a new [`LexingData`] structure.
    pub fn new() -> Self {
        Self {
            end_line: false,
            errors: vec![],
            tokens: vec![],
            strings: HashMap::new(),
            string_id: StringId(1),
        }
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

    /// Stores a string and returns the corresponding id.
    pub fn push_str(&mut self, str: String) -> StringId {
        let string_id = &mut self.string_id;
        *self
            .strings
            .entry(str)
            .or_insert_with(|| string_id.post_inc())
    }

    /// Pushes a token to the lexing data.
    ///
    /// # Note
    ///
    /// If two successive constant strings are found, they are merged.
    pub fn push_token(&mut self, next: Token) {
        if let Some(prev) = self.tokens.last()
            && let (TokenValue::Str(prev_id), start) = prev.as_value_location()
            && let (TokenValue::Str(next_id), end) = next.as_value_location()
        {
            let id = self.concat_strs(*prev_id, *next_id);
            self.tokens
                .push(Token::from((TokenValue::Str(id), start.into_extended(end))));
        } else {
            self.tokens.push(next);
        }
    }

    /// Sets the lexing data in end-of-line
    pub const fn set_end_line(&mut self) {
        self.end_line = true;
    }
}

impl StringResolver<Vec<Token>> {
    /// Function to display tokens in a user-readable format.
    ///
    /// # Examples
    ///
    /// ```
    /// use c_parser::*;
    ///
    /// let tokens = lex("int x = 3", 0)
    ///     .unwrap_or_display(&[])
    ///     .unwrap()
    ///     .display();
    /// assert!(&displayed == "[int, $x, Assign, 3]", "!{displayed}!");
    /// ```
    #[must_use]
    pub fn display(&self) -> String {
        format!(
            "[{}]",
            self.as_value()
                .iter()
                .map(|token| self.display_token(token))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    /// Displays one token, converting the value with its meaning if necessary.
    fn display_token(&self, token: &Token) -> String {
        match token.as_value() {
            TokenValue::Char(ch) => format!("'{ch}'"),
            TokenValue::Ident(id) => format!("${}", self.resolve(*id)),
            TokenValue::Keyword(keyword) => format!("{keyword}"),
            TokenValue::Number(number) => format!("{number}"),
            TokenValue::Str(id) => format!("\"{}\"", self.resolve(*id)),
            TokenValue::Symbol(symbol) => format!("{symbol:?}"),
        }
    }
}
