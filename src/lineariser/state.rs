//! Walks the [`Ast`](crate::parser::api::Ast) and converts it to the [`Ssa`]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::collections::btree_map::Entry;
use std::collections::HashMap;

use crate::errors::api::{CompileError, IntoError as _, Located};
use crate::lineariser::Ssa;
use crate::lineariser::basic_block::BasicBlocks;
use crate::lineariser::symbol::{ElementBuilder, FunctionBuilder, LiteralBuilder, Type};
use crate::parser::api::{
    Attribute, AttributeKeyword, BasicDataType, BracedBlock, Literal, Modifiers, Qualifiers
};
use crate::utils::SingleUse;
use crate::{Number, Res};

/// Helper macro to create attribute keywords.
macro_rules! attr {
    ($y:ident $t:ident) => {
        Attribute::Keyword(AttributeKeyword::$y($y::$t))
    };
}

/// Linearising State used to convert the parsed
/// [`Ast`](crate::parser::api::Ast) into a [`Ssa`].
#[derive(Default, Debug)]
pub struct LState {
    /// Array of length `depth` containing the variables declared in this scope.
    elements: Vec<BTreeMap<String, ElementBuilder>>,
    /// Errors that occurred while linearising the Ast.
    errors: Vec<CompileError>,
    /// Declared functions.
    functions: BTreeMap<String, FunctionBuilder>,
    /// Literals to put in rodata.
    literals: HashMap<Literal, LiteralBuilder>,
    /// Unique id of the next symbol to be declared.
    next_symbol_id: usize,
    /// Current state of the SSA being built.
    ssa: Ssa,
}

impl LState {
    /// Decrements the depth: exits a block.
    pub fn decrement_depth(&mut self) {
        for (name, element) in self
            .elements
            .pop()
            .expect("can't decrement without first incrementing")
        {
            self.ssa.push_symbol(element.with_name(name));
        }
    }

    /// Returns the ID of a function by name, if found.
    pub fn find_function(&self, fname: &str) -> Option<usize> {
        self.functions.get(fname).map(|func| func.id)
    }

    /// Increment the id and return the one that can be used.
    ///
    /// This function ensures that every id is unique.
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "todo: fail when no more ids available"
    )]
    const fn get_and_bump_symbol_id(&mut self) -> SingleUse<usize> {
        let old = self.next_symbol_id;
        self.next_symbol_id += 1;
        SingleUse::from(old)
    }

    /// Increments the depth: enters a block.
    pub fn increment_depth(&mut self) {
        self.elements.push(BTreeMap::new());
    }

    /// Returns the inner [`Ssa`].
    pub fn into_ssa(mut self) -> Res<Ssa> {
        debug_assert!(self.elements.is_empty(), "unclosed block");
        self.literals
            .into_iter()
            .for_each(|(value, lit)| self.ssa.push_symbol(lit.with_value(value)));
        self.functions
            .into_iter()
            .for_each(|(name, func)| self.ssa.push_symbol(func.with_name(name)));
        Res::from((self.ssa, self.errors))
    }

    /// Creates a variable [`Symbol`](super::symbol::Symbol).
    pub fn push_element(&mut self, name: Located<String>, ty: &Type, init_value: Option<Literal>) {
        let (name_v, loc) = name.into_inner();
        if self.functions.contains_key(&name_v) {
            self.errors
                .push(loc.to_fault(format!("Variable declaration shadows function {name_v}")));
        }
        let mut id = self.get_and_bump_symbol_id();
        let last = self.elements.last_mut().expect("depth>=1");
        match last.entry(name_v.clone()) {
            Entry::Vacant(vacant) => {
                let symbol = ElementBuilder {
                    metadata: LiteralBuilder { id: id.as_value(), ty: ty.to_owned() },
                    init_value,
                };
                vacant.insert(symbol);
            }
            Entry::Occupied(mut occupied) => {
                let old_symbol = occupied.get_mut();
                match old_symbol {
                    ElementBuilder { metadata, .. } if *ty != metadata.ty => self.errors.push(
                        loc.to_crash(format!("Redeclaration of {name_v} with a different type")),
                    ),
                    ElementBuilder { init_value: Some(_), .. } =>
                        if init_value.is_some() {
                            self.errors
                                .push(loc.to_crash(format!("Redefinition of variable {name_v}")));
                        },
                    ElementBuilder { init_value: old_val @ None, .. } => *old_val = init_value,
                }
            }
        }
        self.reset_symbol_id(id);
    }

    /// Adds an error to the state.
    pub fn push_error(&mut self, err: CompileError) {
        self.errors.push(err);
    }

    /// Creates a function [`Symbol`](super::symbol::Symbol).
    pub fn push_function(
        &mut self,
        name: Located<String>,
        args: Vec<Type>,
        ret: Type,
        maybe_fn_body: Option<BracedBlock>,
    ) {
        let (name_v, loc) = name.into_inner();
        if self.elements.len() > 1 {
            self.errors
                .push(loc.to_fault("Non top-level functions is a GCC extension.".to_owned()));
        }
        if self
            .elements
            .last()
            .expect("depth>=1")
            .contains_key(&name_v)
        {
            self.errors
                .push(loc.to_fault(format!("Function declaration shadows variable {name_v}")));
        }

        let mut id = self.get_and_bump_symbol_id();
        let body = maybe_fn_body.map(|body| BasicBlocks::from_function_body(body, self));
        match self.functions.entry(name_v.clone()) {
            Entry::Vacant(vacant) => {
                vacant.insert(FunctionBuilder { args, body, ret, id: id.as_value() });
            }
            Entry::Occupied(mut occupied) => {
                let old_symbol = occupied.get_mut();
                match old_symbol {
                    FunctionBuilder { args: old_args, ret: old_ret, .. }
                        if args != *old_args || ret != *old_ret =>
                        self.errors.push(loc.to_crash(format!(
                            "Redeclaration of function {name_v} with a different signature"
                        ))),
                    FunctionBuilder { body: Some(_), .. } =>
                        if body.is_some() {
                            self.errors
                                .push(loc.to_crash(format!("Redefinition of function {name_v}")));
                        },
                    FunctionBuilder { body: old_body @ None, .. } => *old_body = body,
                }
            }
        }
        self.reset_symbol_id(id);
    }

    /// Creates a new symbol for a literal value.
    pub fn push_literal(&mut self, literal: Literal) -> usize {
        if let Some(sym) = self.literals.get(&literal) {
            return sym.id;
        }
        let mut ty = vec![attr!(Qualifiers Const)];
        ty.extend(match literal {
            Literal::Char(_) => vec![attr!(BasicDataType Char)],
            Literal::ConstantBool(_) => vec![attr!(BasicDataType Bool)],
            Literal::Null => vec![attr!(BasicDataType Void), Attribute::Indirection],
            Literal::Str(_) => vec![
                attr!(BasicDataType Char),
                Attribute::Indirection,
                attr!(Qualifiers Const),
            ],
            Literal::Number(Number::Int(_)) => vec![attr!(BasicDataType Int)],
            Literal::Number(Number::Long(_)) => vec![attr!(Modifiers Long)],
            Literal::Number(Number::LongLong(_)) =>
                vec![attr!(Modifiers Long), attr!( Modifiers Long)],
            Literal::Number(Number::Float(_)) => vec![attr!(BasicDataType Float)],
            Literal::Number(Number::Double(_)) => vec![attr!(BasicDataType Double)],
            Literal::Number(Number::LongDouble(_)) =>
                vec![attr!(Modifiers Long), attr!(BasicDataType Double)],
            Literal::Number(Number::UInt(_)) =>
                vec![attr!(Modifiers Unsigned), attr!(BasicDataType Int)],
            Literal::Number(Number::ULong(_)) =>
                vec![attr!(Modifiers Unsigned), attr!(Modifiers Long)],
            Literal::Number(Number::ULongLong(_)) => vec![
                attr!(Modifiers Unsigned),
                attr!(Modifiers Long),
                attr!(Modifiers Long),
            ],
        });
        let id = self.get_and_bump_symbol_id().as_value();
        self.literals.insert(literal, LiteralBuilder { id, ty });
        id
    }

    /// Resets the symbol id to the given value.
    const fn reset_symbol_id(&mut self, value: SingleUse<usize>) {
        if let Some(old) = value.try_into_value() {
            self.next_symbol_id = old;
        }
    }
}
