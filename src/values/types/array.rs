use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::token::{Token, TokenType};
use crate::values::types::bool::BoolType;
use crate::values::vtype::{ValueType, ValueTypeAsAny, ValueTypes};
use crate::values::vtype::ValueTypes::Bool;

#[derive(Clone)]
pub struct ArrayType {
    size: u64,
    children_type: Box<dyn ValueType>
}

impl ArrayType {

    pub fn new(size: u64, children_type: Box<dyn ValueType>) -> Self {
        ArrayType {
            size,
            children_type
        }
    }

    pub fn size(&self) -> &u64 { &self.size }
    pub fn children_type(&self) -> &Box<dyn ValueType> { &self.children_type }

}

impl ValueTypeAsAny for ArrayType {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for ArrayType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<NumberType>[]")
    }
}

impl ValueType for ArrayType {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::Array
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

    fn box_clone(&self) -> Box<dyn ValueType> {
        Box::new(self.clone())
    }

}