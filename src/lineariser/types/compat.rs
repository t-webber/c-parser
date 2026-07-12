use crate::Res;
use crate::lineariser::types::{CONST, Type, UNSIGNED};
use crate::parser::api::{BasicDataType, UnaryOperator};

impl Type {
    /// Returns the output of a type when passed to a unary operator.
    pub fn apply_unary(self, op: UnaryOperator) -> Res<Self> {
        match op {
            UnaryOperator::AddressOf => todo!(),
            UnaryOperator::Indirection => todo!(),
            UnaryOperator::LogicalNot => Res::ok(Self::from_base(BasicDataType::Bool.into())),
            UnaryOperator::Minus => Res::ok(self.drop_unsigned().drop_const()),
            UnaryOperator::Plus
            | UnaryOperator::BitwiseNot
            | UnaryOperator::PostfixDecrement
            | UnaryOperator::PostfixIncrement
            | UnaryOperator::PrefixDecrement
            | UnaryOperator::PrefixIncrement => Res::ok(self.drop_const()),
        }
    }

    /// Drops the const qualifier, if present.
    fn drop_const(mut self) -> Self {
        self.indirections
            .last_mut()
            .expect(">=1")
            .retain(|dec| *dec != CONST);
        self
    }

    /// Drops the unsigned modifier, if present.
    fn drop_unsigned(mut self) -> Self {
        self.base_decorations.retain(|dec| *dec != UNSIGNED);
        self
    }
}
