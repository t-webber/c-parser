use super::parsing_state::{ParsingState, TriBool};
use super::Token;
use crate::errors::{compile::CompileError, location::Location};
use crate::to_error;
use core::mem::take;

pub fn end_both(p_state: &mut ParsingState, tokens: &mut Vec<Token>, location: &Location) {
    end_operator(p_state, tokens, location);
    end_literal(p_state, tokens, location);
}

fn end_literal(p_state: &mut ParsingState, tokens: &mut Vec<Token>, location: &Location) {
    if !p_state.literal.is_empty() {
        let mut chars = p_state.literal.chars();
        let first = chars.next().unwrap();
        if first.is_numeric() {
            if chars.all(|ch| ch.is_numeric() || ch == '_' || ch == '.') {
                tokens.push(Token::from_number(
                    take(&mut p_state.literal),
                    p_state,
                    location,
                ));
            } else {
                p_state.literal.clear();
                p_state.errors.push(to_error!(location, "Number immediatly followed by character. Literals can only start with alphabetic characters. Did you forget a space?"));
            };
        } else if first.is_alphabetic() {
            tokens.push(Token::from_identifier(
                take(&mut p_state.literal),
                p_state,
                location,
            ));
        } else {
            p_state.literal.clear();
            p_state.errors.push(to_error!(
                location,
                "Literals must start with a alphanumeric character, found symbol {first}."
            ));
        }
    }
}

pub fn end_operator(p_state: &mut ParsingState, tokens: &mut Vec<Token>, location: &Location) {
    let mut idx: usize = 0;
    while !p_state.is_empty() && idx <= 2 {
        idx += 1;
        if let Some((size, symbol)) = p_state.try_to_operator() {
            tokens.push(Token::from_symbol(symbol, size, p_state, location));
        } else {
            panic!(
                "This can't happen, as p_state is not empty! ParsingState: {:?}",
                &p_state
            );
        }
    }
    assert!(p_state.is_empty(), "Not possible: executing 3 times the conversion, with stritcly decreasing number of non empty elements! This can't happen. ParsingState: {:?}", &p_state);
}

fn end_string(p_state: &mut ParsingState, tokens: &mut Vec<Token>, location: &Location) {
    if !p_state.literal.is_empty() {
        tokens.push(Token::from_str(
            take(&mut p_state.literal),
            p_state,
            location,
        ));
    }
    assert!(p_state.literal.is_empty(), "Not possible: The string was just cleared, except if i am stupid and take doesn't clear ??!! ParsingState:{:?}", &p_state);
}

pub fn handle_double_quotes(
    p_state: &mut ParsingState,
    tokens: &mut Vec<Token>,
    location: &Location,
) {
    if p_state.double_quote {
        end_string(p_state, tokens, location);
        p_state.double_quote = false;
    } else {
        end_both(p_state, tokens, location);
        p_state.double_quote = true;
    }
}

pub fn handle_escaped_character(ch: char, p_state: &mut ParsingState, location: &Location) {
    if p_state.double_quote || p_state.single_quote == TriBool::True {
        match ch {
            'n' => p_state.literal.push('\n'),
            't' => p_state.literal.push('\t'),
            'u' => todo!(),
            '\\' => p_state.literal.push('\\'),
            '\0' => p_state.literal.push('\0'),
            _ => p_state
                .errors
                .push(to_error!(location, "Character {ch} can not be escaped.")),
        }
    } else {
        p_state.errors.push(to_error!(
            location,
            "\\ escape character can only be used inside a string or char to espace a character."
        ));
    }
    p_state.escape = false;
}

pub fn handle_single_quotes(p_state: &mut ParsingState, location: &Location) {
    match p_state.single_quote {
        TriBool::False => p_state.single_quote = TriBool::True,
        TriBool::Intermediate => p_state.single_quote = TriBool::False,
        TriBool::True => p_state.errors.push(to_error!(
            location,
            "A char must contain exactly one element, but none where found. Did you mean '\\''?"
        )),
    }
}

pub fn handle_symbol(
    ch: char,
    p_state: &mut ParsingState,
    location: &Location,
    tokens: &mut Vec<Token>,
) {
    end_literal(p_state, tokens, location);
    if let Some((size, symbol)) = p_state.push(ch) {
        tokens.push(Token::from_symbol(symbol, size, p_state, location));
    }
}
