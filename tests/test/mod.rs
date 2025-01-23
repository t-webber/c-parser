use std::fs;

use c_parser::*;

mod blk;
mod ctrl;
mod err;
mod func;
mod nb;
mod op;
mod pars;
mod str;
mod var;

const SEP: &str = "--------------------\n";

#[macro_export]
macro_rules! make_string_tests {
        ($($name:ident: $input:expr => $output:expr)*) => {
            $(
                #[test]
                fn $name() {
                    super::test_string($input, $output)
                }
            )*
        };
}

#[macro_export]
macro_rules! make_string_error_tests {
    ($($name:ident: $input:expr => $output:expr)*) => {
        $(
            #[test]
            fn $name() {
                super::test_string_error($input, $output)
            }
        )*

    };
}

fn test_string(content: &str, expected: &str) {
    let files = &[(String::new(), content)];
    eprint!("{SEP}Content = {content}\n{SEP}");
    print!("a");
    let mut location = Location::from(String::new());
    let tokens = lex_file(content, &mut location).unwrap_or_display(files, "lexer");
    let node = parse_tokens(tokens).unwrap_or_display(files, "parser");
    let computed = format!("{node}");
    print!("b");
    assert!(
        expected == computed,
        "{SEP}Mismatch! Expected:\n!{expected:?}!\n!= Computed\n!{computed:?}!\n{SEP}Len e = {} | Len c = {}\n{SEP}",
        expected.len(),
        computed.len()
    );
}

fn test_string_error(content: &str, expected: &str) {
    let files = &[(String::new(), content)];
    let mut location = Location::from(String::new());
    let res = lex_file(content, &mut location);
    let computed = if res.errors_empty() {
        let tokens = res.unwrap_or_display(files, "lexer");
        println!("Tokens = {}", display_tokens(&tokens));
        let parsed = parse_tokens(tokens);
        let errors = parsed.get_displayed_errors(files, "parser");
        if errors.is_empty() {
            panic!("Ast = {}", parsed.unwrap_or_display(files, "never happens"))
        }
        errors
    } else {
        res.get_displayed_errors(files, "lexer")
    };
    if expected != computed {
        fs::write("expected.txt", expected).unwrap();
        fs::write("computed.txt", &computed).unwrap();
        panic!(
            "{SEP}Mismatch! Expected:\n!{expected}!\n!= Computed\n!{computed}!{SEP}Len e = {} | Len c = {}{SEP}",
            expected.len(),
            computed.len()
        );
    }
}
