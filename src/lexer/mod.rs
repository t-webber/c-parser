mod numbers;
mod lexing_state;
mod special_chars;

use core::fmt;

use crate::{errors::compile::Res, to_suggestion};
use crate::errors::location::Location;
use crate::to_error;
use numbers::Number;
use lexing_state::{CharStatus, CommentStatus, EscapeStatus, ParsingState};
use special_chars::{
    end_both, end_escape_sequence, end_operator, handle_double_quotes, handle_escaped,
    handle_single_quotes, handle_symbol,
};

#[derive(Debug)]
pub enum Symbol {
    // Unique
    Eol,
    // one character
    Ampercent,
    Assign,
    BitwiseNot,
    BitwiseOr,
    BitwiseXor,
    BraceClose,
    BraceOpen,
    BracketClose,
    BracketOpen,
    Colon,
    Comma,
    Divide,
    Dot,
    Gt,
    Interrogation,
    LogicalNot,
    Lt,
    Minus,
    Modulo,
    ParenthesisClose,
    ParenthesisOpen,
    Plus,
    SemiColon,
    Star,
    // two characters
    AddAssign,
    AndAssign,
    Arrow,
    Decrement,
    Different,
    DivAssign,
    Equal,
    Ge,
    Increment,
    Le,
    LogicalAnd,
    LogicalOr,
    ModAssign,
    MulAssign,
    OrAssign,
    ShiftLeft,
    ShiftRight,
    SubAssign,
    XorAssign,
    // three characters
    ShiftLeftAssign,
    ShiftRightAssign,
}
pub struct Token {
    location: Location,
    value: TokenValue,
}

impl Token {
    pub fn from_char(ch: char, location: &Location) -> Self {
        Self {
            value: TokenValue::Char(ch),
            location: location.to_owned(),
        }
    }

    pub fn from_identifier(
        identifier: String,
        location: &Location,
    ) -> Self {
        Self {
            location: location.to_owned().into_past(identifier.len()),
            value: TokenValue::Identifier(identifier),
        }
    }

    pub fn from_number(number: Number, location: &Location) -> Self {
        Self {
            value: TokenValue::Number(number),
            location: location.to_owned(),
        }
    }

    pub fn from_str(str: String, location: &Location) -> Self {
        Self {
            location: location.to_owned().into_past(str.len()),
            value: TokenValue::Str(str),
        }
    }

    pub fn from_symbol(
        symbol: Symbol,
        size: usize,
        location: &Location,
    ) -> Self {
        Self {
            value: TokenValue::Symbol(symbol),
            location: location.to_owned().into_past(size),
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

#[derive(Debug)]
pub enum TokenValue {
    Char(char),
    Identifier(String),
    Number(Number),
    Str(String),
    Symbol(Symbol),
}

fn lex_line(expression: &str, location: &mut Location, lex_state: &mut ParsingState) {
    for ch in expression.chars() {
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
            _ if lex_state.comments == CommentStatus::Star => lex_state.comments = CommentStatus::True,

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
            '\'' => handle_single_quotes(lex_state, location),
            '\"' => handle_double_quotes(lex_state, location),
            // middle
            _ if lex_state.single_quote == CharStatus::Written => lex_state.push_err(to_error!(
                location,
                "A char must contain only one character. A token is a number iff it contains only alphanumeric chars and '_' and '.' and starts with a digit.")),
            _ if lex_state.single_quote == CharStatus::Opened => {
                lex_state.push_token(Token::from_char(ch, location));
                lex_state.single_quote = CharStatus::Written;
            }
            _ if lex_state.double_quote => lex_state.literal.push(ch),

            /* Operator symbols */
            '/' if lex_state.last_symbol() == Some('/') => {
                end_both(lex_state, location);
                return;
            },
            '(' | ')' | '[' | ']' | '{' | '}' | '~' | '!' | '*' | '&' | '%' | '/'
            | '>' | '<' | '=' | '|' | '^' | ',' | '?' | ':' | ';' => {
                handle_symbol(ch, lex_state, location);
            }
            '.' if !lex_state.is_number() || lex_state.literal.contains('.') => handle_symbol(ch, lex_state, location), 
            '+' | '-' if !lex_state.is_number() => {handle_symbol(ch, lex_state, location)},
            '+' | '-' if lex_state.is_hex() && !matches!(lex_state.last_literal_char().unwrap_or('\0'), 'p' | 'P') => {handle_symbol(ch, lex_state, location)},
            '+' | '-' if !lex_state.is_hex() && !matches!(lex_state.last_literal_char().unwrap_or('\0'), 'e' | 'E') => {handle_symbol(ch, lex_state, location)},

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
        location.incr_col();
    }
    if lex_state.escape != EscapeStatus::Trivial(false) {
        if lex_state.escape == EscapeStatus::Trivial(true) {
            let token = Token::from_symbol(Symbol::Eol, 1, location);
            lex_state.push_token(token);
            lex_state.escape = EscapeStatus::Trivial(false);
        } else {
            end_escape_sequence(lex_state, location);
        }
    }
}


pub fn lex_file(content: &str, location: &mut Location) -> Res<Vec<Token>> {
    let mut lex_state = ParsingState::new();

    for line in content.lines() {
        lex_line(line.trim_end(), location, &mut lex_state);
        if line.ends_with(char::is_whitespace) && line.trim_end().ends_with('\\') {
            lex_state.push_err(to_suggestion!(
                location,
                "found white space after '\\' at EOL. Please remove the space."
            ));
        }
        end_operator(&mut lex_state, location);
        assert!(
            lex_state.is_empty(),
            "symbols remaining in state after end_operator"
        );
        lex_state.failed = false;
        lex_state.start_of_line = true;
        location.new_line();
    };

    Res::from((lex_state.take_tokens(), lex_state.take_errors()))
}