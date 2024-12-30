use core::{convert, ops};

use super::compile::CompileError;
use super::display::display_errors;

type PublicRes<T> = (T, Vec<CompileError>);

/// Struct to store the errors, whilst still having the desired value.
///
/// This struct is meant as a [`Result`], but with the were it is possible to
/// have a value and some errors at the same time. It is for example the case
/// for warnings and suggestions (cf.
/// [`CompileError`] for more information), that must be stored, and at the
/// same time, the compiler continues to work.
#[derive(Debug)]
pub struct Res<T> {
    errors: Vec<CompileError>,
    result: T,
}

impl<T> Res<T> {
    /// Checks if the ``errors`` field is empty
    ///
    /// # Examples
    ///
    /// ```
    /// assert!(c_parser::Res::from(0).errors_empty() == true);
    /// ```
    ///
    /// ```ignore
    /// assert!(Res::from_errs(vec![]).errors_empty() == false);
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
    /// If there is at least one error of level `Error`.
    #[inline]
    pub fn get_displayed_errors(self, files: &[(String, &str)], err_type: &str) -> String {
        display_errors(self.errors, files, err_type)
            .expect("Buffer overflow, failed to fetch errors")
    }

    /// Prints all the errors to the user.
    ///
    /// # Returns
    ///
    /// The value of the [`Res`] if there aren't any errors of level `Error`.
    ///
    /// # Panics
    ///
    /// If there is at least one error of level `Error`.
    #[inline]
    #[expect(clippy::print_stderr)]
    pub fn unwrap_or_display(self, files: &[(String, &str)], err_type: &str) -> T {
        if self.errors.iter().any(CompileError::is_error) {
            eprintln!("{}", self.get_displayed_errors(files, err_type));
            panic!(/* Fail when displaying errors */)
        } else {
            self.result
        }
    }
}

impl<T: Default> Res<T> {
    /// Creates a [`Res`] from one error
    pub(crate) fn from_err(err: CompileError) -> Self {
        Self {
            result: T::default(),
            errors: vec![err],
        }
    }

    /// Creates a [`Res`] from a list of errors
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
