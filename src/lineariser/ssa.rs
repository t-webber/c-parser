//! Module to define the [`Ssa`] structure.

use crate::EMPTY;
use crate::errors::api::Located;
use crate::lineariser::basic_block::BasicBlocks;
use crate::lineariser::symbol::{ElementBuilder, FunctionBuilder, LiteralBuilder, Symbol, Value};
use crate::utils::StringResolver;

/// Static Single Assignment structure.
#[derive(Debug)]
pub struct Ssa {
    /// Basic blocks
    pub basic_blocks: BasicBlocks,
    /// List of global symbols (variarbles, functions, etc.)
    pub symbols: Vec<Symbol>,
}

impl Ssa {
    /// Sorts the symbols of the Ssa for consistency.
    pub fn sort(mut self) {
        self.symbols.sort_by_key(Symbol::id);
    }
}

impl StringResolver<Ssa> {
    /// Returns the display string for the [`Ssa`], sorted to ensure it always
    /// outputs the same string.
    pub fn display(&self) -> String {
        self.as_value()
            .symbols
            .iter()
            .map(|symbol| match symbol {
                Symbol::Element {
                    name,
                    value: ElementBuilder { metadata: LiteralBuilder { id, ty }, value },
                } => format!(
                    "[{}] {} x{id} = {}",
                    name.as_ref()
                        .map_or(EMPTY, |name_id| self.resolve(*name_id)),
                    self.display_type(ty.as_slice(), |attr| attr),
                    self.display_value(value),
                ),
                Symbol::Function { name, value: FunctionBuilder { args, body, id, ret } } =>
                    format!(
                        "[{}] {} f{id}({}) \u{2912} {}",
                        self.resolve(*name),
                        self.display_type(ret.as_slice(), |attr| attr),
                        args.iter()
                            .map(|(id, ty)| format!(
                                "{} x{id}",
                                self.display_type(ty.as_slice(), |attr| attr),
                            ))
                            .collect::<Vec<_>>()
                            .join(", "),
                        if let Some(bb) = body {
                            self.display_bbs(bb)
                        } else {
                            EMPTY.to_owned()
                        }
                    ),
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn display_bbs(&self, bbs: &BasicBlocks) -> String {}
    fn display_value(&self, value: &Value) -> String {}
}
