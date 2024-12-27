#![allow(dead_code, reason = "not yet implemented")]

mod attributes;
mod controlflow;
mod functions;

use attributes::{AttributeKeyword as Attr, UnsortedAttributeKeyword as UnsortedAttr};
use controlflow::ControlFlowKeyword as CtrlFlow;
use functions::FunctionKeyword as Func;

use crate::lexer::api::Keyword;

pub enum KeywordParsing {
    Attr(Attr),
    CtrlFlow(CtrlFlow),
    False,
    Func(Func),
    Nullptr,
    True,
}

impl From<(Keyword, bool)> for KeywordParsing {
    fn from((keyword, case_context): (Keyword, bool)) -> Self {
        match keyword {
            // consts
            Keyword::True => Self::True,
            Keyword::False => Self::False,
            Keyword::Nullptr => Self::Nullptr,
            // funcs
            Keyword::Sizeof => Self::Func(Func::Sizeof),
            Keyword::Typeof => Self::Func(Func::Typeof),
            Keyword::TypeofUnqual => Self::Func(Func::TypeofUnqual),
            Keyword::Alignof | Keyword::UAlignof => Self::Func(Func::Alignof),
            Keyword::StaticAssert | Keyword::UStaticAssert => Self::Func(Func::StaticAssert),
            // controlflow
            Keyword::Do => Self::CtrlFlow(CtrlFlow::Do),
            Keyword::If => Self::CtrlFlow(CtrlFlow::If),
            Keyword::For => Self::CtrlFlow(CtrlFlow::For),
            Keyword::Case => Self::CtrlFlow(CtrlFlow::Case),
            Keyword::Else => Self::CtrlFlow(CtrlFlow::Else),
            Keyword::Goto => Self::CtrlFlow(CtrlFlow::Goto),
            Keyword::While => Self::CtrlFlow(CtrlFlow::While),
            Keyword::Break => Self::CtrlFlow(CtrlFlow::Break),
            Keyword::Return => Self::CtrlFlow(CtrlFlow::Return),
            Keyword::Switch => Self::CtrlFlow(CtrlFlow::Switch),
            Keyword::Continue => Self::CtrlFlow(CtrlFlow::Continue),
            Keyword::Default if case_context => Self::CtrlFlow(CtrlFlow::Default),
            // attr
            Keyword::Int => Self::Attr(Attr::from(UnsortedAttr::Int)),
            Keyword::Long => Self::Attr(Attr::from(UnsortedAttr::Long)),
            Keyword::Auto => Self::Attr(Attr::from(UnsortedAttr::Auto)),
            Keyword::Char => Self::Attr(Attr::from(UnsortedAttr::Char)),
            Keyword::Enum => Self::Attr(Attr::from(UnsortedAttr::Enum)),
            Keyword::Void => Self::Attr(Attr::from(UnsortedAttr::Void)),
            Keyword::Short => Self::Attr(Attr::from(UnsortedAttr::Short)),
            Keyword::Float => Self::Attr(Attr::from(UnsortedAttr::Float)),
            Keyword::Union => Self::Attr(Attr::from(UnsortedAttr::Union)),
            Keyword::Const => Self::Attr(Attr::from(UnsortedAttr::Const)),
            Keyword::Inline => Self::Attr(Attr::from(UnsortedAttr::Inline)),
            Keyword::Double => Self::Attr(Attr::from(UnsortedAttr::Double)),
            Keyword::Signed => Self::Attr(Attr::from(UnsortedAttr::Signed)),
            Keyword::Extern => Self::Attr(Attr::from(UnsortedAttr::Extern)),
            Keyword::Static => Self::Attr(Attr::from(UnsortedAttr::Static)),
            Keyword::Struct => Self::Attr(Attr::from(UnsortedAttr::Struct)),
            Keyword::Typedef => Self::Attr(Attr::from(UnsortedAttr::Typedef)),
            Keyword::UAtomic => Self::Attr(Attr::from(UnsortedAttr::Atomic)),
            Keyword::UBigInt => Self::Attr(Attr::from(UnsortedAttr::BigInt)),
            Keyword::Default => Self::Attr(Attr::from(UnsortedAttr::Default)),
            Keyword::Unsigned => Self::Attr(Attr::from(UnsortedAttr::Unsigned)),
            Keyword::Register => Self::Attr(Attr::from(UnsortedAttr::Register)),
            Keyword::Restrict => Self::Attr(Attr::from(UnsortedAttr::Restrict)),
            Keyword::Volatile => Self::Attr(Attr::from(UnsortedAttr::Volatile)),
            Keyword::UComplex => Self::Attr(Attr::from(UnsortedAttr::Complex)),
            Keyword::UGeneric => Self::Attr(Attr::from(UnsortedAttr::Generic)),
            Keyword::UNoreturn => Self::Attr(Attr::from(UnsortedAttr::Noreturn)),
            Keyword::Constexpr => Self::Attr(Attr::from(UnsortedAttr::Constexpr)),
            Keyword::UDecimal64 => Self::Attr(Attr::from(UnsortedAttr::Decimal64)),
            Keyword::UImaginary => Self::Attr(Attr::from(UnsortedAttr::Imaginary)),
            Keyword::UDecimal32 => Self::Attr(Attr::from(UnsortedAttr::Decimal32)),
            Keyword::UDecimal128 => Self::Attr(Attr::from(UnsortedAttr::Decimal128)),
            Keyword::Alignas | Keyword::UAlignas => Self::Attr(Attr::from(UnsortedAttr::Alignas)),
            Keyword::Bool | Keyword::UBool => Self::Attr(Attr::from(UnsortedAttr::Bool)),
            Keyword::ThreadLocal | Keyword::UThreadLocal => {
                Self::Attr(Attr::from(UnsortedAttr::ThreadLocal))
            }
        }
    }
}
