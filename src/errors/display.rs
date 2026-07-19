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
#[derive(Debug)]
struct OneLineError<'disp> {
    /// Line of code in which the error occurred
    code_line: &'disp str,
    /// Column of the start of the error
    col1: u32,
    /// Column of the start of the error
    col2: u32,
    /// Level of the error to be displayed
    err_lvl: &'disp str,
    /// File of the error
    file_name: &'disp str,
    /// Length of the error on the line
    len1: u32,
    /// Length of the error on the line
    len2: u32,
    /// Line of the error
    line: u32,
}

impl OneLineError<'_> {
    /// Displays the error, with prefix, snippet and squiggles
    fn disp(&self, buf: &mut String, msg: &str) -> bool {
        display_prefix(buf, self.file_name, self.line, self.col1, msg, self.err_lvl)
            && display_snippet(buf, self.line, self.code_line)
            && display_squiggles(buf, self)
    }
}

/// Precise line of code
struct CodeLine<'disp>(&'disp str, &'disp str);

impl<'disp> CodeLine<'disp> {
    /// Converts the code line to an error
    const fn err(
        &self,
        col1: u32,
        len1: u32,
        col2: u32,
        len2: u32,
        line: u32,
        err_lvl: &'disp str,
    ) -> OneLineError<'disp> {
        OneLineError { col1, col2, code_line: self.1, err_lvl, file_name: self.0, len1, len2, line }
    }

    /// Returns a precise line of code
    ///
    /// This takes as input the list of the content of all the files, the file
    /// wanted and the line number within this file and returns the line of code
    /// described by these two parameters.
    fn new(
        file_contents: &HashMap<u32, (&'disp str, &'disp str)>,
        file_name: u32,
        line: u32,
    ) -> Self {
        let (name, content) = file_contents.get(&file_name).expect("file of error exists");
        Self(
            name,
            content
                .lines()
                .nth(usize::try_from(safe_decrement(line)).expect("never fails"))
                .expect("line of error exists"),
        )
    }
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
        ErrorLocation::Char(file, line, col) => CodeLine::new(file_contents, file, line)
            .err(col, 1, col, 1, line, &err_lvl)
            .disp(buf, msg),
        ErrorLocation::Token(file, line, col, len) => CodeLine::new(file_contents, file, line)
            .err(col, len, col, len, line, &err_lvl)
            .disp(buf, msg),
        ErrorLocation::Block(file, start_line, start_col, end_line, end_col) =>
            if let start_code_line = CodeLine::new(file_contents, file, start_line)
                && let name = start_code_line.0
                && writeln_bool!(buf)
                && display_prefix(buf, name, start_line, start_col, msg, &err_lvl)
                && let start_len = usize_to_u32(
                    start_code_line
                        .1
                        .len()
                        .checked_sub(u32_to_usize(safe_decrement(start_col)))
                        .expect("col <= len"),
                )
                && start_code_line
                    .err(start_col, start_len, start_col, start_len, start_line, &err_lvl)
                    .disp(buf, "Multi-line error occurred. Starts here...")
                && CodeLine::new(file_contents, file, end_line)
                    .err(end_col, 1, end_col, 1, end_line, &err_lvl)
                    .disp(buf, "...and ends here.")
                && writeln_bool!(buf)
            {
                true
            } else {
                false
            },
        ErrorLocation::None => unreachable!("never built"),
        ErrorLocation::TwoTokens(file, line1, col1, len1, line2, col2, len2) =>
            if line1 == line2 {
                CodeLine::new(file_contents, file, line1)
                    .err(col1, len1, col2, len2, line1, &err_lvl)
                    .disp(buf, msg)
            } else {
                CodeLine::new(file_contents, file, line1)
                    .err(col1, len1, col1, len1, line1, &err_lvl)
                    .disp(buf, msg)
                    && CodeLine::new(file_contents, file, line1)
                        .err(col2, len2, col2, len2, line1, &err_lvl)
                        .disp(buf, msg)
            },
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
    let spaces = " ".repeat(u32_to_usize(
        8u32.checked_add(safe_decrement(err.col1))
            .unwrap_or(err.col1),
    ));
    let squiggles = "~".repeat(u32_to_usize(safe_decrement(err.len1)));
    write!(buf, "{spaces}^{squiggles}").is_ok()
        && if err.col1 == err.col2 {
            writeln_bool!(buf, "")
        } else {
            let between = " "
                .repeat(u32_to_usize(err.col2.saturating_sub(err.col1).saturating_sub(err.len1)));
            let second = "~".repeat(u32_to_usize(safe_decrement(err.len2)));
            writeln_bool!(buf, "{between}^{second}")
        }
}

/// Decrements a value of 1
const fn safe_decrement(val: u32) -> u32 {
    val.checked_sub(1)
        .expect("line, col, len are initialised at 1, then incremented")
}
