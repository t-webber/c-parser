use std::collections::HashMap;

use super::compile::CompileError;

#[inline]
pub fn display_errors(errors: Vec<CompileError>, files: &[(String, &str)], err_type: &str) {
    let mut files_status: HashMap<String, Vec<&str>> = HashMap::new();
    for (filename, content) in files {
        files_status.insert(filename.to_owned(), content.lines().collect());
    }
    for error in errors {
        let (location, message, err_lvl, length) = error.get();
        let (filename, line_nb, column_nb) = location.into_values();
        let code_lines = files_status
            .get(&filename)
            .expect("Never happens: File of error doesn't exist");
        let code_line = code_lines.get(line_nb - 1).unwrap_or_else(|| {
            panic!("Never happens: given line of file that doesn't exist: {filename}:{line_nb}:{column_nb}")
        });
        eprintln!("\n{filename}:{line_nb}:{column_nb}: {err_type} {err_lvl}: {message}");
        eprintln!("{line_nb:5} | {code_line}");
        eprintln!("{}^{}", " ".repeat(8 + column_nb - 1), "~".repeat(length));
    }
}
