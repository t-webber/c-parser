//! Declares all the global symbols in the given node.

use crate::lineariser::state::LState;
use crate::lineariser::symbol::{Type, Value};
use crate::parser::api::{Ast, AttributeVariable, Declaration, DeclarationValue};

impl AttributeVariable {
    /// Declares all the global symbols in the given node.
    pub fn declare(self, state: &mut LState) {
        let ty = self.attrs;
        for decl in self.declarations.into_iter().flatten() {
            decl.declare(&ty, state);
        }
    }
}

impl Declaration {
    /// Declares all the global symbols in the given node.
    pub fn declare(self, ty: &Type, state: &mut LState) {
        let (name, value) = self.into_name_value();
        let init_value = match value {
            DeclarationValue::None => Value::DeclaredOnly,
            DeclarationValue::Value(Ast::Leaf(lit)) => Value::Literal(lit),
            DeclarationValue::Bitfield(_) | DeclarationValue::Value(_) => todo!(),
        };
        state.push_declaration(name, ty, init_value);
    }
}
