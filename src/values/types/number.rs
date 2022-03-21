use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::token::{Token, TokenType};
use crate::values::types::bool::BoolType;
use crate::values::vtype::{ValueType, ValueTypeAsAny, ValueTypes};

#[derive(Clone)]
pub struct NumberType {}

impl NumberType {

    pub fn new() -> Self {
        NumberType {}
    }

}

impl ValueTypeAsAny for NumberType {
    fn as_any(&self) -> &dyn Any {
        self
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

    fn is_valid_bin_op(&self, op: &Token, t: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>> {
        if t.value_type() != ValueTypes::Number {
            return None;
        }

        match op.token_type() {
            TokenType::Minus | TokenType::Plus | TokenType::Mul | TokenType::Div | TokenType::BitOr | TokenType::BitAnd => Some(Box::new(NumberType::new())),
            TokenType::Ee | TokenType::Ne | TokenType::Gt | TokenType::Lt | TokenType::Gte | TokenType::Lte => Some(Box::new(BoolType::new())),
            _ => None,
        }
    }

    fn is_valid_unary_op(&self, op: &Token) -> Option<Box<dyn ValueType>> {
        match op.token_type() {
            TokenType::Minus | TokenType::Plus => Some(Box::new(NumberType::new())),
            _ => None
        }
    }

    fn box_clone(&self) -> Box<dyn ValueType> {
        Box::new(self.clone())
    }
    
}