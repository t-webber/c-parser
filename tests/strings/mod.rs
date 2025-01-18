use c_parser::*;

const SEP: &str = "--------------------\n";

mod blocks;
mod control_flows;
mod errors;
mod functions;
mod operators;
mod variables;

fn test_string(content: &str, output: &str) {
    let files = &[(String::new(), content)];
    eprint!("{SEP}Content = {content}\n{SEP}");
    print!("a");
    let mut location = Location::from(String::new());
    let tokens = lex_file(content, &mut location).unwrap_or_display(files, "lexer");
    let node = parse_tokens(tokens).unwrap_or_display(files, "parser");
    let displayed = format!("{node}");
    print!("b");
    assert!(
        output == displayed,
        "{SEP}Mismatch! Expected:\n!{output:?}!\n!= Computed\n!{displayed:?}!\n{SEP}Len o = {} | Len d = {}\n{SEP}",
        output.len(),
        displayed.len()
    );
}

fn test_string_error(content: &str, output: &str) {
    let files = &[(String::new(), content)];
    let mut location = Location::from(String::new());
    let res = lex_file(content, &mut location);
    let displayed = if res.errors_empty() {
        let tokens = res.unwrap_or_display(files, "lexer");
        parse_tokens(tokens).get_displayed_errors(files, "parser")
    } else {
        res.get_displayed_errors(files, "lexer")
    };
    assert!(
        output == displayed,
        "{SEP}Mismatch! Expected:\n!{output}!\n!= Computed\n!{displayed}!{SEP}Len o = {} | Len d = {}{SEP}",
        output.len(),
        displayed.len()
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
