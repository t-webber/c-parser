mod parsing_state;
mod special_chars;

use core::{fmt, mem};

use crate::errors::compile::Res;
use crate::errors::location::Location;
use crate::to_error;
use parsing_state::{CharStatus, EscapeStatus, ParsingState};
use special_chars::{
    end_both, end_operator, handle_double_quotes, handle_escaped, handle_single_quotes,
    handle_symbol,
};

#[derive(Debug)]
pub enum Symbol {
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

    pub fn from_number(number: String, p_state: &mut ParsingState, location: &Location) -> Self {
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
    Number(String),
    Str(String),
    Symbol(Symbol),
}

pub fn parse(expression: &str, location: &mut Location) -> Res<Vec<Token>> {
    let mut tokens = vec![];
    let mut p_state = ParsingState::from(location.to_owned());
    for ch in expression.chars() {
        // println!("ParsingState = {:?}\tTokens = {:?}", &p_state, &tokens);
        match ch {
            /* Escape character */
            _ if p_state.escape != EscapeStatus::Trivial(false) => {
                handle_escaped(ch, &mut p_state, location)
            }
            '\\' => p_state.escape = EscapeStatus::Trivial(true),

            /* Static strings and chars*/
            // open/close
            '\'' => handle_single_quotes(&mut p_state, location),
            '\"' => handle_double_quotes(&mut p_state, &mut tokens, location),
            // middle
            _ if p_state.single_quote == CharStatus::Written => p_state.errors.push(to_error!(
                location,
                "A char must contain only one character"
            )),
            _ if p_state.single_quote == CharStatus::Opened => {
                tokens.push(Token::from_char(ch, location));
                p_state.single_quote = CharStatus::Written;
            }
            _ if p_state.double_quote => p_state.literal.push(ch),

            // Operator symbols
            '+' | '-' | '(' | ')' | '[' | ']' | '{' | '}' | '~' | '!' | '*' | '&' | '%' | '/'
            | '>' | '<' | '=' | '|' | '^' | ',' | '?' | ':' => {
                handle_symbol(ch, &mut p_state, location, &mut tokens);
            }
            '.' if !p_state.is_number() => handle_symbol(ch, &mut p_state, location, &mut tokens),

            // Whitespace: end of everyone
            _ if ch.is_whitespace() => {
                end_both(&mut p_state, &mut tokens, location);
                p_state.initial_location.incr_col();
            }

            // Whitespace: end of everyone
            _ if ch.is_alphanumeric() || ch == '_' || ch == '.' => {
                end_operator(&mut p_state, &mut tokens, location);
                p_state.literal.push(ch);
            }
            _ => {
                end_both(&mut p_state, &mut tokens, location);
                p_state.errors.push(to_error!(
                    location,
                    "Character not supported by compiler: {ch}"
                ));
            }
        }
        location.incr_col();
    }
    end_both(&mut p_state, &mut tokens, location);
    Res::from((tokens, p_state.errors))
}
