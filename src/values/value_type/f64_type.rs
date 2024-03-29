use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::token::{Token, TokenType};
use crate::values::value_size::ValueSize;
use crate::values::value_type::bool_type::BoolType;
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};

#[derive(Clone)]
pub struct F64Type {}

impl F64Type {
    pub fn new() -> Self {
        Self {}
    }
}

impl ValueTypeAsAny for F64Type {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for F64Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "f64")
    }
}

impl ValueType for F64Type {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::F64
    }

    fn eq(&self, other: &Box<dyn ValueType>) -> bool {
        self.value_type() == other.value_type()
    }

    fn is_valid_bin_op(&self, op: &Token, t: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>> {
        if t.value_type() != ValueTypes::F64 {
            return None;
        }

        match op.token_type() {
            TokenType::Minus
            | TokenType::Plus
            | TokenType::Mul
            | TokenType::Div
            | TokenType::Modulo
            | TokenType::BitOr
            | TokenType::BitAnd
            | TokenType::BitXor
            | TokenType::BitShl
            | TokenType::BitShr => Some(Box::new(Self::new())),
            TokenType::Ee
            | TokenType::Ne
            | TokenType::Gt
            | TokenType::Lt
            | TokenType::Gte
            | TokenType::Lte => Some(Box::new(BoolType::new())),
            _ => None,
        }
    }

    fn is_valid_unary_op(&self, op: &Token) -> Option<Box<dyn ValueType>> {
        match op.token_type() {
            TokenType::Minus | TokenType::Plus | TokenType::BitNot => Some(Box::new(Self::new())),
            _ => None,
        }
    }

    fn is_valid_cast(&self, t: &Box<dyn ValueType>) -> bool {
        t.value_type() == ValueTypes::Bool || t.value_type() == ValueTypes::U64
    }

    fn box_clone(&self) -> Box<dyn ValueType> {
        Box::new(self.clone())
    }

    fn get_size(&self) -> ValueSize {
        ValueSize::Qword
    }
}
