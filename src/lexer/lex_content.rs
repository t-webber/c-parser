//! Module that implements the functions that lex raw strings.
//!
//! See [`lex_file`] for more information.

use super::state::api::{
    CommentState, EscapeState, LexingState as LS, SymbolState, end_current, handle_escape
};
use super::types::api::{LexingData, Token};
use crate::errors::api::{Location, Res};

/// Function to manage one character.
///
/// This function updates the [`LS`] automaton, and executes the right
/// handlers.
#[expect(clippy::too_many_lines)]
fn lex_char(
    ch: char,
    location: &Location,
    lex_data: &mut LexingData,
    lex_state: &mut LS,
    escape_state: &mut EscapeState,
    eol: bool,
) {
    match (ch, lex_state, escape_state) {
        (_, LS::StartOfLine, _) if ch.is_whitespace() => (),
        /* Inside comment */
        ('/', state @ LS::Comment(CommentState::Star), _) => {
            *state = LS::Comment(CommentState::False);
        }
        ('*', state @ LS::Comment(CommentState::True), _) => {
            *state = LS::Comment(CommentState::Star);
        }
        (_, LS::Comment(CommentState::True), _) => (),
        (_, state @ LS::Comment(CommentState::Star), _) => {
            *state = LS::Comment(CommentState::True);
        }
        /* Escaped character */
        (
            _,
            state @ (LS::Char(None) | LS::Str(_)),
            escape @ (EscapeState::Single | EscapeState::Sequence(_)),
        ) => {
            if let Some(escaped) = handle_escape(ch, lex_data, escape, location) {
                *escape = EscapeState::False;
                #[expect(clippy::wildcard_enum_match_arm)]
                match state {
                    LS::Char(None) => *state = LS::Char(Some(escaped)),
                    LS::Str(val) => val.push(escaped),
                    _ => panic!("this can't happen, see match above"),
                }
            }
        }

        (_, _, EscapeState::Single | EscapeState::Sequence(_)) => {
            panic!("Can't happen because error raised on escape creation if wrong state.")
        }
        /* Create comment */
        ('*', state, _) if state.symbol().and_then(SymbolState::last) == Some('/') => {
            state.clear_last_symbol();
            end_current(state, lex_data, location);
            *state = LS::Comment(CommentState::True);
        }

        /* Escape character */
        ('\\', LS::Char(None) | LS::Str(_), escape) => *escape = EscapeState::Single,
        ('\\', _, escape) if eol => *escape = EscapeState::Single,
        ('\\', state, _) => lex_data.push_err(location.to_failure(format!(
            "Escape characters are only authorised in strings or chars, not in '{}' context.",
            state.repr(),
        ))),

        /* Static strings and chars */
        // open/close
        ('\'', LS::Symbols(symbol_state), _) if symbol_state.is_trigraph() => {
            if let Some((size, symbol)) = symbol_state.push(ch, lex_data, location) {
                lex_data.push_token(Token::from_symbol(symbol, size, location));
            }
        }
        ('\'', state @ LS::Char(_), _) => end_current(state, lex_data, location),
        ('\'', state, _) if !matches!(state, LS::Str(_)) => {
            end_current(state, lex_data, location);
            *state = LS::Char(None);
        }
        ('\"', state @ LS::Str(_), _) => {
            end_current(state, lex_data, location);
        }
        ('\"', state, _) if !matches!(state, LS::Char(_)) => {
            end_current(state, lex_data, location);
            *state = LS::Str(String::new());
        }
        // middle
        (_, LS::Char(Some(_)), _) => lex_data
            .push_err(location.to_failure("A char must contain only one character.".to_owned())),
        (_, state @ LS::Char(None), _) => *state = LS::Char(Some(ch)),
        (_, LS::Str(val), _) => val.push(ch),

        /* Operator symbols */
        ('/', state, _) if state.symbol().and_then(SymbolState::last) == Some('/') => {
            state.clear_last_symbol();
            end_current(state, lex_data, location);
            lex_data.set_end_line();
        }
        ('.', LS::Ident(ident), _) if !ident.contains('.') && ident.is_number() => {
            ident.push('.');
        }
        ('+' | '-', LS::Ident(ident), _)
            if !ident.contains('-') && !ident.contains('+') && ident.last_is_exp() =>
        {
            ident.push(ch);
        }
        (
            '(' | ')' | '[' | ']' | '{' | '}' | '~' | '!' | '*' | '&' | '%' | '/' | '>' | '<' | '='
            | '|' | '^' | ',' | '?' | ':' | ';' | '.' | '+' | '-',
            state,
            _,
        ) => {
            if let LS::Symbols(symbol_state) = state {
                if let Some((size, symbol)) = symbol_state.push(ch, lex_data, location) {
                    lex_data.push_token(Token::from_symbol(symbol, size, location));
                }
            } else {
                end_current(state, lex_data, location);
                *state = LS::Symbols(SymbolState::from(ch));
            }
        }

        /* Whitespace: end of everyone */
        (_, state, _) if ch.is_whitespace() => {
            end_current(state, lex_data, location);
        }

        // Whitespace: end of everyone
        (_, LS::Ident(val), _) if ch.is_alphanumeric() || matches!(ch, '_' | '.' | '+' | '-') => {
            val.push(ch);
        }
        (_, state, _) if ch.is_alphanumeric() || matches!(ch, '_') => {
            if let LS::Symbols(symbol) = state
                && symbol.last() == Some('.')
                && ch.is_ascii_digit()
            {
                symbol.clear_last();
                end_current(state, lex_data, location);
                state.new_ident_str(format!("0.{ch}"));
            } else {
                end_current(state, lex_data, location);
                state.new_ident(ch);
            }
        }
        (_, _, _) => {
            lex_data.push_err(location.to_failure(format!("Character '{ch}' not supported.")));
        }
    }
}

/// Function that lexes a whole source file.
///
/// This function creates the automaton and the data to be modified by the other
/// functions. Every character is parsed one by one, and the state is modified
/// accordingly. When the state changes, the buffers of the state are empty into
/// the data.
#[inline]
pub fn lex_file(content: &str, location: &mut Location) -> Res<Vec<Token>> {
    let mut lex_data = LexingData::default();
    let mut lex_state = LS::default();

    for line in content.lines() {
        lex_line(line, location, &mut lex_data, &mut lex_state);
        if let Err(err) = location.incr_line() {
            lex_data.push_err(err);
        }
    }

    lex_data.into_res()
}

/// Function that lexes one line.
///
/// It stops at the first erroneous character, or at the end of the line if
/// everything was ok.
fn lex_line(line: &str, location: &mut Location, lex_data: &mut LexingData, lex_state: &mut LS) {
    lex_data.newline();
    let mut escape_state = EscapeState::False;
    let trimmed = line.trim_end();
    if trimmed.is_empty() {
        return;
    }
    let last = trimmed.len().checked_sub(1).expect("trimmed is not empty");
    for (idx, ch) in trimmed.chars().enumerate() {
        lex_char(
            ch,
            location,
            lex_data,
            lex_state,
            &mut escape_state,
            idx == last,
        );
        if let Err(err) = location.incr_col() {
            lex_data.push_err(err);
        }
        if lex_data.is_end_line() {
            break;
        }
    }
    if escape_state != EscapeState::Single {
        end_current(lex_state, lex_data, location);
    }
    if line.trim_end().ends_with('\\') {
        if line.ends_with(char::is_whitespace) {
            lex_data.push_err(location.to_suggestion(
                "found white space after '\\' at EOL. Please remove the space.".to_owned(),
            ));
        }
    } else {
        *lex_state = LS::default();
    }
}
