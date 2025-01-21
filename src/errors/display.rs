//! Module to display the errors
//!
//! Implements the [`display_errors`] function that converts the
//! [`CompileError`] to a user-readable error string.

use core::fmt::Write as _;
use std::collections::HashMap;

use super::compile::CompileError;

/// Transforms [`CompileError`] into a human-readable string
///
/// See [`Res::get_displayed_errors`](super::result::Res::get_displayed_errors)
/// for extra information and examples.
///
/// # Errors
///
/// Returns an error when the writing on the string buffer fails.
pub(super) fn display_errors(
    errors: &Vec<CompileError>,
    files: &[(String, &str)],
    err_type: &str,
) -> Result<String, ()> {
    let mut files_state: HashMap<String, Vec<&str>> = HashMap::new();
    let mut res = String::new();
    for (filename, content) in files {
        files_state.insert(filename.to_owned(), content.lines().collect());
    }
    for error in errors {
        let (location, message, err_lvl) = error.get_values();
        let (filename, line_nb, column_nb, length) = location.get_values();
        let code_lines = files_state
            .get(filename)
            .expect("Never happens: File of error doesn't exist");
        let code_line = code_lines
            .get(safe_decrement(line_nb))
            .expect("Never happens: file line given doesn't exist");
        let mut too_long = false;
        let col = safe_decrement(column_nb);
        let under_spaces = " ".repeat(8usize.checked_add(col).unwrap_or_else(|| {
            too_long = true;
            col
        }));
        let under_tilde = "~".repeat(safe_decrement(length));
        writeln!(
            res,
            "{filename}:{line_nb}:{column_nb}: {err_type} {err_lvl}: {message}\n{line_nb:5} | {code_line}\n{under_spaces}^{under_tilde}"
        ).map_err(|_| ())?;
        if too_long {
            writeln!(
                res,
                "{filename}:{line_nb}:{column_nb}: format warning: This line of code exceeds the maximum size of {}. Consider refactoring your code. {line_nb:5} | {code_line}\n{under_spaces}^{under_tilde}",
                usize::MAX
            )
            .map_err(|_| ())?;
        }
    }
    Ok(res)
}

/// Decrements a value of 1
const fn safe_decrement(val: usize) -> usize {
    val.checked_sub(1)
        .expect("line, col, len are initialised at 1, then incremented")
}
