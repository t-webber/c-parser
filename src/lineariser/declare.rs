//! Declares all the global symbols in the given node.

use crate::lineariser::ssa::Symbol;
use crate::lineariser::state::LState;
use crate::parser::api::{Ast, AttributeVariable, Declaration, DeclarationValue};

impl AttributeVariable {
    /// Declares all the global symbols in the given node.
    pub fn declare(self, state: &mut LState) {
        for decl in self.declarations.into_iter().flatten() {
            decl.declare(state);
        }
    }
}

impl Declaration {
    /// Declares all the global symbols in the given node.
    pub fn declare(self, state: &mut LState) {
        let (name, value) = self.into_name_value();
        let init_value = match value {
            DeclarationValue::None => None,
            DeclarationValue::Value(Ast::Leaf(lit)) => Some(lit),
            this @ (DeclarationValue::Bitfield(_) | DeclarationValue::Value(_)) =>
                todo!("{this:?}"),
        };
        let id = state.get_and_bump_symbol_id();
        let symbol = Symbol::Element { id, init_value };
        state.push_symbol(name, symbol);
    }
}
