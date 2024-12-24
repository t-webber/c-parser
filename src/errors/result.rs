use core::{convert, ops};

use super::compile::CompileError;
use crate::prelude::display_errors;

pub struct Res<T> {
    errors: Vec<CompileError>,
    result: T,
}

type PublicRes<T> = (T, Vec<CompileError>);

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

impl<T> Res<T> {
    #[inline]
    pub fn unwrap_or_display(self, files: &[(String, &str)], err_type: &str) -> T {
        if self.errors.is_empty() {
            self.result
        } else {
            display_errors(self.errors, files, err_type);
            panic!()
        }
    }

    pub(crate) fn edit_errors<F: FnMut(&mut CompileError)>(&mut self, edit: F) {
        self.errors.iter_mut().for_each(edit);
    }

    pub(crate) fn extend<U>(&mut self, other: Res<U>) -> U {
        self.errors.extend(other.errors);
        other.result
    }
}

impl<T> Res<Option<T>> {
    pub(crate) const fn from_errors(errors: Vec<CompileError>) -> Self {
        Self {
            result: None,
            errors,
        }
    }

    pub(crate) fn into_value(self) -> (Option<T>, Vec<CompileError>) {
        (self.result, self.errors)
    }
}

impl<T: Default> Res<T> {
    pub(crate) fn from_err(err: CompileError) -> Self {
        Self {
            result: T::default(),
            errors: vec![err],
        }
    }
}

impl<T> ops::FromResidual<Vec<CompileError>> for Res<Option<T>> {
    #[inline]
    fn from_residual(residual: Vec<CompileError>) -> Self {
        Self::from_errors(residual)
    }
}

impl<T> ops::Try for Res<Option<T>> {
    type Output = Option<T>;
    type Residual = Vec<CompileError>;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Self::from(output)
    }

    #[inline]
    fn branch(self) -> ops::ControlFlow<Self::Residual, Self::Output> {
        if self.errors.is_empty() {
            ops::ControlFlow::Continue(self.result)
        } else {
            ops::ControlFlow::Break(self.errors)
        }
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
