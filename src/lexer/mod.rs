mod lexing_state;
mod numbers;
mod special_chars;
pub mod types;
use crate::errors::location::Location;
use crate::to_error;
use crate::{errors::compile::Res, to_suggestion};
use lexing_state::{CommentStatus, EscapeStatus, ParsingState};
use special_chars::{
    end_both, end_escape_sequence, end_operator, handle_double_quotes, handle_escaped,
    handle_single_quotes, handle_symbol,
};
use types::{Symbol, Token};

#[macro_export]
macro_rules! safe_parse_int {
    ($err_prefix:expr, $dest_type:ident, $location:ident, $function_call:expr) => {{
        let parsed: Result<$dest_type, core::num::ParseIntError> = $function_call.map_err(|err| err.into());
        match parsed {
            Ok(nb) => Ok(nb),
            Err(err) => match *err.kind() {
                core::num::IntErrorKind::Empty => panic!("Never happens. Checks for non empty."),
                core::num::IntErrorKind::InvalidDigit => Err(to_error!(
                    $location,
                    "{}invalid decimal number: must contain only ascii digits and at most one '.', one 'e' followed by at most a sign."
                , $err_prefix)),
                core::num::IntErrorKind::PosOverflow => Err(to_error!(
                    $location,
                    "{}postive overflow on decimal number: number is too large to fit in attributed type. Add a suffix or reduce value."
                , $err_prefix)),
                core::num::IntErrorKind::NegOverflow => Err(to_error!(
                    $location,
                    "{}negative overflow on decimal number: number is too large to fit in attributed type. Add a suffix or reduce value."
                , $err_prefix)),
                core::num::IntErrorKind::Zero | _ => panic!("Unexpected error"),

            },
        }
    }};
}

fn lex_char(ch: char, location: &Location, lex_state: &mut ParsingState) {
    let mut start_of_line = false;
    match ch {
        _ if lex_state.failed => return,
        _ if ch.is_whitespace() && lex_state.start_of_line => start_of_line = true,
        /* Inside comment */
        '/' if lex_state.comments == CommentStatus::Star => {
            lex_state.comments = CommentStatus::False;
        }
        '*' if lex_state.comments == CommentStatus::True => {
            lex_state.comments = CommentStatus::Star;
        }
        _ if lex_state.comments == CommentStatus::True => (),
        _ if lex_state.comments == CommentStatus::Star => {
            lex_state.comments = CommentStatus::True;
        }

        /* Escaped character */
        _ if lex_state.escape != EscapeStatus::Trivial(false) => {
            handle_escaped(ch, lex_state, location);
        }

        /* Create comment */
        '*' if lex_state.last_symbol() == Some('/') => {
            lex_state.clear_last();
            end_both(lex_state, location);
            lex_state.comments = CommentStatus::True;
        }

        /* Escape character */
        '\\' => lex_state.escape = EscapeStatus::Trivial(true),

        /* Static strings and chars*/
        // open/close
        '\'' if !lex_state.double_quote => handle_single_quotes(lex_state, location),
        '\"' if !lex_state.single_quote => handle_double_quotes(lex_state, location),
        // middle
        _ if lex_state.single_quote && !lex_state.literal.is_empty() => lex_state.push_err(
            to_error!(location, "A char must contain only one character."),
        ),
        _ if lex_state.single_quote => {
            lex_state.literal.push(ch);
        }
        _ if lex_state.double_quote => lex_state.literal.push(ch),

        /* Operator symbols */
        '/' if lex_state.last_symbol() == Some('/') => {
            lex_state.clear_last();
            end_both(lex_state, location);
            return;
        }
        '(' | ')' | '[' | ']' | '{' | '}' | '~' | '!' | '*' | '&' | '%' | '/' | '>' | '<' | '='
        | '|' | '^' | ',' | '?' | ':' | ';' => {
            handle_symbol(ch, lex_state, location);
        }
        '.' if !lex_state.is_number() || lex_state.literal.contains('.') => {
            handle_symbol(ch, lex_state, location);
        }
        '+' | '-' if !lex_state.is_number() => handle_symbol(ch, lex_state, location),
        '+' | '-'
            if lex_state.is_hex()
                && !matches!(lex_state.last_literal_char().unwrap_or('\0'), 'p' | 'P') =>
        {
            handle_symbol(ch, lex_state, location);
        }
        '+' | '-'
            if !lex_state.is_hex()
                && !matches!(lex_state.last_literal_char().unwrap_or('\0'), 'e' | 'E') =>
        {
            handle_symbol(ch, lex_state, location);
        }

        /* Whitespace: end of everyone */
        _ if ch.is_whitespace() => {
            end_both(lex_state, location);
        }

        // Whitespace: end of everyone
        _ if ch.is_alphanumeric() || matches!(ch, '_' | '.' | '+' | '-') => {
            end_operator(lex_state, location);
            lex_state.literal.push(ch);
        }
        _ => {
            end_both(lex_state, location);
            lex_state.push_err(to_error!(
                location,
                "Character not supported in this context: '{ch}'"
            ));
        }
    }
    lex_state.start_of_line = start_of_line;
}

fn lex_line(line: &str, location: &mut Location, lex_state: &mut ParsingState) {
    for ch in line.trim_end().chars() {
        lex_char(ch, location, lex_state);
        location.incr_col();
    }
    if lex_state.escape != EscapeStatus::Trivial(false) {
        if lex_state.escape == EscapeStatus::Trivial(true) {
            let token = Token::from_symbol(Symbol::Eol, 1, location);
            lex_state.push_token(token);
            lex_state.escape = EscapeStatus::Trivial(false);
        } else {
            let _e = end_escape_sequence(lex_state, location);
        }
    }
    if line.ends_with(char::is_whitespace) && line.trim_end().ends_with('\\') {
        lex_state.push_err(to_suggestion!(
            location,
            "found white space after '\\' at EOL. Please remove the space."
        ));
    }
    end_operator(lex_state, location);
    assert!(
        lex_state.is_empty(),
        "symbols remaining in state after end_operator"
    );
}

pub fn lex_file(content: &str, location: &mut Location) -> Res<Vec<Token>> {
    let mut lex_state = ParsingState::new();

    for line in content.lines() {
        lex_line(line, location, &mut lex_state);
        lex_state.failed = false;
        lex_state.start_of_line = true;
        location.new_line();
    }

    Res::from((lex_state.take_tokens(), lex_state.take_errors()))
}
