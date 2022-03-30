use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::token::Token;
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};

#[derive(Clone)]
pub struct ExternType {}

impl ExternType {

    pub fn new() -> Self {
        ExternType {}
    }

}

impl ValueTypeAsAny for ExternType {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for ExternType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ExternType>[]")
    }
}

impl ValueType for ExternType {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::Extern
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
        0
    }
}