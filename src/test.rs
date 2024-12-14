use crate::errors::{compile::Res, location::Location};
use crate::lexer;
use std::fs;

#[allow(clippy::unwrap_used)]
fn test_lexer_on_file(nb: usize) {
    let path = format!(".tests/lexer-{nb}.c");
    let content = fs::read_to_string(&path).unwrap();
    let mut location = Location::from(path.clone());
    let Res { errors, .. } = lexer::lex_file(&content, &mut location);
    assert!(
        errors.is_empty(),
        "Path = {path}\nErrors = {:?}",
        errors.into_iter().map(|x| x.get().1).collect::<Vec<_>>()
    );
}

#[test]
fn test_lexer_1() {
    test_lexer_on_file(1);
}

#[test]
fn test_lexer_2() {
    test_lexer_on_file(2);
}
