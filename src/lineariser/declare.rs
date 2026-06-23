//! Declares all the global symbols in the given node.

use crate::lineariser::state::LState;
use crate::lineariser::symbol::{Type, Value};
use crate::parser::api::{Ast, AttributeVariable, Declaration, DeclarationValue};

impl AttributeVariable {
    /// Declares all the global symbols in the given node.
    pub fn declare(self, state: &mut LState) -> usize {
        let ty = self.attrs;
        let mut last_id = None;
        for decl in self.declarations.into_iter().flatten() {
            last_id = Some(decl.declare(&ty, state));
        }
        last_id.unwrap_or_else(|| todo!())
    }
}

impl Declaration {
    /// Declares all the global symbols in the given node.
    pub fn declare(self, ty: &Type, state: &mut LState) -> usize {
        let (name, value) = self.into_name_value();
        let init_value = match value {
            DeclarationValue::None => Value::DeclaredOnly,
            DeclarationValue::Value(Ast::Leaf(lit)) => Value::Literal(lit.drop_location()),
            DeclarationValue::Bitfield(_) | DeclarationValue::Value(_) => todo!(),
        };
        state.push_declaration(name, ty, init_value)
    }
}
