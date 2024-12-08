use super::parsing_state::{ParsingState, StateState, TriBool};
use super::Token;
use crate::errors::{compile::CompileError, location::Location};
use crate::to_error;
use core::mem::take;

pub fn end_both(p_state: &mut ParsingState, tokens: &mut Vec<Token>, location: &Location) {
    end_operator(p_state, tokens);
    end_literal(p_state, tokens, location);
}

pub fn end_literal(p_state: &mut ParsingState, tokens: &mut Vec<Token>, location: &Location) {
    if !p_state.literal.is_empty() {
        let mut chars = p_state.literal.chars();
        let first = chars.next().unwrap();
        if first.is_numeric() {
            if chars.all(|ch| ch.is_numeric() || ch == '_' || ch == '.') {
                tokens.push(Token::Number(take(&mut p_state.literal)));
            } else {
                p_state.literal.clear();
                p_state.errors.push(to_error!(location, "Number immediatly followed by character. Literals can only start with alphabetic characters. Did you forget a space?"));
            };
        } else if first.is_alphabetic() {
            tokens.push(Token::Identifier(take(&mut p_state.literal)));
        } else {
            p_state.literal.clear();
            p_state.errors.push(to_error!(
                location,
                "Literals must start with a alphanumeric character, found symbol {first}."
            ));
        }
    }
}

pub fn end_operator(p_state: &mut ParsingState, tokens: &mut Vec<Token>) {
    let mut idx: usize = 0;
    while !p_state.is_empty() && idx <= 2 {
        idx += 1;
        if let Some(operator) = p_state.try_to_operator() {
            tokens.push(Token::Symbol(operator));
        } else {
            panic!(
                "This can't happen, as p_state is not empty! ParsingState: {:?}",
                &p_state
            );
        }
    }
    assert!(p_state.is_empty(), "Not possible: executing 3 times the conversion, with stritcly decreasing number of non empty elements! This can't happen. ParsingState: {:?}", &p_state);
}

fn end_string(p_state: &mut ParsingState, tokens: &mut Vec<Token>) {
    if !p_state.literal.is_empty() {
        tokens.push(Token::Str(take(&mut p_state.literal)));
    }
    assert!(p_state.literal.is_empty(), "Not possible: The string was just cleared, except if i am stupid and take doesn't clear ??!! ParsingState:{:?}", &p_state);
}

pub fn handle_double_quotes(
    p_state: &mut ParsingState,
    tokens: &mut Vec<Token>,
    location: &Location,
) {
    if p_state.double_quote {
        end_string(p_state, tokens);
        p_state.double_quote = false;
    } else {
        end_both(p_state, tokens, location);
        p_state.double_quote = true;
    }
}

pub fn handle_escaped_character(ch: char, p_state: &mut ParsingState, location: &Location) {
    if p_state.p_state == StateState::None
        || p_state.p_state == StateState::Symbol
        || (!p_state.double_quote && p_state.single_quote != TriBool::True)
    {
        p_state.errors.push(to_error!(
            location,
            "\\ escape character can only be used inside a string or char to espace a character."
        ));
    } else {
        match ch {
            'n' => p_state.literal.push('\n'),
            't' => p_state.literal.push('\t'),
            _ => p_state
                .errors
                .push(to_error!(location, "Character {ch} can not be escaped.")),
        }
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
