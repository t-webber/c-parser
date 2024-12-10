use super::compile::CompileError;
use std::collections::HashMap;

pub fn display_errors(errors: Vec<CompileError>, files: &[(String, &str)]) {
    let mut files_status: HashMap<String, Vec<&str>> = HashMap::new();
    for (filename, content) in files {
        files_status.insert(filename.to_owned(), content.lines().collect());
    }
    for error in errors {
        let (location, message) = error.get();
        let (filename, line_nb, column_nb) = location.get();
        let code_lines = files_status
            .get(&filename)
            .expect("Never happens: File of error doesn't exist");
        let code_line = code_lines
            .get(line_nb - 1)
            .expect("Never happens: given line of file that doesn't exist");
        eprintln!("\n{filename}:{line_nb}:{column_nb}: error: {message}");
        eprintln!("{line_nb:5} | {code_line}");
        eprintln!("{}^", " ".repeat(8 + column_nb - 1));
    }
}
