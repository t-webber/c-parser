mod blocks;
mod control_flows;
mod errors;
mod functions;
mod numbers;
mod operators;
mod strings;
mod variables;

use std::fs;

use c_parser::*;

#[macro_export]
/// Convenience macro to test parsing on a string.
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
/// Convenience macro to test parsing on a string, when the parsing is supposed
/// to return an error.
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

const SEP: &str = "--------------------\n";

fn test_string(content: &str, expected: &str) {
    let files = &[(String::new(), content)];
    eprint!("{SEP}Content = {content}\n{SEP}");
    let mut location = LocationPointer::from("");
    let tokens = lex_file(content, &mut location)
        .unwrap_or_display(files)
        .unwrap();
    let node = parse_tokens(tokens).unwrap_or_display(files).unwrap();
    let computed = format!("{node}");
    assert!(
        expected == computed,
        "{SEP}Mismatch! Expected:\n!{expected:?}!\n!= Computed\n!{computed:?}!\n{SEP}Len e = {} | Len c = {}\n{SEP}",
        expected.len(),
        computed.len()
    );
}

fn test_string_error(content: &str, expected: &str) {
    let files = &[(String::new(), content)];
    let mut location = LocationPointer::from("");
    let computed = lex_file(content, &mut location)
        .stop_at_failure()
        .and_then(|tokens| {
            println!("Tokens = {}", display_tokens(&tokens));
            parse_tokens(tokens)
        })
        .as_displayed_errors(files);
    if expected != computed {
        fs::write("expected.txt", expected).unwrap();
        fs::write("computed.txt", &computed).unwrap();
        unreachable!(
            "{SEP}Mismatch! Expected:\n!{expected}!\n!= Computed\n!{computed}!{SEP}Len e = {} | Len c = {}{SEP}",
            expected.len(),
            computed.len()
        );
    }
}
