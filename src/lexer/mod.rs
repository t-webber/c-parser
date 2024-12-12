mod end_state;
mod handle_state;
mod numbers;
mod types;
use crate::errors::location::Location;
use crate::to_error;
use crate::{errors::compile::Res, to_suggestion};
use end_state::end_current;
use handle_state::{handle_escape_one_char, handle_escaped_sequence};
use types::escape_state::EscapeStatus;
use types::lexing_data::LexingData;
use types::lexing_state::{CommentStatus, LexingStatus, SymbolStatus};
use types::tokens_types::Token;

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

fn lex_char(
    ch: char,
    location: &Location,
    lex_data: &mut LexingData,
    lex_status: &mut LexingStatus,
    escape_status: &mut EscapeStatus,
    eol: bool,
) -> Result<bool, ()> {
    let mut start_of_line = false;
    use LexingStatus::*;
    match (ch, lex_status, escape_status) {
        (_, StartOfLine, _) if ch.is_whitespace() => start_of_line = true,
        /* Inside comment */
        ('/', status @ Comment(CommentStatus::Star), _) => {
            *status = Comment(CommentStatus::False);
        }
        ('*', status @ Comment(CommentStatus::True), _) => {
            *status = Comment(CommentStatus::Star);
        }
        (_, Comment(CommentStatus::True), _) => (),
        (_, status @ Comment(CommentStatus::Star), _) => {
            *status = Comment(CommentStatus::True);
        }
        /* Escaped character */
        (_, status @ Char(None), escape @ EscapeStatus::Single) => {
            if let Some(ch) = handle_escape_one_char(ch, lex_data, escape, location) {
                // escape_status is reset by handler
                *status = Char(Some(ch));
            }
        }
        (_, Str(val), escape @ EscapeStatus::Single) => {
            if let Some(ch) = handle_escape_one_char(ch, lex_data, escape, location) {
                // escape_status is reset by handler
                val.push(ch);
            }
        }
        (_, Char(None) | Str(_), EscapeStatus::Sequence(escape_sequence)) => {
            handle_escaped_sequence(ch, escape_sequence, lex_data, location);
        }
        (_, _, EscapeStatus::Single | EscapeStatus::Sequence(_)) => {
            panic!("Can't happend because error raised on escape creation if wrong state.")
        }
        /* Create comment */
        ('*', status, _) if status.symbol().and_then(|symb| symb.last()) == Some('/') => {
            status.clear_last_symbol();
            end_current(status, lex_data, location);
            *status = Comment(CommentStatus::True);
        }

        /* Escape character */
        ('\\', Char(None) | Str(_), escape) => *escape = EscapeStatus::Single,
        ('\\', _, escape) if eol => *escape = EscapeStatus::Single,
        ('\\', state, _) => lex_data.push_err(to_error!(
            location,
            "Escape characters are only authorised in strings or chars, not in '{}' context.",
            state.repr()
        ))?,

        /* Static strings and chars*/
        // open/close
        ('\'', status @ Char(_), _) => end_current(status, lex_data, location),
        ('\'', status, _) if !matches!(status, Str(_)) => {
            end_current(status, lex_data, location);
            *status = LexingStatus::Char(None)
        }
        ('\"', status @ Str(_), _) => {
            end_current(status, lex_data, location);
        }
        ('\"', status, _) if !matches!(status, Char(_)) => {
            end_current(status, lex_data, location);
            *status = LexingStatus::Str(String::new());
        }
        // middle
        (_, Char(Some(_)), _) => lex_data.push_err(to_error!(
            location,
            "A char must contain only one character."
        ))?,
        (_, status @ Char(None), _) => *status = Char(Some(ch)),
        (_, Str(val), _) => val.push(ch),

        /* Operator symbols */
        ('/', status, _) if status.symbol().and_then(|symb| symb.last()) == Some('/') => {
            status.clear_last_symbol();
            end_current(status, lex_data, location);
            Err(())?;
        }
        ('.', Identifier(ident), _) if !ident.contains('.') && ident.is_number() => {
            ident.push('.');
        }
        ('+' | '-', Identifier(ident), _) if !ident.contains(ch) && ident.last_is_exp() => {}
        (
            '(' | ')' | '[' | ']' | '{' | '}' | '~' | '!' | '*' | '&' | '%' | '/' | '>' | '<' | '='
            | '|' | '^' | ',' | '?' | ':' | ';' | '.',
            status,
            _,
        ) => match status {
            Symbols(symbol_status) => {
                if let Some((size, symbol)) = symbol_status.push(ch) {
                    lex_data.push_token(Token::from_symbol(symbol, size, location));
                }
            }
            _ => {
                end_current(status, lex_data, location);
                *status = LexingStatus::Symbols(SymbolStatus::new(ch));
            }
        },

        /* Whitespace: end of everyone */
        (_, status, _) if ch.is_whitespace() => {
            end_current(status, lex_data, location);
        }

        // Whitespace: end of everyone
        (_, Identifier(val), _) if ch.is_alphanumeric() || matches!(ch, '_' | '.' | '+' | '-') => {
            val.push(ch);
        }
        (_, status, _) if ch.is_alphanumeric() || matches!(ch, '_' | '.' | '+' | '-') => {
            end_current(status, lex_data, location);
            status.new_ident();
        }
        (_, status, _) => {
            lex_data.push_err(to_error!(
                location,
                "Character '{ch}' not supported in context of a '{}'.",
                status.repr()
            ))?;
        }
    }
    Ok(start_of_line)
}

fn lex_line(line: &str, location: &mut Location, lex_data: &mut LexingData) {
    let mut lex_status = LexingStatus::StartOfLine;
    let mut escape_state = EscapeStatus::False;
    let trimed = line.trim_end();
    if trimed.is_empty() {
        return;
    }
    let last = trimed.len() - 1;
    for (idx, ch) in trimed.chars().enumerate() {
        if let Ok(still_start_of_line) = lex_char(
            ch,
            location,
            lex_data,
            &mut lex_status,
            &mut escape_state,
            idx == last,
        ) {
            if !(still_start_of_line && lex_status == LexingStatus::StartOfLine) {
                lex_status = LexingStatus::Unset;
            }
            location.incr_col();
        } else {
            break;
        }
    }
    if line.ends_with(char::is_whitespace) && line.trim_end().ends_with('\\') {
        lex_data.push_err(to_suggestion!(
            location,
            "found white space after '\\' at EOL. Please remove the space."
        ));
    }
    end_current(&mut lex_status, lex_data, &location);
}

pub fn lex_file(content: &str, location: &mut Location) -> Res<Vec<Token>> {
    let mut lex_data = LexingData::default();

    for line in content.lines() {
        lex_line(line, location, &mut lex_data);
        location.new_line();
    }

    Res::from((lex_data.take_tokens(), lex_data.take_errors()))
}
