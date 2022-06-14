use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::token::{Token, TokenType};
use crate::values::value_type::{ValueType, ValueTypeAsAny, ValueTypes};
use crate::values::value_type::bool_type::BoolType;

#[derive(Clone)]
pub struct PointerType {
    pointee_type: Box<dyn ValueType>,
    is_mutable: bool,
}

impl PointerType {
    pub fn new(pointee_type: Box<dyn ValueType>, is_mutable: bool) -> Self {
        Self {
            pointee_type,
            is_mutable,
        }
    }

    pub fn pointee_type(&self) -> &Box<dyn ValueType> {
        &self.pointee_type
    }
    pub fn is_mutable(&self) -> &bool {
        &self.is_mutable
    }
}

impl ValueTypeAsAny for PointerType {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for PointerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}*{}", self.pointee_type, if self.is_mutable {  " mut" } else { "" })
    }
}

impl ValueType for PointerType {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::Pointer
    }

    fn eq(&self, other: &Box<dyn ValueType>) -> bool {
        self.value_type() == other.value_type() && self.pointee_type.eq(other.as_any().downcast_ref::<Self>().unwrap().pointee_type()) && self.is_mutable == other.as_any().downcast_ref::<Self>().unwrap().is_mutable
    }

    fn is_valid_bin_op(&self, op: &Token, t: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>> {
        if (t.value_type() == ValueTypes::Number || t.value_type() == ValueTypes::Pointer)
            && (op.token_type() == TokenType::Plus || op.token_type() == TokenType::Minus
        ) {
            return Some(self.box_clone());
        }

        if (t.value_type() == ValueTypes::Number || t.value_type() == ValueTypes::Pointer)
            && (op.token_type() == TokenType::Ee || op.token_type() == TokenType::Ne || op.token_type() == TokenType::Gt || op.token_type() == TokenType::Lt || op.token_type() == TokenType::Gte || op.token_type() == TokenType::Lte) {
            return Some(Box::new(BoolType::new()));
        }

        if op.token_type() == TokenType::PointerAssign && t.eq(&self.pointee_type) && self.is_mutable {
            return Some(self.pointee_type.box_clone());
        }

        if op.token_type() == TokenType::Offset && t.value_type() == ValueTypes::Number {
            return Some(self.pointee_type.clone());
        }

        None
    }

    fn is_valid_unary_op(&self, op: &Token) -> Option<Box<dyn ValueType>> {
        None
    }

    fn is_valid_cast(&self, t: &Box<dyn ValueType>) -> bool {
        // TODO: maybe add conversion to number
        if t.value_type() == ValueTypes::Number {
            return true;
        }

        if t.value_type() == ValueTypes::Pointer {
            let pointer_type = t.as_any().downcast_ref::<Self>().unwrap();
            /*if pointer_type.pointee_type().eq(&self.pointee_type) && !*pointer_type.is_mutable() {
                return true;
            }*/
            if !*pointer_type.is_mutable() || (self.is_mutable == *pointer_type.is_mutable()) {
                return true;
            }
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