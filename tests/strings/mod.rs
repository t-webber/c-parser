use c_parser::*;

const SEP: &str = "\n--------------------\n";

mod control_flows;
mod errors;
mod operators;

fn test_string(content: &str, output: &str) {
    let files = &[(String::new(), content)];
    let mut location = Location::from(String::new());
    eprint!("{SEP}Content = {content}{SEP}");
    let tokens = lex_file(content, &mut location).unwrap_or_display(files, "lexer");
    eprint!("{SEP}Tokens = {}{SEP}", display_tokens(&tokens));
    let node = parse_tokens(tokens).unwrap_or_display(files, "parser");
    let displayed = format!("{node}");
    assert!(
        output == displayed,
        "{SEP}Mismatch! Expected:\n!{output:?}!\n!= Computed\n!{displayed:?}!{SEP}Len o = {} | Len d = {}{SEP}",
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
            mod parser_string {
                $(
                    #[test]
                    fn $name() {
                        super::super::test_string($input, $output)
                    }
                )*
            }

        };
}
