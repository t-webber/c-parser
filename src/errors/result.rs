use core::{convert, ops};

use super::compile::CompileError;
use super::display::display_errors;

type PublicRes<T> = (T, Vec<CompileError>);

#[derive(Debug)]
pub struct Res<T> {
    errors: Vec<CompileError>,
    result: T,
}

impl<T> Res<T> {
    #[inline]
    pub fn unwrap_or_display(self, files: &[(String, &str)], err_type: &str) -> T {
        if self.errors.is_empty() {
            self.result
        } else {
            display_errors(self.errors, files, err_type);
            panic!(/* Fail when displaying errors */)
        }
    }
}

impl<T: Default> Res<T> {
    pub(crate) fn from_err(err: CompileError) -> Self {
        Self {
            result: T::default(),
            errors: vec![err],
        }
    }

    pub(crate) fn from_errors(errors: Vec<CompileError>) -> Self {
        Self {
            result: T::default(),
            errors,
        }
    }
}

impl<T> From<PublicRes<T>> for Res<T> {
    #[inline]
    fn from((result, errors): PublicRes<T>) -> Self {
        Self { errors, result }
    }
}

impl<T> From<T> for Res<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self {
            result: value,
            errors: vec![],
        }
    }
}

impl<T> ops::FromResidual<Vec<CompileError>> for Res<Option<T>> {
    #[inline]
    fn from_residual(residual: Vec<CompileError>) -> Self {
        Self::from_errors(residual)
    }
}

impl<T> ops::FromResidual<Result<convert::Infallible, CompileError>> for Res<Option<T>> {
    #[inline]
    fn from_residual(residual: Result<convert::Infallible, CompileError>) -> Self {
        match residual {
            Ok(_) => unreachable!(/* By definition of Infallible */),
            Err(err) => Self::from_err(err),
        }
    }
}

impl<T> ops::Try for Res<Option<T>> {
    type Output = Option<T>;
    type Residual = Vec<CompileError>;

    #[inline]
    fn branch(self) -> ops::ControlFlow<Self::Residual, Self::Output> {
        if self.errors.is_empty() {
            ops::ControlFlow::Continue(self.result)
        } else {
            ops::ControlFlow::Break(self.errors)
        }
    }

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Self::from(output)
    }
}
