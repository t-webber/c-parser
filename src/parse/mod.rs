mod numbers;
mod parsing_state;
mod special_chars;

use core::{fmt, mem};

use crate::errors::compile::Res;
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
        p_state: &mut ParsingState,
        location: &Location,
    ) -> Self {
        Self {
            value: TokenValue::Identifier(identifier),
            location: mem::replace(&mut p_state.initial_location, location.to_owned()),
        }
    }

    pub fn from_number(number: Number, p_state: &mut ParsingState, location: &Location) -> Self {
        Self {
            value: TokenValue::Number(number),
            location: mem::replace(&mut p_state.initial_location, location.to_owned()),
        }
    }

    pub fn from_str(str: String, p_state: &mut ParsingState, location: &Location) -> Self {
        Self {
            value: TokenValue::Str(str),
            location: mem::replace(&mut p_state.initial_location, location.to_owned()),
        }
    }

    pub fn from_symbol(
        symbol: Symbol,
        size: usize,
        p_state: &mut ParsingState,
        location: &Location,
    ) -> Self {
        location.clone_into(&mut p_state.initial_location);
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

pub fn parse(expression: &str, location: &mut Location) -> Res<Vec<Token>> {
    let mut p_state = ParsingState::from(location.to_owned());
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
                handle_escaped(ch, &mut p_state, location);
            }

            /* Create comment */
            '*' if p_state.last() == Some('/') => {
                p_state.clear_last();
                end_both(&mut p_state, location);
                p_state.comments = CommentStatus::True;
            }

            /* Escape character */
            '\\' => p_state.escape = EscapeStatus::Trivial(true),

            /* Static strings and chars*/
            // open/close
            '\'' => handle_single_quotes(&mut p_state, location),
            '\"' => handle_double_quotes(&mut p_state, location),
            // middle
            _ if p_state.single_quote == CharStatus::Written => p_state.push_err(to_error!(
                location,
                "A char must contain only one character. A token is a number iff it contains only alphanumeric chars and '_' and '.' and starts with a digit."            )),
            _ if p_state.single_quote == CharStatus::Opened => {
                p_state.push_token(Token::from_char(ch, location));
                p_state.single_quote = CharStatus::Written;
            }
            _ if p_state.double_quote => p_state.literal.push(ch),

            /* Operator symbols */
            '/' if p_state.last() == Some('/') => {
                p_state.clear();
                break;
            }
            '+' | '-' | '(' | ')' | '[' | ']' | '{' | '}' | '~' | '!' | '*' | '&' | '%' | '/'
            | '>' | '<' | '=' | '|' | '^' | ',' | '?' | ':' | ';' => {
                handle_symbol(ch, &mut p_state, location);
            }
            '.' if !p_state.is_number() => handle_symbol(ch, &mut p_state, location),

            /* Whitespace: end of everyone */
            _ if ch.is_whitespace() => {
                end_both(&mut p_state, location);
                p_state.initial_location.incr_col();
            }

            // Whitespace: end of everyone
            _ if ch.is_alphanumeric() || ch == '_' || ch == '.' => {
                end_operator(&mut p_state, location);
                p_state.literal.push(ch);
            }
            _ => {
                end_both(&mut p_state, location);
                p_state.push_err(to_error!(
                    location,
                    "Character not supported by parser: '{ch}'"
                ));
            }
        }
        location.incr_col();
    }
    if p_state.escape != EscapeStatus::Trivial(false) {
        if p_state.escape == EscapeStatus::Trivial(true) {
            let token = Token::from_symbol(Symbol::Eol, 1, &mut p_state, location);
            p_state.push_token(token);
        } else {
            end_escape_sequence(&mut p_state, location);
        }
    }
    end_both(&mut p_state, location);
    Res::from((p_state.take_tokens(), p_state.take_errors()))
}
