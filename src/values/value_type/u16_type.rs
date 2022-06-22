use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::token::{OldToken, TokenType};
use crate::values::value_size::ValueSize;
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};
use crate::values::value_type::bool_type::BoolType;

#[derive(Clone)]
pub struct U16Type {}

impl U16Type {
    pub fn new() -> Self {
        U16Type {}
    }
}

impl ValueTypeAsAny for U16Type {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for U16Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "u16")
    }
}

impl ValueType for U16Type {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::U16
    }

    fn eq(&self, other: &Box<dyn ValueType>) -> bool {
        self.value_type() == other.value_type()
    }

    fn is_valid_bin_op(&self, op: &OldToken, t: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>> {
        if t.value_type() != ValueTypes::U16 {
            return None;
        }

        match op.token_type() {
            TokenType::Minus | TokenType::Plus | TokenType::Mul | TokenType::Div | TokenType::Modulo | TokenType::BitOr | TokenType::BitAnd | TokenType::BitXor | TokenType::BitShl | TokenType::BitShr => Some(Box::new(U16Type::new())),
            TokenType::Ee | TokenType::Ne | TokenType::Gt | TokenType::Lt | TokenType::Gte | TokenType::Lte => Some(Box::new(BoolType::new())),
            _ => None,
        }
    }

    fn is_valid_unary_op(&self, op: &OldToken) -> Option<Box<dyn ValueType>> {
        match op.token_type() {
            TokenType::Minus | TokenType::Plus | TokenType::BitNot => Some(Box::new(U16Type::new())),
            _ => None
        }
    }

    fn is_valid_cast(&self, t: &Box<dyn ValueType>) -> bool {
        if t.value_type() == ValueTypes::Bool
            || t.value_type() == ValueTypes::Pointer
            || t.value_type() == ValueTypes::Char
            || t.value_type() == ValueTypes::U64
            || t.value_type() == ValueTypes::U32
            || t.value_type() == ValueTypes::U8 {
            return true;
        }

        false
    }

    fn box_clone(&self) -> Box<dyn ValueType> {
        Box::new(self.clone())
    }

    fn get_size(&self) -> ValueSize {
        ValueSize::Word
    }
}