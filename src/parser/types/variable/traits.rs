//! Implements traits to define method widely used through
//! variable types.

use crate::parser::keyword::attributes::UserDefinedTypes;
use crate::parser::types::literal::Attribute;

/// Methods for *pure type*
///
/// A *pure type* is a variable declaration that has a type but no variable
/// name, i.e., contains only attributes and type names, but no variable name or
/// value.
pub trait PureType {
    /// Returns the type of the variable if it is a *pure type*.
    ///
    /// # Note
    ///
    /// This method is used to create casts and compound literals.
    fn is_pure_type(&self) -> bool;
    /// Returns the type of the variable if it is a *pure type*.
    ///
    /// # Returns
    ///
    /// - Some(type) if it is a *pure type*
    /// - None if it is not a *pure type*
    ///
    /// # Note
    ///
    /// This method is used to create casts and compound literals.
    fn take_pure_type(&mut self) -> Option<Vec<Attribute>>;
}

/// Methods to interface with the content of a [`Variable`](super::Variable),
/// either by taking the data out of it, or by checking this data.
pub trait VariableConversion {
    /// Checks if a variable is in reality a type definition.
    ///
    /// `struct Name` is parsed as a variable attributes `struct` and `Name` and
    /// is waiting for the variable name. But if the next token is block, like
    /// in `struct Name {}`, it is meant as a control flow to define the type.
    fn as_partial_typedef(&mut self) -> Option<(&UserDefinedTypes, Option<String>)>;
    /// Checks if a [`Variable`](super::Variable) as a `=` sign.
    fn has_eq(&self) -> bool;
    /// Transforms a [`Variable`](super::Variable) into [`Attribute`]
    fn into_attrs(self) -> Result<Vec<Attribute>, String>;
    /// Transforms a [`Variable`](super::Variable) into a partial typedef
    fn into_partial_typedef(self) -> Option<(UserDefinedTypes, Option<String>)>;
    /// Tries to push a comma into a variable
    fn push_comma(&mut self) -> bool;
}
