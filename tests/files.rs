mod files {
    use std::fs;

    use c_parser::*;

    const PREFIX: &str = "./tests/data/";

    fn test_file(file: &str, parser_works: bool) {
        let path = format!("{PREFIX}{file}.i");
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|err| unreachable!("Failed to read file {path}:\n{err}"));
        let mut location = LocationPointer::from(&path);
        let files: &[(String, &str)] = &[(path, &content)];
        let tokens = lex_file(&content, &mut location).unwrap_or_display(files, "lexer");
        if parser_works {
            let _tree = parse_tokens(tokens).unwrap_or_display(files, "parser");
        }
    }

    #[test]
    fn escape() {
        test_file("escape", true);
    }

    #[test]
    fn general() {
        test_file("general", false);
    }

    #[test]
    fn operators() {
        test_file("operators", true);
    }

    #[test]
    fn no_control_flow() {
        test_file("no-control-flow", true);
    }

    #[test]
    fn control_flows() {
        test_file("control-flows", false);
    }
}
