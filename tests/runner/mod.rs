//! Defines the logic to test and bless code.

use std::collections::BTreeMap;
use std::env::var;
use std::fs;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};

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
_TOKENS_, "─ tokens ─"
_PARSED_, "── tree ──"
_LINEAR_, "── ssa ───"
//
SIDE, "\x1b[33m───────────────"
C0, "\x1b[0m"
EOL, "\x1b[38;5;240m¤\x1b[0m\n"
);

#[derive(Copy, Clone)]
pub enum TestScope {
    AstNoError,
    Ast,
    Ssa,
}

impl TestScope {
    fn run(self, content: &str) -> String {
        let files = &[("", content)];
        eprintln!("{SIDE}{CONTENTS}{SIDE}{C0}\n{content}");

        // lex
        eprintln!("{SIDE}{_TOKENS_}{SIDE}{C0}");
        let (tokens, err) = lex(content, "").as_displayed_errors(files);
        eprintln!("{}", display_tokens(tokens.as_ref().unwrap()));
        if !err.is_empty() {
            return err;
        }

        // parse
        eprintln!("{SIDE}{_PARSED_}{SIDE}{C0}");
        let (tree, err) = parse(tokens.unwrap()).as_displayed_errors(files);
        eprintln!("{}", tree.as_ref().unwrap());
        if !err.is_empty() && !matches!(self, Self::AstNoError) {
            eprintln!("{}", tree.as_ref().unwrap());
            return err;
        }
        if !matches!(self, Self::Ssa) {
            return tree.unwrap().to_string();
        }

        // linearise
        eprintln!("{SIDE}{_LINEAR_}{SIDE}{C0}");
        let (ssa, err) = linearise(tree.unwrap()).as_displayed_errors(files);
        let ssa_str = ssa.unwrap().display();
        eprintln!("{ssa_str}");
        if err.is_empty() { ssa_str } else { err }
    }
}

struct Tests(BTreeMap<String, String>);

impl Tests {
    fn path() -> PathBuf {
        PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("output")
    }

    fn load() -> Self {
        Self(
            fs::read(Self::path())
                .map(|content| postcard::from_bytes(&content).unwrap())
                .unwrap_or_default(),
        )
    }

    fn remove(&mut self, key: &str) {
        self.0.remove(key);
    }

    fn store(&self) {
        fs::write(Self::path(), postcard::to_allocvec(&self.0).unwrap()).unwrap();
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    fn set(&mut self, key: String, content: &str) {
        self.0
            .entry(key)
            .and_modify(|old| content.clone_into(old))
            .or_insert_with(|| content.to_owned());
    }
}

static FS_VALUES: LazyLock<Mutex<Tests>> = LazyLock::new(|| Mutex::from(Tests::load()));

pub fn test(module_name: &str, test_name: &str, content: &str, scope: TestScope) {
    let computed = scope.run(content);
    eprint!("{SIDE}{COMPUTED}{SIDE}{C0}\n{}{EOL}", computed.replace('\n', EOL));
    eprintln!("{SIDE}{EXPECTED}{SIDE}{C0}");

    let key = format!("{module_name}::{test_name}");

    let pin = var("CARGO_PIN").unwrap_or_default();
    if pin == "pin" {
        let mut lock = FS_VALUES.lock().unwrap();
        lock.set(key, &computed);
        lock.store();
        drop(lock);
        return;
    }

    let expected = if pin == "unpin" {
        let mut lock = FS_VALUES.lock().unwrap();
        lock.remove(&key);
        lock.store();
        drop(lock);
        None
    } else if !pin.is_empty() {
        panic!(
            "\x1b[31mInvalid value for CARGO_PIN environenment variable. Please stick to using `cargo test`, `cargo pin` or `cargo unpin`.\x1b[0m"
        )
    } else {
        let binding = FS_VALUES.lock().unwrap();
        let expected = binding.get(&key).map(ToString::to_string);
        drop(binding);
        expected
    };

    if let Some(exp) = expected {
        eprint!("{}{EOL}", exp.replace('\n', EOL));
        assert_eq!(exp, computed);
    } else {
        panic!(
            "\x1b[31mNo expected output provided, use `cargo bless` to use current computed output as expected test output.\x1b[0m"
        )
    }
}

/// Convenience macro to create tests with the right scope.
#[macro_export]
macro_rules! one_test {
    ($name:ident, $scope:ident, $input:expr) => {
        #[test]
        fn $name() {
            $crate::runner::test(
                module_path!(),
                stringify!($name),
                $input,
                $crate::runner::TestScope::$scope,
            )
        }
    };
}

/// Convenience macro to create ast tests.
#[macro_export]
macro_rules! ast {
    ($($name:ident: $input:expr)*) => {
        $($crate::one_test!($name, Ast, $input);)*
    };
}

/// Convenience macro to create ast tests while ignoring errors.
#[macro_export]
macro_rules! ast_no_error {
    ($($name:ident: $input:expr)*) => {
        $($crate::one_test!($name, Ast, $input);)*
    };
}

/// Convenience macro to create ssa tests.
#[macro_export]
macro_rules! ssa {
    ($($name:ident: $input:expr)*) => {
        $($crate::one_test!($name, Ssa, $input);)*
    };
}
