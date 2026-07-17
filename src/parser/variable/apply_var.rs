//! Traits applied to attribute variables for building it.

use crate::Ast;
use crate::errors::api::{ErrorLocation, Located};
use crate::parser::api::{Attribute, UserDefinedTypes};
use crate::parser::modifiers::functions::{CanMakeFnRes, MakeFunction};
use crate::parser::tree::api::{CanPush, PushAttribute};
use crate::parser::variable::Variable;
use crate::parser::variable::api::{PureType, VariableConversion};

impl MakeFunction for Variable {
    fn can_make_function(&self) -> CanMakeFnRes {
        if self.full {
            CanMakeFnRes::None
        } else {
            self.value.can_make_function()
        }
    }

    fn make_function(&mut self, depth: u32, arguments: Vec<Ast>, parens_location: ErrorLocation) {
        self.value.make_function(depth, arguments, parens_location);
    }
}

impl CanPush for Variable {
    fn can_push_leaf(&self) -> bool {
        self.value.can_push_leaf()
    }
}

impl PureType for Variable {
    fn is_pure_type(&self) -> bool {
        self.value.is_pure_type()
    }

    fn take_pure_type(&mut self) -> Option<Vec<Located<Attribute>>> {
        self.value.take_pure_type()
    }
}

impl PushAttribute for Variable {
    fn add_attribute_to_left_variable(
        &mut self,
        previous_attrs: Vec<Located<Attribute>>,
    ) -> Result<(), String> {
        if self.full {
            Err("Can't push attributes to full variable".to_owned())
        } else {
            self.value.add_attribute_to_left_variable(previous_attrs)
        }
    }
}

impl VariableConversion for Variable {
    fn as_partial_typedef(
        &mut self,
    ) -> Option<(Located<UserDefinedTypes>, Option<Located<String>>)> {
        if self.full {
            None
        } else {
            self.value.as_partial_typedef()
        }
    }

    fn into_attrs(self) -> Result<Vec<Located<Attribute>>, String> {
        self.value.into_attrs()
    }

    fn push_comma(&mut self) -> bool {
        if self.full {
            false
        } else {
            self.value.push_comma()
        }
    }
}
