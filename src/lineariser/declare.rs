//! Declares all the global symbols in the given node.

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
            DeclarationValue::Value(Ast::Leaf { value: lit, .. }) => Some(lit),
            this @ (DeclarationValue::Bitfield(_) | DeclarationValue::Value(_)) =>
                todo!("{this:?}"),
        };
        state.push_symbol(name, init_value);
    }
}
