macro_rules! impl_keywords {
    ($($pascal:ident $ktype:ident $str:expr ,)*) => {
        #[derive(Debug, PartialEq, Eq)]
        pub enum Keyword {
            $($pascal,)*
        }

        impl Keyword {
            pub const fn keyword_type(&self) -> KeywordType {
                match self {
                    $(Self::$pascal => KeywordType::$ktype,)*
                }
            }

            pub const fn repr(&self) -> &str {
                match self {
                    $(Self::$pascal => $str,)*
                }
            }
        }

        impl TryFrom<&str> for Keyword {
            type Error = ();
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    $($str => Ok(Self::$pascal),)*
                    _ => return Err(()),
                }
            }
        }
    };
}

impl_keywords!(
    Alignof Operator "alignof",
    Alignas Storage "alignas",
    Auto Storage "auto",
    Bool Type "bool",
    Break Control "break",
    Case Control "case",
    Char Type "char",
    Const Storage "const",
    Constexpr Storage "constexpr",
    Continue Control "continue",
    Default Control "default",
    Do Control "do",
    Double Type "double",
    Else Control "else",
    Enum Type "enum",
    Extern Storage "extern",
    False Literal "false",
    Float Type "float",
    For Control "for",
    Goto Control "goto",
    If Control "if",
    Inline Storage "inline",
    Int Type "int",
    Long Type "long",
    Nullptr Literal "nullptr",
    Register Storage "register",
    Restrict Storage "restrict",
    Return Control "return",
    Short Type "short",
    Signed Type "signed",
    Sizeof Operator "sizeof",
    Static Storage "static",
    StaticAssert Control "static_assert",
    Struct Type "struct",
    Switch Control "switch",
    ThreadLocal Storage "thread_local",
    True Literal "true",
    Typedef Storage "typedef",
    Typeof Operator "typeof",
    TypeofUnqual Operator "typeof_unqual",
    Union Type "union",
    Unsigned Type "unsigned",
    Void Type "void",
    Volatile Storage "volatile",
    While Control "while",
    UAlignas Storage "_Alignas", // depr C23
    UAlignof Operator "_Alignof", // depr C23
    UAtomic Storage "_Atomic",
    UBitInt Type "_BitInt",
    UBool Type "_Bool", // depr C23
    UComplex Type "_Complex",
    UDecimal128 Type "_Decimal128",
    UDecimal32 Type "_Decimal32",
    UDecimal64 Type "_Decimal64",
    UGeneric Operator "_Generic",
    UImaginary Type "_Imaginary",
    UNoreturn Storage "_Noreturn", // depr C23
    UStaticAssert Control "_Static_assert", // depr C23
    UThreadLocal Storage "_Thread_local", // depr C23
);

#[derive(Debug, PartialEq, Eq)]
pub enum KeywordType {
    Control,
    Literal,
    Operator,
    Storage,
    Type,
}
