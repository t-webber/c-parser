//! Walks the [`Ast`](crate::parser::api::Ast) and converts it to the [`Ssa`]

extern crate alloc;
use alloc::collections::BTreeMap;

use crate::Res;
use crate::errors::api::{CompileError, ErrorLocation, IntoError as _};
use crate::lineariser::Ssa;
use crate::lineariser::ssa::Symbol;
use crate::parser::api::Literal;

/// Linearising State used to convert the parsed
/// [`Ast`](crate::parser::api::Ast) into a [`Ssa`].
#[derive(Default)]
pub struct LState {
    /// Array of length `depth` containing the variables declared in this scope.
    declarations: Vec<BTreeMap<String, usize>>,
    /// Current scope depth.
    depth: usize,
    /// List of errors that occurred while linearising.
    errors: Vec<CompileError>,
    /// Unique id of the next symbol to be declared.
    next_symbol_id: usize,
    /// Current state of the built [`Ssa`].
    ssa: Ssa,
}

impl LState {
    /// Decrements the depth: exits a block.
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "overflow not possible because AST didn't overflow"
    )]
    pub fn decrement_depth(&mut self) {
        self.depth -= 1;
        let popped = self.declarations.pop();
        debug_assert!(popped.is_some(), "can't decrement without first incrementing");
    }

    /// Increment the id and return the one that can be used.
    ///
    /// This function ensures that every id is unique.
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "todo: fail when no more ids available"
    )]
    pub const fn get_and_bump_symbol_id(&mut self) -> usize {
        let old = self.next_symbol_id;
        self.next_symbol_id += 1;
        old
    }

    /// Increments the depth: enters a block.
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "overflow not possible because AST didn't overflow"
    )]
    pub fn increment_depth(&mut self) {
        self.depth += 1;
        self.declarations.push(BTreeMap::new());
    }

    /// Returns the inner [`Ssa`]
    pub fn into_ssa(self) -> Res<Ssa> {
        Res::from((self.ssa, self.errors))
    }

    /// Pushes a [`Symbol`] in the appropriate symbol table.
    pub fn push_symbol(
        &mut self,
        name: &str,
        init_value: Option<Literal>,
        variable_location: &ErrorLocation,
    ) {
        let id = self.get_and_bump_symbol_id();
        self.ssa.global_symbols.push(Symbol { id, init_value });
        let last = self.declarations.last_mut().expect("depth>=1");
        if last.insert(name.to_owned(), id).is_some() {
            self.errors
                .push(variable_location.to_fault(format!("Redefinition of variable '{name}'")));
        }
    }
}
