use expressions::prelude::*;

fn test_parser_on_string(input: &str, output: &str) {
    let mut location = Location::from(String::new());
    let Res {
        errors: lex_errors,
        result: tokens,
    } = lex_file(input, &mut location);
    if !lex_errors.is_empty() {
        display_errors(lex_errors, &[(String::new(), input)], "lexing");
        panic!();
    }
    let Res {
        errors: pars_errors,
        result: node,
    } = parse_tokens(tokens);
    if !pars_errors.is_empty() {
        display_errors(pars_errors, &[(String::new(), input)], "parsing");
        panic!();
    }
    assert!(output == format!("{node}"));
}

#[test]
fn parser_1() {
    test_parser_on_string(
        "a + b * c - d / e % f + g - h * i + j % k * l ^ m & n | o || p && q",
        "(((((((a + (b * c) - ((d / e) % f)) + g) - (h * i)) + ((j % k) * l)) ^ (m & n)) | o) || (p && q))",
    );
}
