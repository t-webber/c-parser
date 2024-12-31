mod files {
    use std::fs;

    use c_parser::*;

    const PREFIX: &str = "./tests/data/";

    #[expect(clippy::unwrap_used)]
    fn test_file(file: &str, parser_works: bool) {
        let path = format!("{PREFIX}{file}.c");
        let content = fs::read_to_string(&path).unwrap();
        let mut location = Location::from(path.clone());
        let files: &[(String, &str)] = &[(path, &content)];
        let tokens = lex_file(&content, &mut location).unwrap_or_display(files, "lexer");
        if parser_works {
            let _tree = parse_tokens(tokens).unwrap_or_display(files, "parser");
        }
    }

    #[test]
    fn escape() {
        test_file("escape", false);
    }

    #[test]
    fn general() {
        test_file("general", false);
    }

    #[test]
    fn operators() {
        test_file("general", false);
    }

    #[test]
    fn no_control_flow() {
        test_file("no-control-flow", true);
    }
}