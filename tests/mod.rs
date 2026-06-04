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
            eprintln!("{SIDE}{}{SIDE}{}", $prefix, $content);
        };
    }

    pub fn test(content: &str, expected: &str, step: &CompilationStep) {
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

        print!(EXPECTED, expected);
        print!(COMPUTED, computed);
        print!(LEN_DIFF, lens);
        print!(________, "");
    }

    #[expect(dead_code)]
    pub enum CompilationStep {
        Lexing,
        Parsing,
        Linearising,
    }

    impl CompilationStep {
        fn run(&self, content: &str) -> String {
            let files = &[("", content)];
            let res = lex(content, "");
            if matches!(self, Self::Lexing) {
                let (ok, err) = res.as_displayed_errors(files);
                return if err.is_empty() {
                    display_tokens(&ok.unwrap())
                } else {
                    err
                };
            }
            let res = res.stop_at_suggestion().and_then(|tokens| {
                print!(_TOKENS_, display_tokens(&tokens));
                parse(tokens)
            });
            if matches!(self, Self::Parsing) {
                let (ok, err) = res.as_displayed_errors(files);
                return if err.is_empty() {
                    ok.unwrap().to_string()
                } else {
                    err
                };
            }
            let (ok, err) = res
                .stop_at_suggestion()
                .map(|ast| {
                    print!(_PARSED_, ast);
                    linearise(ast)
                })
                .as_displayed_errors(files);
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
                    $crate::runner::test($input, $output, &$crate::runner::CompilationStep::Parsing)
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
                    $crate::runner::test($input, $output, &$crate::runner::CompilationStep::Linearising)
                }
            )*
        };
    }
}
