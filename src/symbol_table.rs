use std::collections::HashMap;

use crate::values::value_type::ValueType;

pub struct Symbol {
    value_type: Box<dyn ValueType>,
    is_mutable: bool
}

impl Symbol {

    pub fn new(value_type: Box<dyn ValueType>, is_mutable: bool) -> Self {
        Symbol {
            value_type,
            is_mutable
        }
    }

    pub fn is_mutable(&self) -> bool { self.is_mutable }
    pub fn value_type(&self) -> &Box<dyn ValueType> { &self.value_type }

}