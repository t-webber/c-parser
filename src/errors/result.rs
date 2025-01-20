//! Module to store the errors while still returning a result.
//!
//! This crate implements the [`Res`] struct and its methods.

extern crate alloc;
use alloc::vec;
use core::{convert, fmt, ops};

use super::compile::CompileError;
use super::display::display_errors;

/// [`Result`] alias for [`CompileError`]
pub type CompileRes<T> = Result<T, CompileError>;

/// Struct to store the errors, whilst still having the desired value.
///
/// This struct is meant as a [`Result`], but were it is possible to
/// have a value and some errors at the same time. It is for example the case
/// for warnings and suggestions (cf.
/// [`CompileError`] for more information), that must be stored, and at the
/// same time, the compiler continues to work.
#[derive(Debug)]
pub struct Res<T> {
    /// The errors that occurred
    errors: Vec<CompileError>,
    /// The desired result
    result: T,
}

impl<T> Res<T> {
    /// Adds an error to a current [`Res`]
    pub(crate) fn add_err(self, error: Option<CompileError>) -> Self {
        let mut mutable = self;
        if let Some(err) = error {
            mutable.errors.push(err);
        }
        mutable
    }

    /// Checks if the ``errors`` field is empty
    ///
    /// # Examples
    ///
    /// ```
    /// assert!(c_parser::Res::from(0).errors_empty() == true);
    /// ```
    ///
    /// ```ignore
    /// assert!(Res::from_errs(vec![]).errors_empty() == true);
    /// ```
    #[inline]
    pub const fn errors_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns all the errors in a user-readable format.
    ///
    /// # Returns
    ///
    /// A [`String`] containing all the errors, displayed in a user-readable
    /// format, with a clickable location, and an explanation message.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs;
    ///
    /// use c_parser::{Location, lex_file};
    ///
    /// let content = "int m@in() { }";
    /// let res = lex_file(&content, &mut Location::from("filename.c"));
    /// let errors = res.get_displayed_errors(&[("filename.c".to_owned(), content)], "lexer");
    /// let expected = "filename.c:1:6: lexer error: Character '@' not supported.
    ///     1 | int m@in() { }
    ///              ^
    /// ";
    ///
    /// assert!(errors == expected, "!{errors}!\n!{expected}!");
    /// ```
    ///
    /// # Panics
    ///
    /// If there are too many errors, a buffer overflow occurs
    #[inline]
    pub fn get_displayed_errors(&self, files: &[(String, &str)], err_type: &str) -> String {
        display_errors(&self.errors, files, err_type)
            .expect("Buffer overflow, failed to fetch errors")
    }

    /// Checks if the [`Res`] contains critical failures.
    pub(crate) fn has_failures(&self) -> bool {
        self.errors.iter().any(CompileError::is_failure)
    }

    /// Returns the errors of a [`Res`]
    ///
    /// This drops the `result`.
    pub(crate) fn into_errors(self) -> Vec<CompileError> {
        self.errors
    }

    /// Stores the errors with a function and returns the value
    pub(crate) fn store_errors<F: FnMut(CompileError)>(self, store: &mut F) -> T {
        for err in self.errors {
            store(err);
        }
        self.result
    }

    /// Prints all the errors to the user.
    ///
    /// # Returns
    ///
    /// The value of the [`Res`] if there aren't any errors of level `Failure`.
    ///
    /// # Panics
    ///
    /// If there is at least one error of level `Failure`.
    #[inline]
    #[expect(clippy::print_stderr)]
    pub fn unwrap_or_display(self, files: &[(String, &str)], err_type: &str) -> T {
        eprint!("{}", self.get_displayed_errors(files, err_type));
        if self.has_failures() {
            panic!(/* Fail when displaying errors */)
        } else {
            self.result
        }
    }
}

impl<T: Default> From<CompileError> for Res<T> {
    #[inline]
    fn from(err: CompileError) -> Self {
        Self {
            result: T::default(),
            errors: vec![err],
        }
    }
}

impl<T> From<(T, Vec<CompileError>)> for Res<T> {
    #[inline]
    fn from((result, errors): (T, Vec<CompileError>)) -> Self {
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

impl<T: Default + fmt::Debug> ops::FromResidual<Vec<CompileError>> for Res<T> {
    #[inline]
    fn from_residual(residual: Vec<CompileError>) -> Self {
        Self {
            errors: residual,
            result: T::default(),
        }
    }
}

impl<T: Default> ops::FromResidual<Result<convert::Infallible, CompileError>> for Res<T> {
    #[inline]
    fn from_residual(residual: Result<convert::Infallible, CompileError>) -> Self {
        match residual {
            Ok(_) => panic!(/* By definition of Infallible */),
            Err(err) => Self::from(err),
        }
    }
}

impl<T: Default + fmt::Debug> ops::Try for Res<T> {
    type Output = T;
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

/// Equivalent of [`Res`], but with only one error.
#[derive(Debug)]
pub struct SingleRes<T> {
    /// The error that occurred
    err: Option<CompileError>,
    /// The desired result
    result: T,
}

impl<T> SingleRes<Option<T>> {
    /// Adds an error to a [`SingleRes`] to make it a [`Res`]
    pub(crate) fn add_err(self, error: Option<CompileError>) -> Res<Option<T>> {
        let mut res = Res::from(self.result);
        if let Some(err) = self.err {
            res.errors.push(err);
        }
        if let Some(err) = error {
            res.errors.push(err);
        }
        res
    }

    /// Applies a function to the value if it exists, and applies another
    /// function to the error if it exists.
    ///
    /// # Note
    ///
    /// There can be a value and an error at the same time. In this case, both
    /// functions will be applied.
    #[expect(clippy::min_ident_chars)]
    pub fn map_or_else<U, D: FnMut(CompileError), F: Fn(T) -> U>(
        self,
        mut default: D,
        f: F,
    ) -> Result<U, ()> {
        let (value, error) = self.into_value_err();
        if let Some(err) = error {
            default(err);
        };
        value.map(f).ok_or(())
    }
}

impl<T> SingleRes<T> {
    /// Returns the value and error of the [`SingleRes`].
    fn into_value_err(self) -> (T, Option<CompileError>) {
        (self.result, self.err)
    }
}

impl<T: Default> From<CompileError> for SingleRes<T> {
    fn from(err: CompileError) -> Self {
        Self {
            result: T::default(),
            err: Some(err),
        }
    }
}

impl<T> From<(T, CompileError)> for SingleRes<T> {
    fn from((result, err): (T, CompileError)) -> Self {
        Self {
            result,
            err: Some(err),
        }
    }
}

impl<T> From<T> for SingleRes<T> {
    fn from(result: T) -> Self {
        Self { result, err: None }
    }
}

impl From<Result<(), CompileError>> for SingleRes<()> {
    fn from(value: Result<(), CompileError>) -> Self {
        match value {
            Ok(result) => Self::from(result),
            Err(err) => Self::from(err),
        }
    }
}

impl<T: Default> ops::FromResidual<CompileRes<convert::Infallible>> for SingleRes<T> {
    fn from_residual(residual: CompileRes<convert::Infallible>) -> Self {
        match residual {
            Ok(_) => panic!("infallible type"),
            Err(err) => Self::from(err),
        }
    }
}

impl<T: Default> ops::FromResidual<CompileError> for SingleRes<T> {
    fn from_residual(residual: CompileError) -> Self {
        Self::from(residual)
    }
}

impl<T: Default> ops::Try for SingleRes<T> {
    type Output = T;
    type Residual = CompileError;

    fn branch(self) -> ops::ControlFlow<Self::Residual, Self::Output> {
        if let Some(err) = self.err {
            ops::ControlFlow::Break(err)
        } else {
            ops::ControlFlow::Continue(self.result)
        }
    }

    fn from_output(output: Self::Output) -> Self {
        Self::from(output)
    }
}
