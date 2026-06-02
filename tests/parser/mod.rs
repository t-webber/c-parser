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

fn sep(title: &str) -> String {
    const SIDE: &str = "───────────────";
    format!("\n\x1b[33m{SIDE}{title}{SIDE}\x1b[0m\n")
}

fn print_failure(content: &str, computed: &str, expected: &str) {
    if expected == computed {
        return;
    }
    fs::write("expected.txt", expected).unwrap();
    fs::write("computed.txt", computed).unwrap();

    let e_len = expected.len();
    let c_len = computed.len();
    panic!(
        "{}{content}{}{expected}{}{computed}{}Len e = {e_len} | Len c = {c_len}{}",
        sep(" contents "),
        sep(" expected "),
        sep(" computed "),
        sep(" len diff "),
        sep("──────────")
    );
}

fn test_string(content: &str, expected: &str) {
    let files = &[(String::new(), content)];
    let mut location = LocationPointer::from("");
    let tokens = lex_file(content, &mut location)
        .unwrap_or_display(files)
        .unwrap();
    let node = parse_tokens(tokens).unwrap_or_display(files).unwrap();
    let computed = format!("{node}");
    print_failure(content, &computed, expected);
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
    print_failure(content, &computed, expected);
}
