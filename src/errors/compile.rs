use crate::errors::location::Location;

#[macro_export]
macro_rules! to_error {
    ($location:expr, $($arg:tt)*) => {
        $crate::errors::compile::CompileError::from(($location.to_owned(), format!($($arg)*), $crate::errors::compile::ErrorLevel::Error))
    };
}

#[macro_export]
macro_rules! to_warning {
    ($location:expr, $($arg:tt)*) => {
        $crate::errors::compile::CompileError::from(($location.to_owned(), format!($($arg)*), $crate::errors::compile::ErrorLevel::Warning))
    };
}
#[macro_export]
macro_rules! to_suggestion {
    ($location:expr, $($arg:tt)*) => {
        $crate::errors::compile::CompileError::from(($location.to_owned(), format!($($arg)*), $crate::errors::compile::ErrorLevel::Warning))
    };
}

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
    Warning,
    Error,
    Suggestion,
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

pub struct Res<T> {
    pub errors: Vec<CompileError>,
    pub result: T,
}

pub type FailRes<T> = Result<T, CompileError>;

impl<T> From<(T, Vec<CompileError>)> for Res<T> {
    fn from((result, errors): (T, Vec<CompileError>)) -> Self {
        Self { errors, result }
    }
}

impl<T> From<T> for Res<T> {
    fn from(value: T) -> Self {
        Self {
            result: value,
            errors: vec![],
        }
    }
}
