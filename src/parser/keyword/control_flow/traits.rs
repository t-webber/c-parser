//! Module to implement the [`ControlFlow`] trait.

use core::fmt;

use crate::parser::modifiers::push::Push;
use crate::parser::types::Ast;

/// trait specifies the  methods that the control flows need to implement.
pub trait ControlFlow: Push + fmt::Display {
    /// Authorised keywords for this control flow
    type Keyword;
    /// Returns the last non-full ast of the control flow as immutable.
    fn as_ast(&self) -> Option<&Ast>;
    /// Returns the last non-full ast of the control flow as mutable.
    fn as_ast_mut(&mut self) -> Option<&mut Ast>;
    /// Marks a control flow as full
    fn fill(&mut self);
    /// Creates a control flow from a keyword
    fn from_keyword(keyword: Self::Keyword) -> Self;
    /// Returns whether the control flow is complete or not.
    ///
    /// A control flow is complete if it doesn't need anything more to make
    /// sense: all the required fields were satisfied.
    ///
    /// This function only differs from [`ControlFlow::is_full`] only for
    /// [`Condition`](super::types::conditional::ConditionCtrl), where an `if`
    /// block can be complete without an `else`, but it not full until the
    /// `else` block has been found.
    fn is_complete(&self) -> bool {
        self.is_full()
    }
    /// Checks if the current control flow is a `if-else` block.
    ///
    /// # Note
    ///
    /// This doesn't search in depth, it only checks the current depth. No
    /// recursion here.
    fn is_condition(&self) -> bool {
        false
    }
    /// Returns whether the control flow is full or not.
    ///
    /// A control flow is full
    /// if nothing can be pushed inside anymore: all the fields were satisfied
    /// and an end of control flow was found (end of scope, semicolon, etc.)
    fn is_full(&self) -> bool;
    /// Checks if the current control flow is a `switch` block.
    ///
    /// # Note
    ///
    /// This doesn't search in depth, it only checks the current depth. No
    /// recursion here.
    fn is_switch(&self) -> bool {
        false
    }
    /// Checks if the current control flow is a `while` block.
    ///
    /// # Note
    ///
    /// This doesn't search in depth, it only checks the current depth. No
    /// recursion here.
    fn is_while(&self) -> bool {
        false
    }
    /// Tries pushing a colon in a control flow
    ///
    /// # Returns
    ///
    /// A [`bool`] that indicated whether the push was successful or not.
    fn push_colon(&mut self) -> bool;
    /// Tries pushing a semicolon in a control flow
    ///
    /// # Returns
    ///
    /// A [`bool`] that indicated whether the push was successful or not.
    fn push_semicolon(&mut self) -> bool;
}
