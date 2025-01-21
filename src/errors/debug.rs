#![coverage(off)]

use core::fmt;

pub struct Print;

impl Print {
    pub fn push_leaf<T: fmt::Display, U: fmt::Display>(leaf: &T, current: &U, kind: &str) {
        log_print(&format!("Pushing {leaf} as leaf in {kind} {current}"));
    }

    pub fn push_leaf_in<T: fmt::Display, U: fmt::Display>(
        leaf: &T,
        leaf_kind: &str,
        current: &U,
        current_kind: &str,
    ) {
        log_print(&format!(
            "Pushing {leaf_kind} {leaf} as leaf in {current_kind} {current}"
        ));
    }

    pub fn push_in_vec<T: fmt::Display, U: fmt::Display>(
        leaf: &T,
        current: &Vec<U>,
        current_kind: &str,
    ) {
        log_print(&format!(
            "Pushing {leaf} as leaf in {current_kind} [{}]",
            current
                .iter()
                .map(|node| format!("{node}"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    pub fn push_op<T: fmt::Display, U: fmt::Display>(op: &T, current: &U, current_kind: &str) {
        log_print(&format!("Pushing op {op} in {current_kind} {current}"));
    }

    pub fn push_in_node<T: fmt::Display, U: fmt::Display>(
        pushed: &T,
        pushed_kind: &str,
        current: &U,
    ) {
        log_print(&format!("Pushing {pushed_kind} {pushed} in node {current}"));
    }

    pub fn custom_print(msg: &str) {
        log_print(msg);
    }
}

fn log_print(msg: &str) {
    println!("\t\x1b[38;5;240m{msg}\x1b[0m");
}
