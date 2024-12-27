#![allow(clippy::arbitrary_source_item_ordering)]

macro_rules! define_attribute_keywords {
    ($($name:ident: $($variant:ident)*,)*) => {

        pub enum AttributeKeyword {
            $($name($name),)*
        }


        impl From<UnsortedAttributeKeyword> for AttributeKeyword {
            fn from(value: UnsortedAttributeKeyword) -> Self {
                match value {
                    $($(UnsortedAttributeKeyword::$variant => Self::$name($name::$variant),)*)*
                }
            }
        }

        pub enum UnsortedAttributeKeyword {
            $($($variant,)*)*
        }


        $(pub enum $name {
            $($variant,)*
        })*


    };
}

define_attribute_keywords!(
    Atomic: Bool Char Double Float Int Complex Decimal128 Decimal32 Decimal64 Imaginary BigInt Void,
    Modifiers: Signed Unsigned Long Short,
    Storage: Auto ThreadLocal Extern Static Register,
    Qualifiers: Const Constexpr Volatile Default,
    Complex: Typedef Struct Union Enum,
    SpecialAttributes: Alignas Inline Restrict Generic Noreturn Atomic,
);
