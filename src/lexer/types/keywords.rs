//! Module to define the [`Keyword`] type.

use core::fmt;

/// Defines the keyword type and its methods
macro_rules! impl_keywords {
    ($($pascal:ident $type:ident $str:expr ,)*) => {

        /// Keywords of the language
        ///
        /// See [CppReference](https://en.cppreference.com/w/c/keyword) for the list of C keywords.
        #[derive(Debug, PartialEq, Eq)]
        pub enum Keyword {
            $($pascal,)*
        }

        impl Keyword {
            /// Tries to make a keyword from a literal.
            pub fn from_value_or_res(value: &str) -> TryKeyword {
                match value {
                    $($str => TryKeyword::from(Self::$pascal),)*
                    _ => TryKeyword::Failure,
                }
            }

            /// Returns the type of a keyword.
            pub const fn keyword_type(&self) -> KeywordType {
                match self {
                    $(Self::$pascal => KeywordType::$type,)*
                }
            }

        }

        #[expect(clippy::min_ident_chars)]
        impl fmt::Display for Keyword {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$pascal => $str.fmt(f),)*
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
    Null Literal "NULL",
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
    UAlignas Storage "_Alignas",
    UAlignof Operator "_Alignof",
    UAtomic Storage "_Atomic",
    UBigInt Type "_BigInt",
    UBool Type "_Bool",
    UComplex Type "_Complex",
    UDecimal128 Type "_Decimal128",
    UDecimal32 Type "_Decimal32",
    UDecimal64 Type "_Decimal64",
    UGeneric Operator "_Generic",
    UImaginary Type "_Imaginary",
    UNoreturn Storage "_Noreturn",
    UStaticAssert Control "_Static_assert",
    UThreadLocal Storage "_Thread_local",
);

/// Type of keywords
#[derive(Debug, PartialEq, Eq)]
pub enum KeywordType {
    /// Control flow keywords, like `while`, `for`, `case`, `break`. Each
    /// control flow keyword has a specific syntax.
    Control,
    /// Constant keywords, like `true` or `NULL`
    Literal,
    /// Operator keywords. These keyword functions, like `sizeof` or `alignof`.
    Operator,
    /// Storage. These keywords gives information on the storage, the lifetime,
    /// etc. (e.g. `auto`, `static`, `extern`...)
    Storage,
    /// Type keywords mean to specify the type of a variable or of a block. This
    /// includes `enum`, `int`, `_Imaginary`...
    Type,
}

/// Enum to store the keyword and specify if it is deprecated or not.
///
/// # Note
///
/// For the moment, deprecated means deprecated for C23.
pub enum TryKeyword {
    /// Is a keyword, but deprecated for C23
    Deprecated(Keyword),
    /// Not a keyword
    Failure,
    /// Valid keyword
    Success(Keyword),
}

impl From<Keyword> for TryKeyword {
    fn from(keyword: Keyword) -> Self {
        if matches!(keyword, |Keyword::UAlignas| Keyword::UAlignof
            | Keyword::UBool
            | Keyword::UNoreturn
            | Keyword::UStaticAssert
            | Keyword::UThreadLocal)
        {
            Self::Deprecated(keyword)
        } else {
            Self::Success(keyword)
        }
    }
}
