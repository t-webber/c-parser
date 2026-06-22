//! Defines the logic to test and bless code.

#![allow(clippy::non_ascii_literal, reason = "visual alignment")]
#![allow(clippy::panic, reason = "test")]

pub mod macros;
pub mod run;
pub mod store;

use std::env::var;
use std::sync::{LazyLock, Mutex};

use crate::runner::run::TestScope;
use crate::runner::store::Tests;

macro_rules! declare {
        ($($pascal:ident, $str:expr)*) => {
            $(const $pascal: &str = $str;)*
        };
    }

declare!(
CONTENTS, " contents "
EXPECTED, " expected "
COMPUTED, " computed "
__DIFF__, "── diff ──"
________, "──────────"
_TOKENS_, "─ tokens ─"
_PARSED_, "── tree ──"
_LINEAR_, "── ssa ───"
//
SIDE, "\x1b[33m───────────────"
C0, "\x1b[0m"
EOL, "\x1b[38;5;240m¤\x1b[0m\n"
);

static FS_VALUES: LazyLock<Mutex<Tests>> = LazyLock::new(|| Mutex::from(Tests::load()));

pub fn test(module_name: &str, test_name: &str, content: &str, scope: TestScope) {
    let computed = scope.run(content);
    eprint!("{SIDE}{COMPUTED}{SIDE}{C0}\n{}{EOL}", computed.replace('\n', EOL));

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
            "\x1b[31mInvalid value for CARGO_PIN environenment variable: {pin}. Please stick to using `cargo test`, `cargo pin` or `cargo unpin`.\x1b[0m"
        )
    } else {
        let binding = FS_VALUES.lock().unwrap();
        let expected = binding.get(&key).map(str::to_owned);
        drop(binding);
        expected
    };

    let Some(exp) = expected else {
        let msg = "\x1b[31mNo expected output provided, use `cargo bless` to use current computed output as expected test output.\x1b[0m";

        if cfg!(feature = "no_test_fail") {
            eprintln!("{msg}");
            return;
        }
        panic!("{msg}");
    };

    eprintln!("{SIDE}{EXPECTED}{SIDE}{C0}");
    eprint!("{}{EOL}", exp.replace('\n', EOL));
    if exp == computed {
        return;
    }

    eprintln!("{SIDE}{__DIFF__}{SIDE}{C0}");
    let mut e_lines = exp.lines();
    let mut c_lines = computed.lines();

    loop {
        match (e_lines.next(), c_lines.next()) {
            (Some(el), Some(cl)) if el == cl => eprintln!("{el}{C0}"),
            (None, None) =>
                if cfg!(feature = "no_test_fail") {
                    return;
                } else {
                    panic!()
                },
            (Some(el), None) => eprintln!("\x1b[32me>{el}{C0}"),
            (None, Some(cl)) => eprintln!("\x1b[31mc<{cl}{C0}"),
            (Some(el), Some(cl)) => {
                eprintln!("\x1b[31mc<{cl}{C0}");
                eprintln!("\x1b[32me>{el}{C0}");
            }
        }
    }
}
