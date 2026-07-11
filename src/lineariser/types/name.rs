use crate::parser::api::{BasicDataType, UserDefinedTypes};
use crate::utils::{display, from};

/// Actual name of the type segment, stripped of modifiers, qualifiers and what
/// not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeName {
    /// The type is a builtin, like `int` or `char`.
    BasicDataType(BasicDataType),
    /// The type is user-defined with a enum.
    Enum(String),
    /// The type is user-defined with a struct.
    Struct(String),
    /// The type is user-defined with a typedef.
    TypeDef(String),
    /// The type is user-defined with a union.
    Union(String),
}

impl TypeName {
    /// Adds a user defined type attribute to the type name.
    pub fn with(self, usr_def_attr: Option<UserDefinedTypes>) -> Self {
        let Some(usr_def) = usr_def_attr else {
            return self;
        };
        let Self::TypeDef(name) = self else { todo!() };
        match usr_def {
            UserDefinedTypes::Struct => Self::Struct(name),
            UserDefinedTypes::Union => Self::Union(name),
            UserDefinedTypes::Enum => Self::Enum(name),
        }
    }
}

display!(
    TypeName,
    self,
    f,
    match self {
        TypeName::BasicDataType(key) => key.fmt(f),
        TypeName::TypeDef(name) => name.fmt(f),
        TypeName::Struct(name) => write!(f, "struct {name}"),
        TypeName::Union(name) => write!(f, "union {name}"),
        TypeName::Enum(name) => write!(f, "enum {name}"),
    }
);

from!(BasicDataType TypeName);
