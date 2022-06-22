use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::token::OldToken;
use crate::values::value_size::ValueSize;
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};

#[derive(Clone)]
pub struct VoidType {}

impl VoidType {

    pub fn new() -> Self {
        VoidType {}
    }

}

impl ValueTypeAsAny for VoidType {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for VoidType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "void")
    }
}

impl ValueType for VoidType {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::Void
    }

    fn eq(&self, other: &Box<dyn ValueType>) -> bool {
        self.value_type() == other.value_type()
    }

    fn is_valid_bin_op(&self, _op: &OldToken, _t: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>> {
        None
    }

    fn is_valid_unary_op(&self, _op: &OldToken) -> Option<Box<dyn ValueType>> {
        None
    }

    fn is_valid_cast(&self, _t: &Box<dyn ValueType>) -> bool {
        false
    }

    fn box_clone(&self) -> Box<dyn ValueType> {
        Box::new(self.clone())
    }

    fn get_size(&self) -> ValueSize {
        ValueSize::Qword
    }
}