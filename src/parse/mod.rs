mod numbers;
mod parsing_state;
mod special_chars;

use core::fmt;

use crate::{errors::compile::Res, to_suggestion};
use crate::errors::location::Location;
use crate::to_error;
use numbers::Number;
use parsing_state::{CharStatus, CommentStatus, EscapeStatus, ParsingState};
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

fn parse_line(expression: &str, location: &mut Location, p_state: &mut ParsingState) {
    //TODO: when an error is found: break; because otherwise the following are parsed wrong.
    // for example: in '0.5p+3f', p is invalid so 3f is parsed, but is illegal and produces an error. 
    for ch in expression.chars() {
        match ch {
            /* Inside comment */
            '/' if p_state.comments == CommentStatus::Star => {
                p_state.comments = CommentStatus::False;
            }
            '*' if p_state.comments == CommentStatus::True => {
                p_state.comments = CommentStatus::Star;
            }
            _ if p_state.comments == CommentStatus::True => (),
            _ if p_state.comments == CommentStatus::Star => p_state.comments = CommentStatus::True,

            /* Escaped character */
            _ if p_state.escape != EscapeStatus::Trivial(false) => {
                handle_escaped(ch, p_state, location);
            }

            /* Create comment */
            '*' if p_state.last_symbol() == Some('/') => {
                p_state.clear_last();
                end_both(p_state, location);
                p_state.comments = CommentStatus::True;
            }

            /* Escape character */
            '\\' => p_state.escape = EscapeStatus::Trivial(true),

            /* Static strings and chars*/
            // open/close
            '\'' => handle_single_quotes(p_state, location),
            '\"' => handle_double_quotes(p_state, location),
            // middle
            _ if p_state.single_quote == CharStatus::Written => p_state.push_err(to_error!(
                location,
                "A char must contain only one character. A token is a number iff it contains only alphanumeric chars and '_' and '.' and starts with a digit.")),
            _ if p_state.single_quote == CharStatus::Opened => {
                p_state.push_token(Token::from_char(ch, location));
                p_state.single_quote = CharStatus::Written;
            }
            _ if p_state.double_quote => p_state.literal.push(ch),

            /* Operator symbols */
            '/' if p_state.last_symbol() == Some('/') => {
                end_both(p_state, location);
                return;
            },
            '(' | ')' | '[' | ']' | '{' | '}' | '~' | '!' | '*' | '&' | '%' | '/'
            | '>' | '<' | '=' | '|' | '^' | ',' | '?' | ':' | ';' => {
                handle_symbol(ch, p_state, location);
            }
            '.' if !p_state.is_number() || p_state.literal.contains('.') => handle_symbol(ch, p_state, location), 
            '+' | '-' if !p_state.is_number() => {handle_symbol(ch, p_state, location)},
            '+' | '-' if p_state.is_hex() && !matches!(p_state.last_literal_char().unwrap_or('\0'), 'p' | 'P') => {handle_symbol(ch, p_state, location)},
            '+' | '-' if !p_state.is_hex() && !matches!(p_state.last_literal_char().unwrap_or('\0'), 'e' | 'E') => {handle_symbol(ch, p_state, location)},

            /* Whitespace: end of everyone */
            _ if ch.is_whitespace() => {
                end_both(p_state, location);
            }

            // Whitespace: end of everyone
            _ if ch.is_alphanumeric() || matches!(ch, '_' | '.' | '+' | '-') => {
                end_operator(p_state, location);
                p_state.literal.push(ch);
            }
            _ => {
                end_both(p_state, location);
                p_state.push_err(to_error!(
                    location,
                    "Character not supported in this context: '{ch}'"
                ));
            }
        }
        if p_state.failed {
            return;
        }
        location.incr_col();
    }
    if p_state.escape != EscapeStatus::Trivial(false) {
        if p_state.escape == EscapeStatus::Trivial(true) {
            let token = Token::from_symbol(Symbol::Eol, 1, location);
            p_state.push_token(token);
        } else {
            end_escape_sequence(p_state, location);
        }
    }
    end_both(p_state, location);
}


pub fn parse_file(content: &str, location: &mut Location) -> Res<Vec<Token>> {
    let mut p_state = ParsingState::new();

    for line in content.lines() {
        parse_line(line.trim_end(), location, &mut p_state);
        if line.ends_with(char::is_whitespace) && line.trim_end().ends_with('\\') {
            p_state.push_err(to_suggestion!(
                location,
                "found white space after '\\' at EOL. Please remove the space."
            ));
        }
        p_state.clear_all();
        location.new_line();
    };

    Res::from((p_state.take_tokens(), p_state.take_errors()))
}