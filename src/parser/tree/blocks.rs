use core::fmt;

use super::node::Ast;

#[derive(Debug, Default, PartialEq)]
pub struct Block {
    pub elts: Vec<Ast>,
    pub full: bool,
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}{}]",
            self.elts
                .iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<_>>()
                .join(", "),
            if self.full { "" } else { ".." }
        )
    }
}
