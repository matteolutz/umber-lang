use crate::error::Error;
use crate::nodes::Node;
use crate::values::vtype::ValueType;

pub struct ValidationResult {
    error: Option<Error>,
    value_type: Option<Box<dyn ValueType>>,
}

impl ValidationResult {

    pub fn new() -> Self {
        ValidationResult {
            error: None,
            value_type: None,
        }
    }

    pub fn register_res(&mut self, res: ValidationResult) -> Option<Box<dyn ValueType>> {
        if res.has_error() {
            self.error = res.error;
            return None;
        }

        if !res.has_value_type() {
            return None;
        }

        return res.value_type;
    }

    pub fn success(&mut self, value_type: Box<dyn ValueType>) -> () {
        self.value_type = Some(value_type);
    }

    pub fn failure(&mut self, error: Error) -> () {
        self.error = Some(error);
    }

    pub fn has_error(&self) -> bool { return self.error.is_some(); }
    pub fn has_value_type(&self) -> bool { return self.value_type.is_some(); }

    pub fn value_type(&self) -> &Option<Box<dyn ValueType>> { return &self.value_type; }
    pub fn error(&self) -> &Option<Error> { return &self.error; }

}