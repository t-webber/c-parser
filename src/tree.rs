struct Binary {
    operator: BinaryOperator,
    arg_l: Box<Node>,
    arg_r: Box<Node>,
}
enum BinaryOperator {
    Times,
    Divide,
    Modulo,
    Plus,
    Minus,
    BitwiseRightShift,
    BitwiseLeftShift,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    Equal,
    Different,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
}

pub enum Node {
    Leaf(Literal),
    Unary(Unary),
    Binary(Binary),
    Ternary(Ternary),
}

enum Literal {
    String(String),
    Number(u32),
    Variable(String),
}

struct Ternary {
    condition: Box<Node>,
    success: Box<Node>,
    failure: Box<Node>,
}

struct Unary {
    operator: UnaryOperator,
    arg: Box<Node>,
}

enum UnaryOperator {
    Defined,
    PostfixIncrement,
    PostfixDecrement,
    Function(String),
    PrefixIncrement,
    PrefixDecrement,
    Plus,
    Minus,
    BitwiseNot,
    LogicalNot,
}
