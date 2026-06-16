//! Different unit tests to test the whole compilation chain.
//!
//! It is scoped as such to allow sharing code between test files.

#![expect(clippy::restriction, reason = "tests should fail")]

/// Tests for the lexing logic.
mod lexer;
/// Tests for the linearising logic.
mod lineariser;
/// Tests for the parsing logic.
mod parser;

/// Runner to give a convenient way of writing tests.
mod runner {
    use std::fs;

    use c_parser::{display_tokens, lex, linearise, parse};

    macro_rules! declare {
    ($($pascal:ident, $str:expr)*) => {
        $(const $pascal: &str = $str;)*
    };
}

    declare!(
    CONTENTS, " contents "
    EXPECTED, " expected "
    COMPUTED, " computed "
    LEN_DIFF, " len diff "
    ________, "──────────"
    _PARSED_, "── tree ──"
    _TOKENS_, "─ tokens ─"
    //
    SIDE, "───────────────"
    );

    macro_rules! print {
        ($prefix:expr, $content:expr) => {
            eprintln!("\x1b[33m{SIDE}{}{SIDE}\x1b[0m\n{}", $prefix, $content);
        };
        ($prefix:expr) => {
            print!($prefix, "");
        };
    }

    pub fn test(test_name: &str, content: &str, expected: &str, step: &Stop) {
        eprintln!("\x1b[32m{test_name}\x1b[0m");
        print!(CONTENTS, content);
        let computed = &step.run(content);

        if expected == computed {
            return;
        }
        fs::write("expected.txt", expected).unwrap();
        fs::write("computed.txt", computed).unwrap();
        let e_len = expected.len();
        let c_len = computed.len();
        let lens = format!("Len e = {e_len} | Len c = {c_len}");

        print!(COMPUTED, computed);
        print!(EXPECTED, expected);
        print!(LEN_DIFF, lens);
        print!(________, "");
        panic!()
    }

    pub enum Stop {
        ParsingOrSuggestion,
        LinearisingOrSuggestion,
        Parsing,
    }

    impl Stop {
        pub fn run(&self, content: &str) -> String {
            let files = &[("", content)];
            // lex
            print!(_TOKENS_);
            let (tokens, err) = lex(content, "").as_displayed_errors(files);
            if !matches!(self, Self::Parsing) && !err.is_empty() {
                return err;
            }
            eprintln!("{}", display_tokens(tokens.as_ref().unwrap()));
            // parse
            print!(_PARSED_);
            let res = parse(tokens.unwrap());
            let (ast, err) = res.as_displayed_errors(files);
            if !err.is_empty() {
                return err;
            }
            if !matches!(self, Self::LinearisingOrSuggestion) {
                return ast.unwrap().to_string();
            }
            eprintln!("{}", ast.as_ref().unwrap());
            // linearise
            let (ok, err) = linearise(ast.unwrap()).as_displayed_errors(files);
            if err.is_empty() {
                ok.unwrap().to_string()
            } else {
                err
            }
        }
    }

    /// Convenience macro to create tests for the ast.
    #[macro_export]
    macro_rules! ast {
        ($($name:ident: $input:expr => $output:expr)*) => {
            $(
                #[test]
                fn $name() {
                    $crate::runner::test(stringify!($name), $input, $output, &$crate::runner::Stop::ParsingOrSuggestion)
                }
            )*
        };
    }

    /// Convenience macro to create tests for the ast.
    #[macro_export]
    macro_rules! ast_no_error {
        ($($name:ident: $input:expr => $output:expr)*) => {
            $(
                #[test]
                fn $name() {
                    $crate::runner::test(stringify!($name), $input, $output, &$crate::runner::Stop::Parsing)
                }
            )*
        };
    }

    /// Convenience macro to create tests for the ssa.
    #[macro_export]
    macro_rules! ssa {
        ($($name:ident: $input:expr => $output:expr)*) => {
            $(
                #[test]
                fn $name() {
                    $crate::runner::test(stringify!($name), $input, $output, &$crate::runner::Stop::LinearisingOrSuggestion)
                }
            )*
        };
    }
}
