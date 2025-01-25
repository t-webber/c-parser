//! Module to display the errors
//!
//! Implements the [`display_errors`] function that converts the
//! [`CompileError`] to a user-readable error string.

use core::fmt::Write as _;
use std::collections::HashMap;

use super::api::ErrorLocation;
use super::compile::CompileError;

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
    col: usize,
    /// Level of the error to be displayed
    err_lvl: &'error str,
    /// File of the error
    file_name: &'error str,
    /// Length of the error on the line
    len: usize,
    /// Line of the error
    line: usize,
}

/// Returns a precise line of code
///
/// This takes as input the list of the content of all the files, the file
/// wanted and the line number within this file and returns the line of code
/// described by these two parameters.
fn as_code_line<'idk>(
    file_contents: &HashMap<String, Vec<&'idk str>>,
    file_name: &str,
    line: usize,
) -> &'idk str {
    file_contents
        .get(file_name)
        .expect("file of error exists")
        .get(safe_decrement(line))
        .expect("line of error exists")
}

/// Display one error
///
/// This is wrapper for [`display_error`]. Please refer to its documentation.
fn display_error(
    buf: &mut String,
    error: &CompileError,
    err_type: &str,
    file_contents: &HashMap<String, Vec<&str>>,
) -> bool {
    let (location, msg, err_lvl) = error.as_values();
    match location {
        ErrorLocation::Char(file_name, line, col) => display_one_line_error(
            buf,
            err_type,
            &OneLineError {
                col: *col,
                code_line: as_code_line(file_contents, file_name, *line),
                err_lvl: &err_lvl,
                file_name,
                len: 1,
                line: *line,
            },
            msg,
            true,
        ),
        ErrorLocation::Token(file_name, line, col, len) => display_one_line_error(
            buf,
            err_type,
            &OneLineError {
                col: *col,
                code_line: as_code_line(file_contents, file_name, *line),
                err_lvl: &err_lvl,
                file_name,
                len: *len,
                line: *line,
            },
            msg,
            true,
        ),
        ErrorLocation::Block(file_name, start_line, start_col, end_line, end_col) => {
            writeln_bool!(buf)
                && display_prefix(
                    buf,
                    file_name,
                    *start_line,
                    *start_col,
                    err_type,
                    msg,
                    &err_lvl,
                )
                && {
                    let start_code_line = as_code_line(file_contents, file_name, *start_line);
                    display_one_line_error(
                        buf,
                        err_type,
                        &OneLineError {
                            col: *start_col,
                            code_line: start_code_line,
                            err_lvl: &err_lvl,
                            file_name,
                            len: start_code_line
                                .len()
                                .checked_sub(safe_decrement(*start_col))
                                .expect("col <= len"),
                            line: *start_line,
                        },
                        "Multi-line error occurred. Starts here...",
                        true,
                    )
                }
                && display_one_line_error(
                    buf,
                    err_type,
                    &OneLineError {
                        col: *end_col,
                        code_line: as_code_line(file_contents, file_name, *end_line),
                        err_lvl: &err_lvl,
                        file_name,
                        len: 1,
                        line: *end_line,
                    },
                    "...and ends here.",
                    true,
                )
                && writeln_bool!(buf)
        }
        ErrorLocation::None => panic!("never built"),
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
    errors: &Vec<CompileError>,
    files: &[(String, &str)],
    err_type: &str,
) -> Result<String, ()> {
    let mut file_contents: HashMap<String, Vec<&str>> = HashMap::new();
    let mut buf = String::new();
    for (filename, content) in files {
        file_contents.insert(filename.to_owned(), content.lines().collect());
    }
    for error in errors {
        if !display_error(&mut buf, error, err_type, &file_contents) {
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
    err_type: &str,
    err: &OneLineError<'_>,
    msg: &str,
    squiggles: bool,
) -> bool {
    let line = err.line;
    let file_name = err.file_name;
    display_prefix(buf, file_name, line, err.col, err_type, msg, err.err_lvl)
        && display_snippet(buf, line, err.code_line)
        && (!squiggles || display_squiggles(buf, err, err_type))
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
    line: usize,
    col: usize,
    err_type: &str,
    msg: &str,
    err_lvl: &str,
) -> bool {
    writeln_bool!(buf, "{file_name}:{line}:{col}: {err_type} {err_lvl}: {msg}")
}

/// Display the code snippet of the error
///
/// This displays the line of code whence the error originates.
fn display_snippet(buf: &mut String, line: usize, code_line: &str) -> bool {
    writeln_bool!(buf, "{line:5} | {code_line}")
}

/// Display the squiggles under the code snippet to add visual prop
fn display_squiggles(buf: &mut String, err: &OneLineError<'_>, err_type: &str) -> bool {
    let mut too_long = false;
    let under_spaces = " ".repeat(8usize.checked_add(safe_decrement(err.col)).unwrap_or_else(
        || {
            too_long = true;
            err.col
        },
    ));
    (!too_long
        || (display_one_line_error(
            buf,
            err_type,
            err,
            "This line of code exceeds the maximum size of {}. The display of the next error might be erroneous. Consider refactoring your code.",
            false,
        )))
        && {
            writeln_bool!(
                buf,
                "{under_spaces}^{}",
                "~".repeat(safe_decrement(err.len))
            )
        }
}

/// Decrements a value of 1
const fn safe_decrement(val: usize) -> usize {
    val.checked_sub(1)
        .expect("line, col, len are initialised at 1, then incremented")
}
