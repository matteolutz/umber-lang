use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::token::{Token, TokenType};
use crate::values::value_size::ValueSize;
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};
use crate::values::value_type::bool_type::BoolType;

#[derive(Clone)]
pub struct CharType {}

impl CharType {
    pub fn new() -> Self {
        Self {}
    }
}

impl ValueTypeAsAny for CharType {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for CharType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "char")
    }
}

impl ValueType for CharType {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::Char
    }

    fn eq(&self, other: &Box<dyn ValueType>) -> bool {
        self.value_type() == other.value_type()
    }

    fn is_valid_bin_op(&self, op: &Token, t: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>> {
        if t.value_type() != ValueTypes::Char {
            return None;
        }

        match op.token_type() {
            TokenType::Minus | TokenType::Plus | TokenType::Mul | TokenType::Div | TokenType::Modulo | TokenType::BitOr | TokenType::BitAnd => Some(Box::new(CharType::new())),
            TokenType::Ee | TokenType::Ne | TokenType::Gt | TokenType::Lt | TokenType::Gte | TokenType::Lte => Some(Box::new(BoolType::new())),
            _ => None,
        }
    }

    fn is_valid_unary_op(&self, _: &Token) -> Option<Box<dyn ValueType>> {
        None
    }

    fn is_valid_cast(&self, t: &Box<dyn ValueType>) -> bool {
        if t.value_type() == ValueTypes::U64 {
            return true;
        }

        true
    }

    fn box_clone(&self) -> Box<dyn ValueType> {
        Box::new(self.clone())
    }

    fn get_size(&self) -> ValueSize {
        ValueSize::Byte
    }
}