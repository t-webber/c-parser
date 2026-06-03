//! Module to define the [`Keyword`] type.

use utils::display;

/// Defines the keyword type and its methods
macro_rules! impl_keywords {
    ($($pascal:ident $str:expr ,)*) => {

        /// Keywords of the language
        ///
        /// See [CppReference](https://en.cppreference.com/w/c/keyword) for the list of C keywords.
        #[derive(Debug)]
        pub enum Keyword {
            $(
                #[doc = concat!("Represents the `", $str, "` keyword")]
                $pascal,
            )*
        }

        #[coverage(off)]
        impl Keyword {
            /// Tries to make a keyword from a literal.
            pub fn from_value_or_res(value: &str) -> TryKeyword {
                match value {
                    $($str => TryKeyword::from(Self::$pascal),)*
                    _ => TryKeyword::Failure,
                }
            }
        }

        display!(Keyword, self, f,
                match self {
                    $(Self::$pascal => $str.fmt(f),)*
                }
    );
    }

}

impl_keywords!(
    Alignof "alignof",
    Alignas "alignas",
    Auto "auto",
    Bool "bool",
    Break "break",
    Case "case",
    Char "char",
    Const "const",
    Constexpr "constexpr",
    Continue "continue",
    Default "default",
    Do "do",
    Double "double",
    Else "else",
    Enum "enum",
    Extern "extern",
    False "false",
    Float "float",
    For "for",
    Goto "goto",
    If "if",
    Inline "inline",
    Int "int",
    Long "long",
    Null "NULL",
    Register "register",
    Restrict "restrict",
    Return "return",
    Short "short",
    Signed "signed",
    Sizeof "sizeof",
    Static "static",
    StaticAssert "static_assert",
    Struct "struct",
    Switch "switch",
    ThreadLocal "thread_local",
    True "true",
    Typedef "typedef",
    Union "union",
    Unsigned "unsigned",
    Void "void",
    Volatile "volatile",
    While "while",
    UAlignas "_Alignas",
    UAlignof "_Alignof",
    UAtomic "_Atomic",
    UBigInt "_BigInt",
    UBool "_Bool",
    UComplex "_Complex",
    UDecimal128 "_Decimal128",
    UDecimal32 "_Decimal32",
    UDecimal64 "_Decimal64",
    UGeneric "_Generic",
    UImaginary "_Imaginary",
    UNoreturn "_Noreturn",
    UStaticAssert "_Static_assert",
    UThreadLocal "_Thread_local",
);

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
