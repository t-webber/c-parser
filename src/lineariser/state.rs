//! Walks the [`Ast`](crate::parser::api::Ast) and converts it to the [`Ssa`]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::collections::btree_map::Entry;

use crate::Res;
use crate::errors::api::{CompileError, IntoError as _, Located};
use crate::lineariser::Ssa;
use crate::lineariser::ssa::{Symbol, Type};
use crate::parser::api::Literal;

/// Linearising State used to convert the parsed
/// [`Ast`](crate::parser::api::Ast) into a [`Ssa`].
#[derive(Default)]
pub struct LState {
    /// Array of length `depth` containing the variables declared in this scope.
    declarations: Vec<BTreeMap<String, usize>>,
    /// Current scope depth.
    depth: usize,
    /// Errors that occurred while linearising the Ast.
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

    /// Returns the inner [`Ssa`].
    pub fn into_ssa(self) -> Res<Ssa> {
        Res::from((self.ssa, self.errors))
    }

    /// Creates a function [`Symbol`].
    pub fn push_function(
        &mut self,
        name: Located<String>,
        args: Vec<Type>,
        ret: Type,
        body: Option<()>,
    ) {
        let id = self.get_and_bump_symbol_id();
        let (name_v, loc) = name.into_inner();
        let last = self.declarations.last_mut().expect("depth>=1");
        match last.entry(name_v.clone()) {
            Entry::Vacant(vacant) => {
                let symbol = Symbol::Function { id, args, ret, body };
                vacant.insert(id);
                self.ssa.push_symbol(symbol);
            }
            Entry::Occupied(occupied) => {
                #[allow(
                    clippy::arithmetic_side_effects,
                    clippy::expect_used,
                    clippy::allow_attributes,
                    reason = "just incremented"
                )]
                self.next_symbol_id -= 1;
                let &old_id = occupied.get();
                match self.ssa.get_symbol_mut(old_id).expect("id in declarations") {
                    Symbol::Element { .. } => self.errors.push(
                        loc.to_crash(format!("Function declaration shadows variable {name_v}")),
                    ),
                    Symbol::Function { body: Some(()), .. } =>
                        if body.is_some() {
                            self.errors
                                .push(loc.to_crash(format!("Redefinition of function {name_v}")));
                        },
                    Symbol::Function { args: old_args, ret: old_ret, .. }
                        if args != *old_args || ret != *old_ret =>
                        self.errors.push(loc.to_crash(format!(
                            "Redeclaration of function {name_v} with a different signature"
                        ))),
                    Symbol::Function { body: old_body @ None, .. } => *old_body = body,
                }
            }
        }
    }

    /// Creates a variable [`Symbol`].
    pub fn push_symbol(&mut self, name: Located<String>, ty: &Type, init_value: Option<Literal>) {
        let id = self.get_and_bump_symbol_id();
        let (name_v, loc) = name.into_inner();
        let last = self.declarations.last_mut().expect("depth>=1");
        match last.entry(name_v.clone()) {
            Entry::Vacant(vacant) => {
                let symbol = Symbol::Element { id, ty: ty.to_owned(), init_value };
                vacant.insert(id);
                self.ssa.push_symbol(symbol);
            }
            Entry::Occupied(occupied) => {
                #[allow(
                    clippy::arithmetic_side_effects,
                    clippy::expect_used,
                    clippy::allow_attributes,
                    reason = "just incremented"
                )]
                self.next_symbol_id -= 1;
                let &old_id = occupied.get();
                match self.ssa.get_symbol_mut(old_id).expect("id in declarations") {
                    Symbol::Function { .. } => self.errors.push(
                        loc.to_crash(format!("Variable declaration shadows function {name_v}")),
                    ),
                    Symbol::Element { init_value: Some(_), .. } =>
                        if init_value.is_some() {
                            self.errors
                                .push(loc.to_crash(format!("Redefinition of variable {name_v}")));
                        },
                    Symbol::Element { ty: old_ty, .. } if ty != old_ty =>
                        self.errors.push(loc.to_crash(format!(
                            "Defining declared variable {name_v} with a different type"
                        ))),
                    Symbol::Element { init_value: old_val @ None, .. } => *old_val = init_value,
                }
            }
        }
    }
}
