//! Defines a [`Located`] wrapper to make sure a value can access it's location
//! in case an error or warning should be raised on this value.

use core::fmt;

use crate::errors::api::ErrorLocation;

/// Adds an error location to a value.
#[derive(Default, Clone, Copy)]
pub struct Located<T>(T, ErrorLocation);

impl<T> Located<T> {
    /// References the location.
    pub const fn as_location(&self) -> ErrorLocation {
        self.1
    }

    /// Transfers the mutable reference to the value.
    pub const fn as_ref(&self) -> Located<&T> {
        Located(&self.0, self.1)
    }

    /// References the value.
    pub const fn as_value(&self) -> &T {
        &self.0
    }

    /// Drops the location and returns the value.
    pub fn drop_location(self) -> T {
        self.0
    }

    /// Returns inner value and location.
    pub fn into_inner(self) -> (T, ErrorLocation) {
        (self.0, self.1)
    }

    /// Applies a function to the value but keeping the same location.
    pub fn transfer<U, F: FnOnce(T) -> U>(self, apply: F) -> Located<U> {
        Located(apply(self.0), self.1)
    }
}

impl<T: fmt::Display> fmt::Display for Located<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Debug> fmt::Debug for Located<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: PartialEq> PartialEq for Located<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: PartialEq> Eq for Located<T> {}

impl<T> From<(T, ErrorLocation)> for Located<T> {
    fn from((value, loc): (T, ErrorLocation)) -> Self {
        Self(value, loc)
    }
}
