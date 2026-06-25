//! Module to display the errors
//!
//! Implements the [`display_errors`] function that converts the
//! [`CompileError`] to a user-readable error string.

use core::fmt::Write as _;
use std::collections::HashMap;

use super::api::ErrorLocation;
use super::compile::CompileError;
use crate::errors::compile::CompileErrorList;
use crate::utils::{u32_to_usize, usize_to_u32};

/// Wrapper for [`writeln!`] to customise error handling
macro_rules! writeln_bool {
    ($buf:ident $(,$arg:expr)*) => {
        writeln!($buf, $($arg,)*).is_ok()
    };
}

/// Data for an error that holds in one line
struct OneLineError<'error> {
    /// Line of code in which the error occurred
    code_line: &'error str,
    /// Column of the start of the error
    col: u32,
    /// Level of the error to be displayed
    err_lvl: &'error str,
    /// File of the error
    file_name: &'error str,
    /// Length of the error on the line
    len: u32,
    /// Line of the error
    line: u32,
}

/// Returns a precise line of code
///
/// This takes as input the list of the content of all the files, the file
/// wanted and the line number within this file and returns the line of code
/// described by these two parameters.
fn as_code_line<'content, 'name>(
    file_contents: &HashMap<u32, (&'name str, &'content str)>,
    file_name: u32,
    line: u32,
) -> (&'name str, &'content str) {
    let (name, content) = file_contents.get(&file_name).expect("file of error exists");
    (
        name,
        content
            .lines()
            .nth(usize::try_from(safe_decrement(line)).expect("never fails"))
            .expect("line of error exists"),
    )
}

/// Display one error
///
/// This is wrapper for [`display_error`]. Please refer to its documentation.
fn display_error(
    buf: &mut String,
    error: &CompileError,
    file_contents: &HashMap<u32, (&str, &str)>,
) -> bool {
    let (location, msg, err_lvl) = error.as_values();
    match location {
        ErrorLocation::Char(file_name, line, col) => display_one_line_error(
            buf,
            &OneLineError {
                col,
                code_line: as_code_line(file_contents, file_name, line).1,
                err_lvl: &err_lvl,
                file_name: as_code_line(file_contents, file_name, line).0,
                len: 1,
                line,
            },
            msg,
            true,
        ),
        ErrorLocation::Token(file_name, line, col, len) => display_one_line_error(
            buf,
            &OneLineError {
                col,
                code_line: as_code_line(file_contents, file_name, line).1,
                err_lvl: &err_lvl,
                file_name: as_code_line(file_contents, file_name, line).0,
                len,
                line,
            },
            msg,
            true,
        ),
        ErrorLocation::Block(file_name, start_line, start_col, end_line, end_col) =>
            if let start_code_line = as_code_line(file_contents, file_name, start_line)
                && let name = start_code_line.0
                && writeln_bool!(buf)
                && display_prefix(buf, name, start_line, start_col, msg, &err_lvl)
                && {
                    display_one_line_error(
                        buf,
                        &OneLineError {
                            col: start_col,
                            code_line: start_code_line.1,
                            err_lvl: &err_lvl,
                            file_name: name,
                            len: usize_to_u32(
                                start_code_line
                                    .1
                                    .len()
                                    .checked_sub(u32_to_usize(safe_decrement(start_col)))
                                    .expect("col <= len"),
                            ),
                            line: start_line,
                        },
                        "Multi-line error occurred. Starts here...",
                        true,
                    )
                }
                && display_one_line_error(
                    buf,
                    &OneLineError {
                        col: end_col,
                        code_line: as_code_line(file_contents, file_name, end_line).1,
                        err_lvl: &err_lvl,
                        file_name: name,
                        len: 1,
                        line: end_line,
                    },
                    "...and ends here.",
                    true,
                )
                && writeln_bool!(buf)
            {
                true
            } else {
                false
            },

        ErrorLocation::None => unreachable!("never built"),
    }
}

/// Transforms [`CompileError`] into a human-readable string
///
/// See [`Res::as_displayed_errors`](super::result::Res::as_displayed_errors)
/// for extra information and examples.
///
/// # Errors
///
/// Returns an error when the writing on the string buffer fails.
#[coverage(off)]
pub(super) fn display_errors(
    errors: &CompileErrorList,
    files: &[(u32, &str, &str)],
) -> Result<String, ()> {
    let mut file_contents: HashMap<u32, (&str, &str)> = HashMap::new();
    let mut buf = String::new();
    for (id, filename, content) in files {
        file_contents.insert(*id, (filename, content));
    }
    for error in &errors.0 {
        if !display_error(&mut buf, error, &file_contents) {
            return Err(());
        }
    }
    Ok(buf)
}

/// Display an error that fits in one line.
///
/// This is wrapper for [`display_error`]. Please refer to its documentation.
fn display_one_line_error(
    buf: &mut String,
    err: &OneLineError<'_>,
    msg: &str,
    squiggles: bool,
) -> bool {
    let line = err.line;
    let file_name = err.file_name;
    display_prefix(buf, file_name, line, err.col, msg, err.err_lvl)
        && display_snippet(buf, line, err.code_line)
        && (!squiggles || display_squiggles(buf, err))
}

/// Display the prefix of the error
///
/// The prefix of a displayed error is the location, followed by the error type
/// and level, followed by the message. After the
/// prefix, the only pieces remaining are items such as
///
/// - a snippet of the line of code in which the error occurred;
/// - squiggles underneath the line of code;
/// - some information about the end of the error for errors on multiple lines.
fn display_prefix(
    buf: &mut String,
    file_name: &str,
    line: u32,
    col: u32,
    msg: &str,
    err_lvl: &str,
) -> bool {
    writeln_bool!(buf, "{file_name}:{line}:{col}: {err_lvl}: {msg}")
}

/// Display the code snippet of the error
///
/// This displays the line of code whence the error originates.
fn display_snippet(buf: &mut String, line: u32, code_line: &str) -> bool {
    writeln_bool!(buf, "{line:5} | {code_line}")
}

/// Display the squiggles under the code snippet to add visual prop
fn display_squiggles(buf: &mut String, err: &OneLineError<'_>) -> bool {
    let mut too_long = false;
    let under_spaces = " ".repeat(u32_to_usize(
        8u32.checked_add(safe_decrement(err.col))
            .unwrap_or_else(|| {
                too_long = true;
                err.col
            }),
    ));
    (!too_long
        || (display_one_line_error(
            buf,
            err,
            "This line of code exceeds the maximum size of {}. The display of the next error might be erroneous. Consider refactoring your code.",
            false,
        )))
        && {
            writeln_bool!(
                buf,
                "{under_spaces}^{}",
                "~".repeat(u32_to_usize(safe_decrement(err.len)))
            )
        }
}

/// Decrements a value of 1
const fn safe_decrement(val: u32) -> u32 {
    val.checked_sub(1)
        .expect("line, col, len are initialised at 1, then incremented")
}
