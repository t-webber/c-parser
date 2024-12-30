use core::fmt::Write as _;
use std::collections::HashMap;

use super::compile::CompileError;

/// Transforms [`CompileError`] into a human-readable string
///
/// See [`Res::get_displayed_errors`](super::result::Res::get_displayed_errors)
/// for extra information and examples.
pub(super) fn display_errors(
    errors: Vec<CompileError>,
    files: &[(String, &str)],
    err_type: &str,
) -> Result<String, ()> {
    let mut files_state: HashMap<String, Vec<&str>> = HashMap::new();
    let mut res = String::new();
    for (filename, content) in files {
        files_state.insert(filename.to_owned(), content.lines().collect());
    }
    for error in errors {
        let (location, message, err_lvl, length) = error.into_values();
        let (filename, line_nb, column_nb) = location.into_values();
        let code_lines = files_state
            .get(&filename)
            .expect("Never happens: File of error doesn't exist");
        let code_line = code_lines.get(line_nb - 1).unwrap_or_else(|| {
            panic!("Never happens: given line of file that doesn't exist: {filename}:{line_nb}:{column_nb}")
        });
        writeln!(
            res,
            "{filename}:{line_nb}:{column_nb}: {err_type} {err_lvl}: {message}\n{line_nb:5} | {code_line}\n{}^{}",
            " ".repeat(8 + column_nb - 1),
            "~".repeat(length)
        )
        .map_err(|_| ())?;
    }
    Ok(res)
}
