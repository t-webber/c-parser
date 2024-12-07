trait Operator {
    fn precedence(&self) -> u32;
}

enum Associativity {
    LeftToRight,
    RightToLeft,
}

struct Binary {
    operator: BinaryOperator,
    arg_l: Box<Node>,
    arg_r: Box<Node>,
}
enum BinaryOperator {
    // `[]`
    ArraySubscript,
    // (`.`)
    StructEnumMemberAccess,
    // (`->`)
    StructEnumMemberPointerAccess,
    Multiply,
    Divide,
    Modulo,
    Add,
    Subtract,
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
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivivdeAssign,
    ModuloAssign,
    BitwiseLeftShiftAssign,
    BitwiseRightShiftAssign,
    BitwiseAndAssign,
    BitwiseXorAssign,
    BitwiseOrAssign,
    Comma,
}

impl Operator for BinaryOperator {
    fn precedence(&self) -> u32 {
        use BinaryOperator as BO;
        match self {
            BO::ArraySubscript | BO::StructEnumMemberAccess | BO::StructEnumMemberPointerAccess => {
                1
            }
            BO::Multiply | BO::Divide | BO::Modulo => 3,
            BO::Add | BO::Subtract => 4,
            BO::BitwiseRightShift | BO::BitwiseLeftShift => 5,
            BO::LessThan | BO::LessEqual | BO::GreaterThan | BO::GreaterEqual => 6,
            BO::Equal | BO::Different => 7,
            BO::BitwiseAnd => 8,
            BO::BitwiseXor => 9,
            BO::BitwiseOr => 10,
            BO::LogicalAnd => 11,
            BO::LogicalOr => 12,
            BO::Assign
            | BO::AddAssign
            | BO::SubtractAssign
            | BO::MultiplyAssign
            | BO::DivivdeAssign
            | BO::ModuloAssign
            | BO::BitwiseLeftShiftAssign
            | BO::BitwiseRightShiftAssign
            | BO::BitwiseAndAssign
            | BO::BitwiseXorAssign
            | BO::BitwiseOrAssign => 14,
            BO::Comma => 15,
        }
    }
}

struct CompoundLiteral {
    operator: CompoundLiteralOperator,
    type_: String,
    args: Vec<Box<Node>>,
}

struct CompoundLiteralOperator;

impl Operator for CompoundLiteralOperator {
    fn precedence(&self) -> u32 {
        1
    }
}
struct Function {
    operator: FunctionOperator,
    name: String,
    args: Vec<Box<Node>>,
}

struct FunctionOperator;

impl Operator for FunctionOperator {
    fn precedence(&self) -> u32 {
        1
    }
}

enum Node {
    Leaf(Literal),
    Unary(Unary),
    Binary(Binary),
    Ternary(Ternary),
    Function(Function),
    CompoundLiteral(CompoundLiteral),
    Vec(Vec<Box<Node>>),
}

enum Literal {
    /// # Constants
    /// All constants (int, float, char, string, ...)
    /// For exemple, a string will be stored as `"\"Hellow\""`.
    Const(String),
    String(String),
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
    PostfixIncrement,
    PostfixDecrement,
    PrefixIncrement,
    PrefixDecrement,
    Plus,
    Minus,
    BitwiseNot,
    LogicalNot,
    Cast(String),
    /// Dereference (`*`)
    Indirection,
    /// Address-of (`&`)
    AddressOf,
    SizeOf,
    AlignOf,
}

impl Operator for UnaryOperator {
    fn precedence(&self) -> u32 {
        use UnaryOperator as UO;
        match self {
            UO::Defined => 0,
            UO::PostfixIncrement | UO::PostfixDecrement => 1,
            UO::PrefixIncrement
            | UO::PrefixDecrement
            | UO::Plus
            | UO::Minus
            | UO::BitwiseNot
            | UO::LogicalNot
            | UO::Cast(_)
            | UO::Indirection
            | UO::AddressOf
            | UO::SizeOf
            | UO::AlignOf => 2,
        }
    }
}
