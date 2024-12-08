use crate::location::Location;

#[macro_export]
macro_rules! to_error {
    ($location:expr, $($arg:tt)*) => {
        CompileError::from(($location.to_owned(), format!($($arg)*)))
    };
}

#[derive(Debug)]
pub struct CompileError {
    location: Location,
    message: String,
}

type LS = (Location, String);
impl From<LS> for CompileError {
    fn from((location, message): LS) -> Self {
        Self { message, location }
    }
}

pub type Errors = Vec<CompileError>;

pub struct Res<T> {
    pub result: T,
    pub errors: Vec<CompileError>,
}

type TV<T> = (T, Vec<CompileError>);
impl<T> From<TV<T>> for Res<T> {
    fn from((result, errors): TV<T>) -> Self {
        Self { result, errors }
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
