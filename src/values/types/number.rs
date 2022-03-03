use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::values::vtype::{ValueType, ValueTypeAsAny, ValueTypes};

pub struct NumberType {}

impl NumberType {

    pub fn new() -> NumberType {
        NumberType {}
    }

}

impl ValueTypeAsAny for NumberType {
    fn as_any(&self) -> &dyn Any {
        todo!()
    }
}

impl Display for NumberType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<NumberType>[]")
    }
}

impl ValueType for NumberType {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::Number
    }

    fn eq(&self, other: &Box<dyn ValueType>) -> bool {
        self.value_type() == other.value_type()
    }
}