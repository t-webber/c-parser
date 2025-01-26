//! Module to define the methods for push objects into an
//! [`Ast`].

use core::fmt;

use crate::parser::operators::api::OperatorConversions;
use crate::parser::tree::Ast;

/// Trait to implement the different method to push object into parts of an
/// [`Ast`].
pub trait Push {
    /// Pushes a node at the bottom.
    ///
    /// This methods considers `node` as a leaf, and pushes it as a leaf into
    /// the [`Ast`].
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String>;

    /// Tries to push an operator.
    ///
    /// This method finds, according to the associativities, precedences,
    /// arities and context, were to push the `op` into the [`Ast`].
    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy;
}
