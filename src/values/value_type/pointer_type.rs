use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::token::{Token, TokenType};
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};

#[derive(Clone)]
pub struct PointerType {
    pointee_type: Box<dyn ValueType>
}

impl PointerType {
    pub fn new(pointee_type: Box<dyn ValueType>) -> Self {
        Self {
            pointee_type
        }
    }

    pub fn pointee_type(&self) -> &Box<dyn ValueType> {
        &self.pointee_type
    }

}

impl ValueTypeAsAny for PointerType {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for PointerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<PointerType>[{}]", self.pointee_type)
    }
}

impl ValueType for PointerType {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::Pointer
    }

    fn eq(&self, other: &Box<dyn ValueType>) -> bool {
        self.value_type() == other.value_type() && self.pointee_type.eq(other.as_any().downcast_ref::<Self>().unwrap().pointee_type())
    }

    fn is_valid_bin_op(&self, _op: &Token, _t: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>> {
        None
    }

    fn is_valid_unary_op(&self, op: &Token) -> Option<Box<dyn ValueType>> {
        if op.token_type() == TokenType::Dereference {
            return Some(self.pointee_type.clone());
        }

        None
    }

    fn box_clone(&self) -> Box<dyn ValueType> {
        Box::new(self.clone())
    }

    fn get_size(&self) -> u64 {
        8
    }
}