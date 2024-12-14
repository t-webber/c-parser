use crate::errors::display::display_errors;
use crate::errors::{compile::Res, location::Location};
use crate::lexer;
use std::fs;

#[allow(
    clippy::unwrap_used,
    clippy::print_stdout,
    clippy::case_sensitive_file_extension_comparisons
)]
#[test]
fn lext_lexer() {
    let mut panic = false;
    for ref file in fs::read_dir("./data")
        .unwrap()
        .map(|file| file.unwrap().file_name().into_string().unwrap())
        .filter(|x| x.starts_with("lexer-") && x.ends_with(".c"))
    {
        let path = format!("./data/{file}");
        let content = fs::read_to_string(&path).unwrap();
        let mut location = Location::from(path.clone());
        let Res { errors, .. } = lexer::lex_file(&content, &mut location);
        if !errors.is_empty() {
            display_errors(errors, &[(path, &content)]);
            panic = true;
        }
    }
    assert!(!panic);
}
