#[allow(clippy::wildcard_imports)]
use arch_types::*;
use core::fmt;
pub mod arch_types {

    pub type Int = i32;
    #[cfg(target_pointer_width = "32")]
    pub type Long = Int;
    #[cfg(target_pointer_width = "64")]
    pub type Long = LongLong;
    pub type LongLong = i64;
    pub type Float = f32;
    pub type Double = f64;
    pub type LongDouble = f128;
    pub type UInt = u32;
    #[cfg(target_pointer_width = "32")]
    pub type ULong = UiIt;
    #[cfg(target_pointer_width = "64")]
    pub type ULong = ULongLong;
    pub type ULongLong = u64;

    pub type FloatIntPart = u32;
    pub type DoubleIntPart = u64;
    pub type LongDoubleIntPart = u128;
}

macro_rules! define_nb_types {
    ($($t:ident)*) => {
        #[derive(Debug)]
        pub enum Number {
            $($t($t),)*
        }

        #[derive(Clone)]
        pub enum NumberType {
            $($t,)*
        }

    };
}

define_nb_types!(Int Long LongLong Float Double LongDouble UInt ULong ULongLong);

impl NumberType {
    pub const fn is_int(&self) -> bool {
        !matches!(self, Self::Double | Self::Float | Self::LongDouble)
    }

    pub const fn suffix_size(&self) -> usize {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Int => 0,
            Self::Long => 1,
            Self::LongLong => 2,
            Self::Float => 1,
            Self::Double => 0,
            Self::LongDouble => 1,
            Self::UInt => 1,
            Self::ULong => 2,
            Self::ULongLong => 3,
        }
    }
}

#[allow(clippy::min_ident_chars)]
impl fmt::Display for NumberType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Int => "int",
                Self::Long => "long",
                Self::LongLong => "long long",
                Self::Float => "float",
                Self::Double => "double",
                Self::LongDouble => "long double",
                Self::UInt => "unsigned int",
                Self::ULong => "unsigned long",
                Self::ULongLong => "unsigned long long",
            }
        )
    }
}

pub enum Base {
    Binary,
    Decimal,
    Hexadecimal,
    Octal,
}

impl Base {
    pub const fn prefix_size(&self) -> usize {
        match self {
            Self::Binary | Self::Hexadecimal => 2,
            Self::Decimal => 0,
            Self::Octal => 1,
        }
    }

    pub const fn repr(&self) -> &'static str {
        match self {
            Self::Binary => "binary",
            Self::Decimal => "decimal",
            Self::Hexadecimal => "hexadecimal",
            Self::Octal => "octal",
        }
    }
}

pub const ERR_PREFIX: &str = "Invalid number constant type: ";
