use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::token::{Token, TokenType};
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};

#[derive(Clone)]
pub struct BoolType {}

impl BoolType {

    pub fn new() -> Self {
        BoolType {}
    }

}

impl ValueTypeAsAny for BoolType {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for BoolType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BoolType>[]")
    }
}

impl ValueType for BoolType {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::Bool
    }

    fn eq(&self, other: &Box<dyn ValueType>) -> bool {
        self.value_type() == other.value_type()
    }

    fn is_valid_bin_op(&self, op: &Token, t: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>> {
        if t.value_type() != ValueTypes::Bool {
            return None;
        }

        match op.token_type() {
            TokenType::Or | TokenType::And => Some(Box::new(BoolType::new())),
            TokenType::Ee | TokenType::Ne | TokenType::Gt | TokenType::Lt | TokenType::Gte | TokenType::Lte => Some(Box::new(BoolType::new())),
            _ => None,
        }
    }

    fn is_valid_unary_op(&self, op: &Token) -> Option<Box<dyn ValueType>> {
        if op.token_type() == TokenType::Not {
            return Some(Box::new(BoolType::new()));
        }

        None
    }

    fn is_valid_cast(&self, t: &Box<dyn ValueType>) -> bool {
        if t.value_type() == ValueTypes::Number {
            return true;
        }

        false
    }

    fn box_clone(&self) -> Box<dyn ValueType> {
        Box::new(self.clone())
    }

    fn get_size(&self) -> u64 {
        8
    }
}