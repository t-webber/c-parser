//! Walks the [`Ast`](crate::parser::api::Ast) and converts it to the [`Ssa`]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::collections::btree_map::Entry;

use crate::Res;
use crate::errors::api::{CompileError, IntoError as _, Located};
use crate::lineariser::Ssa;
use crate::lineariser::basic_block::BasicBlock;
use crate::lineariser::symbol::{Symbol, Type};
use crate::parser::api::{BracedBlock, Literal};
use crate::utils::SingleUse;

/// Ease macro to handle symbol and basic block ids.
macro_rules! id {
    ($bump:ident $reset:ident $id:tt) => {
        /// Increment the id and return the one that can be used.
        ///
        /// This function ensures that every id is unique.
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "todo: fail when no more ids available"
        )]
        const fn $bump(&mut self) -> SingleUse<usize> {
            let old = self.$id;
            self.$id += 1;
            SingleUse::from(old)
        }

        const fn $reset(&mut self, value: SingleUse<usize>) {
            if let Some(old) = value.try_into_value() {
                self.$id = old;
            }
        }
    };
}

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
    /// Unique id of the next basic block to be created.
    next_basic_block_id: usize,
    /// Unique id of the next symbol to be declared.
    next_symbol_id: usize,
    /// Current state of the built [`Ssa`].
    ssa: Ssa,
}

#[expect(clippy::arbitrary_source_item_ordering, reason = "macro use")]
impl LState {
    id!(get_and_bump_basic_block_id reset_basic_block_id next_basic_block_id);
    id!(get_and_bump_symbol_id reset_symbol_id next_symbol_id);
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
        fn_body: Option<BracedBlock>,
    ) {
        let mut basic_block_id = self.get_and_bump_basic_block_id();
        let mut symbol_id = self.get_and_bump_symbol_id();

        let (name_v, loc) = name.into_inner();
        let last = self.declarations.last_mut().expect("depth>=1");
        match last.entry(name_v.clone()) {
            Entry::Vacant(vacant) => {
                let body = fn_body
                    .map(|body| BasicBlock::from_function_body(basic_block_id.as_value(), body));
                let symbol = Symbol::Function { id: symbol_id.as_value(), args, ret, body };
                vacant.insert(symbol_id.as_value());
                self.ssa.push_symbol(symbol);
            }
            Entry::Occupied(occupied) => {
                let &old_id = occupied.get();
                match self.ssa.get_symbol_mut(old_id).expect("id in declarations") {
                    Symbol::Element { .. } => self.errors.push(
                        loc.to_crash(format!("Function declaration shadows variable {name_v}")),
                    ),
                    Symbol::Function { args: old_args, ret: old_ret, .. }
                        if args != *old_args || ret != *old_ret =>
                        self.errors.push(loc.to_crash(format!(
                            "Redeclaration of function {name_v} with a different signature"
                        ))),
                    Symbol::Function { body: Some(_), .. } =>
                        if fn_body.is_some() {
                            self.errors
                                .push(loc.to_crash(format!("Redefinition of function {name_v}")));
                        },
                    Symbol::Function { body: old_body @ None, .. } =>
                        *old_body = fn_body.map(|body| {
                            BasicBlock::from_function_body(basic_block_id.as_value(), body)
                        }),
                }
            }
        }
        self.reset_basic_block_id(basic_block_id);
        self.reset_symbol_id(symbol_id);
    }

    /// Creates a variable [`Symbol`].
    pub fn push_symbol(&mut self, name: Located<String>, ty: &Type, init_value: Option<Literal>) {
        let mut id = self.get_and_bump_symbol_id();

        let (name_v, loc) = name.into_inner();
        let last = self.declarations.last_mut().expect("depth>=1");
        match last.entry(name_v.clone()) {
            Entry::Vacant(vacant) => {
                let symbol = Symbol::Element { id: id.as_value(), ty: ty.to_owned(), init_value };
                vacant.insert(id.as_value());
                self.ssa.push_symbol(symbol);
            }
            Entry::Occupied(occupied) => {
                let &old_id = occupied.get();
                match self.ssa.get_symbol_mut(old_id).expect("id in declarations") {
                    Symbol::Function { .. } => self.errors.push(
                        loc.to_crash(format!("Variable declaration shadows function {name_v}")),
                    ),
                    Symbol::Element { ty: old_ty, .. } if ty != old_ty => self.errors.push(
                        loc.to_crash(format!("Redeclaration of {name_v} with a different type")),
                    ),
                    Symbol::Element { init_value: Some(_), .. } =>
                        if init_value.is_some() {
                            self.errors
                                .push(loc.to_crash(format!("Redefinition of variable {name_v}")));
                        },
                    Symbol::Element { init_value: old_val @ None, .. } => *old_val = init_value,
                }
            }
        }

        self.reset_symbol_id(id);
    }
}
