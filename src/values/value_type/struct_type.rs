use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
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
        ValueSize::QWORD
    }
}