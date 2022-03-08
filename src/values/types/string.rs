use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::token::Token;
use crate::values::vtype::{ValueType, ValueTypeAsAny, ValueTypes};

pub struct StringType {}

impl StringType {

    pub fn new() -> Self {
        StringType {}
    }

}

impl ValueTypeAsAny for StringType {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for StringType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<StringType>[]")
    }
}

impl ValueType for StringType {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::String
    }

    fn eq(&self, other: &Box<dyn ValueType>) -> bool {
        self.value_type() == other.value_type()
    }

    fn is_valid_bin_op(&self, op: &Token, t: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>> {
        None
    }

    fn is_valid_unary_op(&self, op: &Token) -> Option<Box<dyn ValueType>> {
        None
    }
}