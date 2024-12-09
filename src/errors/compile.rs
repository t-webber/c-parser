use crate::errors::location::Location;

#[macro_export]
macro_rules! to_error {
    ($location:expr, $($arg:tt)*) => {
        $crate::errors::compile::CompileError::from(($location.to_owned(), format!($($arg)*)))
    };
}

#[derive(Debug)]
pub struct CompileError {
    location: Location,
    message: String,
}

impl CompileError {
    pub fn get(self) -> (Location, String) {
        (self.location, self.message)
    }
}

impl From<(Location, String)> for CompileError {
    fn from((location, message): (Location, String)) -> Self {
        Self { location, message }
    }
}

pub type Errors = Vec<CompileError>;

pub struct Res<T> {
    pub errors: Vec<CompileError>,
    pub result: T,
}

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
