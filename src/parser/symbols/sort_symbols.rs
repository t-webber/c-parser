//! Module that defines how to parse a symbol and convert it into a symbol. Then
//! the proper handlers are called.

use super::blocks::recursion::TodoBlock;
use super::handlers::{handle_binary_unary, handle_colon, handle_comma, handle_double_unary};
use crate::lexer::api::Symbol;
use crate::parser::modifiers::push::Push as _;
use crate::parser::operators::api::{BinaryOperator, TernaryOperator, UnaryOperator};
use crate::parser::tree::api::Ast;

/// State to specify how to push the symbol into the [`Ast`].
enum SymbolParsing {
    /// There is a [`BinaryOperator`] and a [`UnaryOperator`] that exist with
    /// that symbol.
    ///
    /// Try the binary operator in first argument, and if it is not allowed, try
    /// push the unary operator in the second argument.
    ///
    /// # Examples
    ///
    /// `*` can be [`BinaryOperator::Multiply`] or
    ///   [`UnaryOperator::Indirection`]
    BinaryUnary(BinaryOperator, UnaryOperator),
    /// The is a block character.
    ///
    /// Open a block and do a recursive call to close the block.
    ///
    /// # Examples
    ///
    /// `{`, '(', ']', etc.
    BracedBlock(TodoBlock),
    /// Colon symbol
    Colon,
    /// Comma symbol
    Comma,
    /// There are 2 [`UnaryOperator`] that exist with that symbol.
    ///
    /// Try the first one, and if it is not allowed, try the second.
    ///
    /// # Examples
    ///
    /// `++` can be a [`UnaryOperator::PrefixIncrement`] or a
    /// [`UnaryOperator::PostfixIncrement`].
    DoubleUnary(UnaryOperator, UnaryOperator),
    /// Interrogation mark
    Interrogation,
    /// The symbol exists only for one operator, a [`BinaryOperator`].
    UniqueBinary(BinaryOperator),
    /// The symbol exists only for one operator, a [`UnaryOperator`].
    UniqueUnary(UnaryOperator),
}

impl From<Symbol> for SymbolParsing {
    fn from(value: Symbol) -> Self {
        match value {
            // invalid
            Symbol::Hash | Symbol::Concat =>
                unreachable!("# should have been removed by preprocessor"),
            // mirror unary
            Symbol::BitwiseNot => Self::UniqueUnary(UnaryOperator::BitwiseNot),
            Symbol::LogicalNot => Self::UniqueUnary(UnaryOperator::LogicalNot),
            // mirror binary
            Symbol::Assign => Self::UniqueBinary(BinaryOperator::Assign),
            Symbol::BitwiseOr => Self::UniqueBinary(BinaryOperator::BitwiseOr),
            Symbol::BitwiseXor => Self::UniqueBinary(BinaryOperator::BitwiseXor),
            Symbol::Divide => Self::UniqueBinary(BinaryOperator::Divide),
            Symbol::Gt => Self::UniqueBinary(BinaryOperator::Gt),
            Symbol::Lt => Self::UniqueBinary(BinaryOperator::Lt),
            Symbol::Modulo => Self::UniqueBinary(BinaryOperator::Modulo),
            Symbol::AddAssign => Self::UniqueBinary(BinaryOperator::AddAssign),
            Symbol::AndAssign => Self::UniqueBinary(BinaryOperator::AndAssign),
            Symbol::Different => Self::UniqueBinary(BinaryOperator::Different),
            Symbol::DivAssign => Self::UniqueBinary(BinaryOperator::DivAssign),
            Symbol::Equal => Self::UniqueBinary(BinaryOperator::Equal),
            Symbol::Ge => Self::UniqueBinary(BinaryOperator::Ge),
            Symbol::Le => Self::UniqueBinary(BinaryOperator::Le),
            Symbol::LogicalAnd => Self::UniqueBinary(BinaryOperator::LogicalAnd),
            Symbol::LogicalOr => Self::UniqueBinary(BinaryOperator::LogicalOr),
            Symbol::ModAssign => Self::UniqueBinary(BinaryOperator::ModAssign),
            Symbol::MulAssign => Self::UniqueBinary(BinaryOperator::MulAssign),
            Symbol::OrAssign => Self::UniqueBinary(BinaryOperator::OrAssign),
            Symbol::ShiftLeft => Self::UniqueBinary(BinaryOperator::ShiftLeft),
            Symbol::ShiftRight => Self::UniqueBinary(BinaryOperator::ShiftRight),
            Symbol::SubAssign => Self::UniqueBinary(BinaryOperator::SubAssign),
            Symbol::XorAssign => Self::UniqueBinary(BinaryOperator::XorAssign),
            Symbol::ShiftLeftAssign => Self::UniqueBinary(BinaryOperator::ShiftLeftAssign),
            Symbol::ShiftRightAssign => Self::UniqueBinary(BinaryOperator::ShiftRightAssign),
            // unique non mirrors
            Symbol::Arrow => Self::UniqueBinary(BinaryOperator::StructEnumMemberPointerAccess),
            Symbol::Dot => Self::UniqueBinary(BinaryOperator::StructEnumMemberAccess),
            // postfix has smaller precedence than prefix
            Symbol::Increment =>
                Self::DoubleUnary(UnaryOperator::PostfixIncrement, UnaryOperator::PrefixIncrement),
            Symbol::Decrement =>
                Self::DoubleUnary(UnaryOperator::PostfixDecrement, UnaryOperator::PrefixDecrement),
            // binary and unary operators
            // cases
            Symbol::Ampersand =>
                Self::BinaryUnary(BinaryOperator::BitwiseAnd, UnaryOperator::AddressOf),
            Symbol::Minus => Self::BinaryUnary(BinaryOperator::Subtract, UnaryOperator::Minus),
            Symbol::Plus => Self::BinaryUnary(BinaryOperator::Add, UnaryOperator::Plus),
            Symbol::Star => Self::BinaryUnary(BinaryOperator::Multiply, UnaryOperator::Indirection),
            // braces & blocks
            Symbol::SemiColon => Self::BracedBlock(TodoBlock::SemiColon),
            Symbol::BraceOpen => Self::BracedBlock(TodoBlock::OpenBraceBlock),
            Symbol::BraceClose => Self::BracedBlock(TodoBlock::CloseBraceBlock),
            Symbol::BracketOpen => Self::BracedBlock(TodoBlock::OpenBracket),
            Symbol::BracketClose => Self::BracedBlock(TodoBlock::CloseBracket),
            Symbol::ParenthesisOpen => Self::BracedBlock(TodoBlock::OpenParens),
            Symbol::ParenthesisClose => Self::BracedBlock(TodoBlock::CloseParens),
            // special
            Symbol::Colon => Self::Colon,
            Symbol::Comma => Self::Comma,
            Symbol::Interrogation => Self::Interrogation,
        }
    }
}

/// Function that pushes a [`Symbol`] into an [`Ast`]
///
/// The symbol is converted to a [`SymbolParsing`] to know how to handle the
/// symbol, and then the proper handler is called.
pub fn handle_one_symbol(symbol: Symbol, current: &mut Ast) -> Result<Option<TodoBlock>, String> {
    match SymbolParsing::from(symbol) {
        // unique
        SymbolParsing::UniqueUnary(op) => current.push_op(op)?,
        SymbolParsing::UniqueBinary(op) => current.push_op(op)?,
        // doubles
        SymbolParsing::DoubleUnary(first, second) => handle_double_unary(current, first, second)?,
        SymbolParsing::BinaryUnary(bin_op, un_op) => handle_binary_unary(current, bin_op, un_op)?,
        // blocks
        SymbolParsing::BracedBlock(block) => return Ok(Some(block)),
        // special
        SymbolParsing::Interrogation => current.push_op(TernaryOperator)?, /* ternary (only */
        // ternary because
        // trigraphs are
        // ignored, and colon
        // is sorted in main
        // function in
        // mod.rs)
        SymbolParsing::Colon => handle_colon(current)?,
        SymbolParsing::Comma => handle_comma(current)?,
    }
    Ok(None)
}
