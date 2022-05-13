use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::token::{Token, TokenType};
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};
use crate::values::value_type::bool_type::BoolType;

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
        write!(f, "number")
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
            TokenType::Minus | TokenType::Plus | TokenType::Mul | TokenType::Div | TokenType::Modulo | TokenType::BitOr | TokenType::BitAnd | TokenType::BitXor | TokenType::BitShl | TokenType::BitShr => Some(Box::new(NumberType::new())),
            TokenType::Ee | TokenType::Ne | TokenType::Gt | TokenType::Lt | TokenType::Gte | TokenType::Lte => Some(Box::new(BoolType::new())),
            _ => None,
        }
    }

    fn is_valid_unary_op(&self, op: &Token) -> Option<Box<dyn ValueType>> {
        match op.token_type() {
            TokenType::Minus | TokenType::Plus | TokenType::BitNot => Some(Box::new(NumberType::new())),
            _ => None
        }
    }

    fn is_valid_cast(&self, t: &Box<dyn ValueType>) -> bool {
        if t.value_type() == ValueTypes::Bool
            || t.value_type() == ValueTypes::Pointer
            || t.value_type() == ValueTypes::Char {
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