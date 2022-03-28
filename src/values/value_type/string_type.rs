use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::token::Token;
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};

#[derive(Clone)]
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

    fn is_valid_bin_op(&self, _op: &Token, _t: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>> {
        None
    }

    fn is_valid_unary_op(&self, _op: &Token) -> Option<Box<dyn ValueType>> {
        None
    }

    fn box_clone(&self) -> Box<dyn ValueType> {
        Box::new(self.clone())
    }

    fn get_size(&self) -> u64 {
        8
    }
}