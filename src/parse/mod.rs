mod parsing_state;
mod special_chars;

use crate::errors::compile::{CompileError, Res};
use crate::errors::location::Location;
use crate::to_error;
use parsing_state::{ParsingState, TriBool};
use special_chars::{
    end_both, end_literal, end_operator, handle_double_quotes, handle_escaped_character,
    handle_single_quotes,
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

#[derive(Debug)]
pub enum Token {
    Char(char),
    Identifier(String),
    Number(String),
    Str(String),
    Symbol(Symbol),
}

pub fn parse(expression: &str, location: &mut Location) -> Res<Vec<Token>> {
    let mut tokens = vec![];
    let mut p_state = ParsingState::default();
    for ch in expression.chars() {
        // println!("ParsingState = {:?}\tTokens = {:?}", &p_state, &tokens);
        match ch {
            /* Escape character */
            _ if p_state.escape => handle_escaped_character(ch, &mut p_state, location),
            '\\' => p_state.escape = true,

            /* Static strings and chars*/
            // open/close
            '\'' => handle_single_quotes(&mut p_state, location),
            '\"' => handle_double_quotes(&mut p_state, &mut tokens, location),
            // middle
            _ if p_state.single_quote == TriBool::Intermediate => p_state.errors.push(to_error!(
                location,
                "A char must contain only one character"
            )),
            _ if p_state.single_quote == TriBool::True => {
                tokens.push(Token::Char(ch));
                p_state.single_quote = TriBool::Intermediate;
            }
            _ if p_state.double_quote => p_state.literal.push(ch),

            // Operator symbols
            '+' | '-' | '(' | ')' | '[' | ']' | '{' | '}' | '~' | '!' | '*' | '&' | '%' | '/'
            | '>' | '<' | '=' | '|' | '^' | ',' | '?' | ':' => {
                end_literal(&mut p_state, &mut tokens, location);
                if let Some(operator) = p_state.push(ch) {
                    tokens.push(Token::Symbol(operator));
                }
            }
            '.' if !p_state.is_number() => {
                end_literal(&mut p_state, &mut tokens, location);
                if let Some(operator) = p_state.push(ch) {
                    tokens.push(Token::Symbol(operator));
                }
            }

            // Whitespace: end of everyone
            _ if ch.is_whitespace() => {
                end_both(&mut p_state, &mut tokens, location);
            }

            // Whitespace: end of everyone
            _ if ch.is_alphanumeric() || ch == '_' || ch == '.' => {
                end_operator(&mut p_state, &mut tokens);
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
