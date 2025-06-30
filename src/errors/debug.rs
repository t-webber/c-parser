//! Handles the debug logging.
//!
//! This is the logging that appears when activating the `debug` feature. Thanks
//! to these logging, we can see a details call stack of the different functions
//! of the parser.

#![coverage(off)]

use core::fmt;

/// Struct that handles logging of debug messages.
///
/// It implements a [`Print:custom_print`] method to display any message and
/// helper messages for messages that are often used.
pub struct Print;

impl Print {
    /// Log message with a custom message
    pub fn custom_print(msg: &str) {
        log_print(msg);
    }

    /// Logs debug message when pushing in a node
    pub fn push_in_node<T: fmt::Display, U: fmt::Display>(
        pushed: &T,
        pushed_kind: &str,
        current: &U,
    ) {
        log_print(&format!("Pushing {pushed_kind} {pushed} in node {current}"));
    }

    /// Logs debug message when pushing in a vec
    pub fn push_in_vec<T: fmt::Display, U: fmt::Display>(
        leaf: &T,
        current: &[U],
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

    /// Logs debug message when pushing a node as leaf
    pub fn push_leaf<T: fmt::Display, U: fmt::Display>(leaf: &T, current: &U, kind: &str) {
        log_print(&format!("Pushing {leaf} as leaf in {kind} {current}"));
    }

    /// Logs debug message when pushing a node as leaf, type information on the
    /// node to be pushed.
    pub fn push_leaf_in<T: fmt::Display, U: fmt::Display>(
        leaf: &T,
        leaf_kind: &str,
        current: &U,
        current_kind: &str,
    ) {
        log_print(&format!("Pushing {leaf_kind} {leaf} as leaf in {current_kind} {current}"));
    }

    /// Debug message for pushing an operator.
    pub fn push_op<T: fmt::Display, U: fmt::Display>(op: &T, current: &U, current_kind: &str) {
        log_print(&format!("Pushing op {op} in {current_kind} {current}"));
    }
}

/// Main logger function to display the debug messages with the right colour and
/// indentation.
///
/// It is not meant to be used directly, only use a wrapper provided by the
/// [`Print`] struct.
#[expect(clippy::print_stdout, reason = "goal of lint")]
fn log_print(msg: &str) {
    println!("\t\x1b[38;5;240m{msg}\x1b[0m");
}
