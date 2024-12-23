use expressions::prelude::*;
use std::fs;

const PREFIX: &str = "./tests/data/lexer-";

#[expect(clippy::unwrap_used)]
fn test_lexer_on_file(file: &str) {
    let path = format!("{PREFIX}{file}.c");
    let content = fs::read_to_string(&path).unwrap();
    let mut location = Location::from(path.clone());
    let _tokens = lex_file(&content, &mut location).unwrap_or_display(&[(path, &content)], "lexer");
}

fn test_lexer_on_nuber(content: &str, expect: &str) {
    let path = String::new();
    let mut location = Location::from(path.as_str());
    let tokens = lex_file(content, &mut location).unwrap_or_display(&[(path, content)], "lexer");
    assert!(tokens.len() == 1, "Lexer error: cut expression into 2 tokens, but only a number was expected: {content} was cut into {tokens:?}");
    let value = tokens.first().unwrap().get_value();
    assert!(*value == TokenValue::Number());
}

#[test]
fn lexer_escape() {
    test_lexer_on_file("escape");
}

#[test]
fn lexer_general() {
    test_lexer_on_file("general");
}

macro_rules! gen_number_test {
    ($i:expr) => {
        #[test]
        fn lexer_number_$i() {}
    };
}

#[test]
fn lexer_numbers_1() {
    test_lexer_on_file("numbers-1");
}

#[test]
fn lexer_numbers_2() {
    test_lexer_on_file("numbers-2");
}
