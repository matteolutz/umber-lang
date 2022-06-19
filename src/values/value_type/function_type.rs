use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::token::Token;
use crate::values::value_size::ValueSize;
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};

// TODO: deprecate this

#[derive(Clone)]
pub struct FunctionType {
    arg_types: Vec<Box<dyn ValueType>>,
    return_type: Box<dyn ValueType>
}

impl FunctionType {

    pub fn new(arg_types: Vec<Box<dyn ValueType>>, return_type: Box<dyn ValueType>) -> Self {
        FunctionType {
            arg_types,
            return_type
        }
    }

    pub fn arg_types(&self) -> &Vec<Box<dyn ValueType>> {
        &self.arg_types
    }

    pub fn return_type(&self) -> &Box<dyn ValueType> {
        &self.return_type
    }

}

impl ValueTypeAsAny for FunctionType {
    fn as_any(&self) -> &dyn Any { self }
}

impl Display for FunctionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<FunctionType>[]")
    }
}

impl ValueType for FunctionType {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::Function
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