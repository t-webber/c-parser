use c_parser::*;

const SEP: &str = "--------------------\n";

mod blocks;
mod control_flows;
mod errors;
mod functions;
mod operators;
mod variables;

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
        parse_tokens(tokens).get_displayed_errors(files, "parser")
    } else {
        res.get_displayed_errors(files, "lexer")
    };
    assert!(
        expected == computed,
        "{SEP}Mismatch! Expected:\n!{expected}!\n!= Computed\n!{computed}!{SEP}Len e = {} | Len c = {}{SEP}",
        expected.len(),
        computed.len()
    );
}

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
