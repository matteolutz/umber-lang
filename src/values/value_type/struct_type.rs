use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::token::Token;
use crate::values::value_size::ValueSize;
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};

#[derive(Clone)]
pub struct StructType {
    name: String,
}

impl StructType {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

}

impl ValueTypeAsAny for StructType {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for StructType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "struct {}", self.name)
    }
}

impl ValueType for StructType {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::Struct
    }

    fn eq(&self, other: &Box<dyn ValueType>) -> bool {
        self.value_type() == other.value_type()
    }

    fn is_valid_bin_op(&self, _: &Token, _: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>> {
        None
    }

    fn is_valid_unary_op(&self, _: &Token) -> Option<Box<dyn ValueType>> {
        None
    }

    fn is_valid_cast(&self, _: &Box<dyn ValueType>) -> bool {
        false
    }

    fn box_clone(&self) -> Box<dyn ValueType> {
        Box::new(self.clone())
    }

    fn get_size(&self) -> ValueSize {
        ValueSize::Qword
    }
}