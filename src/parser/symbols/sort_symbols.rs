//! Module that defines how to parse a symbol and convert it into a symbol. Then
//! the proper handlers are called.

use {BinaryOperator as BOp, Symbol as Sy, UnaryOperator as UOp};

use super::super::types::Ast;
use super::super::types::binary::BinaryOperator;
use super::super::types::unary::UnaryOperator;
use super::blocks::TodoBlock;
use super::handlers::{handle_binary_unary, handle_colon, handle_comma, handle_double_unary};
use crate::lexer::api::Symbol;
use crate::parser::types::ternary::TernaryOperator;

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
            // mirror unary
            Sy::BitwiseNot => Self::UniqueUnary(UOp::BitwiseNot),
            Sy::LogicalNot => Self::UniqueUnary(UOp::LogicalNot),
            // mirror binary
            Sy::Assign => Self::UniqueBinary(BOp::Assign),
            Sy::BitwiseOr => Self::UniqueBinary(BOp::BitwiseOr),
            Sy::BitwiseXor => Self::UniqueBinary(BOp::BitwiseXor),
            Sy::Divide => Self::UniqueBinary(BOp::Divide),
            Sy::Gt => Self::UniqueBinary(BOp::Gt),
            Sy::Lt => Self::UniqueBinary(BOp::Lt),
            Sy::Modulo => Self::UniqueBinary(BOp::Modulo),
            Sy::AddAssign => Self::UniqueBinary(BOp::AddAssign),
            Sy::AndAssign => Self::UniqueBinary(BOp::AndAssign),
            Sy::Different => Self::UniqueBinary(BOp::Different),
            Sy::DivAssign => Self::UniqueBinary(BOp::DivAssign),
            Sy::Equal => Self::UniqueBinary(BOp::Equal),
            Sy::Ge => Self::UniqueBinary(BOp::Ge),
            Sy::Le => Self::UniqueBinary(BOp::Le),
            Sy::LogicalAnd => Self::UniqueBinary(BOp::LogicalAnd),
            Sy::LogicalOr => Self::UniqueBinary(BOp::LogicalOr),
            Sy::ModAssign => Self::UniqueBinary(BOp::ModAssign),
            Sy::MulAssign => Self::UniqueBinary(BOp::MulAssign),
            Sy::OrAssign => Self::UniqueBinary(BOp::OrAssign),
            Sy::ShiftLeft => Self::UniqueBinary(BOp::ShiftLeft),
            Sy::ShiftRight => Self::UniqueBinary(BOp::ShiftRight),
            Sy::SubAssign => Self::UniqueBinary(BOp::SubAssign),
            Sy::XorAssign => Self::UniqueBinary(BOp::XorAssign),
            Sy::ShiftLeftAssign => Self::UniqueBinary(BOp::ShiftLeftAssign),
            Sy::ShiftRightAssign => Self::UniqueBinary(BOp::ShiftRightAssign),
            // unique non mirrors
            Sy::Arrow => Self::UniqueBinary(BOp::StructEnumMemberPointerAccess),
            Sy::Dot => Self::UniqueBinary(BOp::StructEnumMemberAccess),
            // postfix has smaller precedence than prefix //TODO: make sure this works
            Sy::Increment => Self::DoubleUnary(UOp::PostfixIncrement, UOp::PrefixIncrement),
            Sy::Decrement => Self::DoubleUnary(UOp::PostfixDecrement, UOp::PrefixDecrement),
            // binary and unary operators //TODO: not sure this is good, may not work on extreme
            // cases
            Sy::Ampersand => Self::BinaryUnary(BOp::BitwiseAnd, UOp::AddressOf),
            Sy::Minus => Self::BinaryUnary(BOp::Subtract, UOp::Minus),
            Sy::Plus => Self::BinaryUnary(BOp::Add, UOp::Plus),
            Sy::Star => Self::BinaryUnary(BOp::Multiply, UOp::Indirection),
            // braces & blocks
            Sy::SemiColon => Self::BracedBlock(TodoBlock::SemiColon),
            Sy::BraceOpen => Self::BracedBlock(TodoBlock::OpenBraceBlock),
            Sy::BraceClose => Self::BracedBlock(TodoBlock::CloseBraceBlock),
            Sy::BracketOpen => Self::BracedBlock(TodoBlock::OpenBracket),
            Sy::BracketClose => Self::BracedBlock(TodoBlock::CloseBracket),
            Sy::ParenthesisOpen => Self::BracedBlock(TodoBlock::OpenParens),
            Sy::ParenthesisClose => Self::BracedBlock(TodoBlock::CloseParens),
            // special
            Sy::Colon => Self::Colon,
            Sy::Comma => Self::Comma,
            Sy::Interrogation => Self::Interrogation,
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
