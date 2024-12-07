trait Operator {
    fn precedence(&self) -> u32;
}

struct Binary {
    operator: BinaryOperator,
    arg_l: Box<Node>,
    arg_r: Box<Node>,
}
enum BinaryOperator {
    Times,
    Divide,
    Modulo,
    /*  */
    Plus,
    Minus,
    /*  */
    BitwiseRightShift,
    BitwiseLeftShift,
    /*  */
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    /*  */
    Equal,
    Different,
    /* */
    BitwiseAnd,
    /* */
    BitwiseXor,
    /* */
    BitwiseOr,
    /* */
    LogicalAnd,
    /* */
    LogicalOr,
    /* */
    // assignment operators
}

impl Operator for BinaryOperator {
    fn precedence(&self) -> u32 {
        use BinaryOperator as BO;
        match self {
            BO::Times | BO::Divide | BO::Modulo => 3,
            BO::Plus | BO::Minus => 4,
            BO::BitwiseRightShift | BO::BitwiseLeftShift => 5,
            BO::LessThan | BO::LessEqual | BO::GreaterThan | BO::GreaterEqual => 6,
            BO::Equal | BO::Different => 7,
            BO::BitwiseAnd => 8,
            BO::BitwiseXor => 9,
            BO::BitwiseOr => 10,
            BO::LogicalAnd => 11,
            BO::LogicalOr => 12,
        }
    }
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
    operator: TernaryOperator,
    condition: Box<Node>,
    success: Box<Node>,
    failure: Box<Node>,
}

struct TernaryOperator;

impl Operator for TernaryOperator {
    fn precedence(&self) -> u32 {
        13
    }
}

struct Unary {
    operator: UnaryOperator,
    arg: Box<Node>,
}

enum UnaryOperator {
    Defined,
    /*  */
    PostfixIncrement,
    PostfixDecrement,
    Function(String),
    // []
    // ., ->
    // compound literal
    /*  */
    PrefixIncrement,
    PrefixDecrement,
    Plus,
    Minus,
    BitwiseNot,
    LogicalNot,
    // compound literal
    // *, &
    // sizeof, _Alignof
    /*  */
}

impl Operator for UnaryOperator {
    fn precedence(&self) -> u32 {
        match self {
            UnaryOperator::Defined => 0,
            UnaryOperator::PostfixIncrement
            | UnaryOperator::PostfixDecrement
            | UnaryOperator::Function(_) => 1,
            UnaryOperator::PrefixIncrement
            | UnaryOperator::PrefixDecrement
            | UnaryOperator::Plus
            | UnaryOperator::Minus
            | UnaryOperator::BitwiseNot
            | UnaryOperator::LogicalNot => 2,
        }
    }
}
