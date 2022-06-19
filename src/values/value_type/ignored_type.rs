use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::token::Token;
use crate::values::value_size::ValueSize;
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};

#[derive(Clone)]
pub struct IgnoredType {}


impl IgnoredType {
    pub fn new() -> Self {
        Self {}
    }
}

impl ValueTypeAsAny for IgnoredType {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for IgnoredType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IgnoredType")
    }
}

impl ValueType for IgnoredType {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::Ignored
    }

    fn eq(&self, other: &Box<dyn ValueType>) -> bool {
        other.value_type() == ValueTypes::Ignored
    }

    fn is_valid_bin_op(&self, op: &Token, t: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>> {
        None
    }

    fn is_valid_unary_op(&self, op: &Token) -> Option<Box<dyn ValueType>> {
        None
    }

    fn is_valid_cast(&self, t: &Box<dyn ValueType>) -> bool {
        false
    }

    fn box_clone(&self) -> Box<dyn ValueType> {
        Box::new(self.clone())
    }

    fn get_size(&self) -> ValueSize {
        ValueSize::Qword
    }
}