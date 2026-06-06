//! Declares all the global symbols in the given node.

use crate::lineariser::state::LState;
use crate::parser::api::{AstValue, AttributeVariable, Declaration, DeclarationValue};

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
            DeclarationValue::Value(val) =>
                if let AstValue::Leaf(lit) = val.value {
                    Some(lit)
                } else {
                    todo!("{val}")
                },
            this @ DeclarationValue::Bitfield(_) => todo!("{this:?}"),
        };
        state.push_symbol(name, init_value);
    }
}
