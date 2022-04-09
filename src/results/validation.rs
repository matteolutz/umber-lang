use std::collections::HashMap;
use crate::error;
use crate::error::Error;
use crate::position::Position;
use crate::values::value_type::{ValueType, ValueTypes};
use crate::values::value_type::struct_type::StructType;

pub struct ValidationResult {
    error: Option<Error>,
    value_type: Option<Box<dyn ValueType>>,
    return_type: Option<Box<dyn ValueType>>,
}

impl ValidationResult {

    pub fn new() -> Self {
        ValidationResult {
            error: None,
            value_type: None,
            return_type: None
        }
    }

    pub fn register_res(&mut self, res: ValidationResult) -> Option<Box<dyn ValueType>> {
        if res.has_error() {
            self.error = res.error;
            return None;
        }

        self.return_type = res.return_type.clone();

        if !res.has_value_type() {
            return None;
        }

        return res.value_type;
    }

    pub fn success(&mut self, value_type: Box<dyn ValueType>) {
        if value_type.value_type() == ValueTypes::Struct {
            self.error = Some(error::semantic_error(
                Position::empty(), Position::empty(),
                "Structs are not allowed as return types"
            ));
        }

        self.value_type = Some(value_type);
    }

    pub fn success_return(&mut self, return_type: Box<dyn ValueType>) {
        self.return_type = Some(return_type);
    }

    pub fn failure(&mut self, error: Error) {
        self.error = Some(error);
    }

    pub fn has_error(&self) -> bool { self.error.is_some() }
    pub fn has_value_type(&self) -> bool { self.value_type.is_some() }
    pub fn has_return_type(&self) -> bool { self.return_type.is_some() }

    pub fn value_type(&self) -> &Option<Box<dyn ValueType>> { &self.value_type }
    pub fn return_type(&self) -> &Option<Box<dyn ValueType>> { &self.return_type }
    pub fn error(&self) -> &Option<Error> { &self.error }

}