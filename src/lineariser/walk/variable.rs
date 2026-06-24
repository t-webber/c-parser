//! Walks a variable declaration or usage, updating state and
//! creating symbols and basic blocks.

use crate::errors::api::Located;
use crate::lineariser::basic_block::{BasicBlocks, Id};
use crate::lineariser::state::LState;
use crate::lineariser::symbol::{Type, Value};
use crate::parser::api::{Ast, AttributeVariable, Declaration, DeclarationValue};

impl AttributeVariable {
    /// Pushes some content into the [`BasicBlocks`].
    pub fn push_in(self, bbs: &mut BasicBlocks, state: &mut LState) {
        #[cfg(feature = "debug")]
        crate::lgp!(notab: "Pushing attr var {self}");
        let ty = self.attrs.into_iter().map(Located::drop_location).collect();
        for decl in self.declarations.into_iter().flatten() {
            decl.push_in(bbs, state, &ty);
        }
    }
}

impl Declaration {
    /// Pushes some content into the [`BasicBlocks`].
    pub fn push_in(self, bbs: &mut BasicBlocks, state: &mut LState, ty: &Type) {
        #[cfg(feature = "debug")]
        crate::lgp!(notab: "Pushing decl {self} with type {ty:?}");
        let (name, value) = self.into_name_value();
        let init_value = match value {
            DeclarationValue::None => Value::DeclaredOnly,
            DeclarationValue::Value(Ast::Leaf(lit)) =>
                Value::Variable(state.push_literal(lit.drop_location())),
            DeclarationValue::Value(ast) => {
                let loc = ast.location();
                match ast.push_in(bbs, state) {
                    Some(Id::Found(id)) => Value::Variable(id),
                    Some(Id::NotFound) => Value::DeclaredOnly,
                    None => {
                        state.stat_not_expr(loc);
                        return;
                    }
                }
            }
            DeclarationValue::Bitfield(_) => todo!(),
        };
        state.push_declaration(name, ty, init_value);
    }
}
