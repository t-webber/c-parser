//! Module to define the [`Symbol`] type.

/// Type to represent a symbol
///
/// See [`SymbolState`](super::super::state::api::SymbolState) for more
/// information.
#[expect(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    // one character
    /// &
    Ampersand,
    /// =
    Assign,
    /// ~
    BitwiseNot,
    /// |
    BitwiseOr,
    /// ^
    BitwiseXor,
    /// }
    BraceClose,
    /// {
    BraceOpen,
    /// ]
    BracketClose,
    /// [
    BracketOpen,
    /// :
    Colon,
    /// ,
    Comma,
    /// /
    Divide,
    /// .
    Dot,
    /// >
    Gt,
    /// ?
    Interrogation,
    /// !
    LogicalNot,
    /// <
    Lt,
    /// -
    Minus,
    /// %
    Modulo,
    /// )
    ParenthesisClose,
    /// (
    ParenthesisOpen,
    /// +
    Plus,
    /// ;
    SemiColon,
    /// *
    Star,
    // two characters
    /// +=
    AddAssign,
    /// &=
    AndAssign,
    /// ->
    Arrow,
    /// --
    Decrement,
    /// !=
    Different,
    /// /=
    DivAssign,
    /// ==
    Equal,
    /// >=
    Ge,
    /// ++
    Increment,
    /// <=
    Le,
    /// &&
    LogicalAnd,
    /// ||
    LogicalOr,
    /// %=
    ModAssign,
    /// *=
    MulAssign,
    /// |=
    OrAssign,
    /// <<
    ShiftLeft,
    /// >>
    ShiftRight,
    /// -=
    SubAssign,
    /// ^=
    XorAssign,
    // three characters
    /// <<=
    ShiftLeftAssign,
    /// >>=
    ShiftRightAssign,
}
