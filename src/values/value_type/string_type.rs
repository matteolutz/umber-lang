use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::token::{Token, TokenType};
use crate::values::value_size::ValueSize;
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};
use crate::values::value_type::pointer_type::PointerType;

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
        write!(f, "string")
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

    fn is_valid_cast(&self, t: &Box<dyn ValueType>) -> bool {
        if t.value_type() == ValueTypes::Pointer {
            let p = t.as_any().downcast_ref::<PointerType>().unwrap();
            if p.pointee_type().value_type() == ValueTypes::Char && !*p.is_mutable() {
                return true;
            }
        }

        false
    }

    fn box_clone(&self) -> Box<dyn ValueType> {
        Box::new(self.clone())
    }

    fn get_size(&self) -> ValueSize {
        ValueSize::Qword
    }
}