//! Test the code directly on files.

#[expect(clippy::inline_modules, reason = "test names")]
#[expect(clippy::tests_outside_test_module, reason = "this is a test module")]
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

    macro_rules! tst {
        ($($name:ident: $parse:ident,)*) => {
            $(
                #[test]
                fn $name() {
                    test_file(stringify!($name), $parse)
                }
            )*
        };
    }

    tst!(
        escape:true,
        general:false,
        operators:true,
        no_control_flow:true,
        control_flows:false,
    );
}
