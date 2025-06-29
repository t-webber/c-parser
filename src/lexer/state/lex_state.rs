//! Module that defines and implements the [`LexingState`] automaton.

use crate::LocationPointer;
use crate::lexer::state::api::SymbolState;
use crate::lexer::types::api::Ident;

/// State of the comments
///
/// This state only focuses on blocks comments: `/* ... */`.
///
/// Inline comments, starting with `//` are handled by skipping the end of the
/// line. See [`LexingData`](crate::lexer::types::api::LexingData) for more
/// information.
#[derive(Debug, PartialEq, Eq)]
pub enum CommentState {
    /// Outside of comments
    False,
    /// Reading a possible change of comment status: `*/` contain two character,
    /// so, when the first is read, the state is marked as
    /// [`CommentState::Star`].
    Star,
    /// Inside comments
    True,
}

/// Stores the current state of the lexer
#[derive(Debug, Default)]
pub enum LexingState {
    /// Reading a char
    ///
    /// - When `'` is read, the state becomes `Char(None)`.
    /// - The next character is stored inside `Char(_)`.
    Char(Option<char>),
    /// Reading a block comment.
    Comment(CommentState),
    /// Reading an identifier.
    Ident(Ident),
    /// No specific state: just started parsing.
    #[default]
    StartOfLine,
    /// Reading a string literal, between double quotes.
    Str((String, LocationPointer)),
    /// Reading symbols.
    Symbols(SymbolState),
    /// Default variant for when all the buffers are cleared.
    ///
    /// The state is unset after whitespaces that aren't at the beginning of the
    /// line.
    Unset,
}

impl LexingState {
    /// See [`SymbolState::clear_last`].
    pub fn clear_last_symbol(&mut self) {
        if let Self::Symbols(symbol) = self {
            symbol.clear_last();
        } else {
            unreachable!("Didn't check if allowed before calling on symbol")
        }
    }

    /// Creates an identifier from a char.
    pub fn new_ident(&mut self, ch: char) {
        *self = Self::Ident(Ident::from(ch.to_string()));
    }

    /// Creates an identifier from a string.
    pub fn new_ident_str(&mut self, str: String) {
        *self = Self::Ident(Ident::from(str));
    }

    /// Tries to return the symbol state, if the current state is in symbol
    /// state.
    pub fn symbol_and_last_is(&self, ch: char) -> bool {
        if let Self::Symbols(symbol) = self {
            symbol.last() == ch
        } else {
            false
        }
    }
}
