//! Module to define the [`Symbol`] type.

/// Type to represent a symbol
///
/// See [`SymbolState`](crate::lexer::state::api::SymbolState) for more
/// information.
#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    /// +=
    AddAssign,
    /// &
    Ampersand,
    /// &=
    AndAssign,
    /// ->
    Arrow,
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
    /// ##
    Concat,
    /// --
    Decrement,
    /// !=
    Different,
    /// /=
    DivAssign,
    /// /
    Divide,
    /// .
    Dot,
    /// ==
    Equal,
    /// >=
    Ge,
    /// >
    Gt,
    /// #
    Hash,
    /// ++
    Increment,
    /// ?
    Interrogation,
    /// <=
    Le,
    /// &&
    LogicalAnd,
    /// !
    LogicalNot,
    /// ||
    LogicalOr,
    /// <
    Lt,
    /// -
    Minus,
    /// %=
    ModAssign,
    /// %
    Modulo,
    /// *=
    MulAssign,
    /// |=
    OrAssign,
    /// )
    ParenthesisClose,
    /// (
    ParenthesisOpen,
    /// +
    Plus,
    /// ;
    SemiColon,
    /// <<
    ShiftLeft,
    /// <<=
    ShiftLeftAssign,
    /// >>
    ShiftRight,
    /// >>=
    ShiftRightAssign,
    /// *
    Star,
    /// -=
    SubAssign,
    /// ^=
    XorAssign,
}
