use crate::errors::location::Location;

#[derive(Debug)]
pub struct CompileError {
    err_lvl: ErrorLevel,
    length: usize,
    location: Location,
    message: String,
}

impl CompileError {
    pub fn get(self) -> (Location, String, &'static str, usize) {
        (
            self.location,
            self.message,
            self.err_lvl.repr(),
            self.length,
        )
    }

    pub fn is_error(&self) -> bool {
        self.err_lvl == ErrorLevel::Error
    }

    pub fn specify_length(&mut self, length: usize) {
        self.length = length;
    }
}

impl From<(Location, String, ErrorLevel, usize)> for CompileError {
    fn from((location, message, err_lvl, length): (Location, String, ErrorLevel, usize)) -> Self {
        Self {
            err_lvl,
            length,
            location,
            message,
        }
    }
}

impl From<(Location, String, ErrorLevel)> for CompileError {
    fn from((location, message, err_lvl): (Location, String, ErrorLevel)) -> Self {
        Self {
            message,
            length: 0,
            location,
            err_lvl,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorLevel {
    Error,
    Suggestion,
    Warning,
}

impl ErrorLevel {
    const fn repr(&self) -> &'static str {
        match self {
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Suggestion => "suggestion",
        }
    }
}
