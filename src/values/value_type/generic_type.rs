use crate::token::Token;
use crate::values::value_size::ValueSize;
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct GenericType {
    generic_name: String,
}

impl GenericType {
    pub fn new(generic_name: String) -> GenericType {
        Self { generic_name }
    }

    pub fn name(&self) -> &str {
        &self.generic_name
    }
}

impl ValueTypeAsAny for GenericType {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for GenericType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.generic_name)
    }
}

impl ValueType for GenericType {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::Generic
    }

    fn eq(&self, other: &Box<dyn ValueType>) -> bool {
        self.value_type() == other.value_type()
            && self.generic_name
                == other
                    .as_any()
                    .downcast_ref::<GenericType>()
                    .unwrap()
                    .generic_name
    }

    fn is_valid_bin_op(&self, _op: &Token, _t: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>> {
        Some(self.box_clone())
    }

    fn is_valid_unary_op(&self, _op: &Token) -> Option<Box<dyn ValueType>> {
        Some(self.box_clone())
    }

    fn is_valid_cast(&self, _t: &Box<dyn ValueType>) -> bool {
        true
    }

    fn box_clone(&self) -> Box<dyn ValueType> {
        Box::new(self.clone())
    }

    fn get_size(&self) -> ValueSize {
        unreachable!("Size of a generic type is unknown");
    }
}
