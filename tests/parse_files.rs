mod parser_files {
    use std::fs;

    use c_parser::*;

    const PREFIX: &str = "./tests/data/";

    #[expect(clippy::unwrap_used)]
    fn test_parser_on_file(file: &str) {
        let path = format!("{PREFIX}{file}.c");
        let content = fs::read_to_string(&path).unwrap();
        let files: &[(String, &str)] = &[(path.clone(), &content)];
        let mut location = Location::from(path);
        let tokens = lex_file(&content, &mut location).unwrap_or_display(files, "lexer");
        eprintln!(
            "Tokens =
    {}",
            display_tokens(&tokens)
        );
        let _node = parse_tokens(tokens).unwrap_or_display(files, "parser");
    }

    #[test]
    fn no_control_flow() {
        test_parser_on_file("no-control-flow");
    }

    // #[test] // cast not supported yet
    // fn operators() {
    //     test_parser_on_file("operators");
    // }

    // #[test] // control flows not supported yet
    // fn parser_escape() {
    //     test_parser_on_file("escape");
    // }

    // #[test] // control flows not supported yet
    // fn parser_general() {
    //     test_parser_on_file("general");
    // }
}
