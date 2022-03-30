use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::token::Token;
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};

#[derive(Clone)]
pub struct ArrayType {
    size: usize,
    children_type: Box<dyn ValueType>,
}

impl ArrayType {

    pub fn new(size: usize, children_type: Box<dyn ValueType>) -> Self {
        ArrayType {
            size,
            children_type
        }
    }

    pub fn size(&self) -> &usize { &self.size }
    pub fn children_type(&self) -> &Box<dyn ValueType> { &self.children_type }

}

impl ValueTypeAsAny for ArrayType {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for ArrayType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ArrayType>[Type: {}, Size: {}]", self.children_type, self.size)
    }
}

impl ValueType for ArrayType {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::Array
    }

    fn eq(&self, other: &Box<dyn ValueType>) -> bool {
        self.value_type() == other.value_type() && self.size == other.as_any().downcast_ref::<Self>().unwrap().size && self.children_type.eq(other.as_any().downcast_ref::<Self>().unwrap().children_type())
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
        // self.size as u64 * self.children_type.get_size()
        self.children_type.get_size()
    }
}