use std::any::Any;
use std::fmt::Display;

use crate::token::Token;

pub mod number_type;
pub mod bool_type;
pub mod string_type;
pub mod function_type;
pub mod void_type;
pub mod array_type;
pub mod extern_type;
pub mod pointer_type;
pub mod char_type;
pub mod struct_type;

#[derive(PartialEq, Debug)]
pub enum ValueTypes {
    Number,
    Bool,
    Char,
    String,
    Function,
    Void,
    Array,
    Extern,
    Pointer,
    Struct
}

pub trait ValueTypeAsAny {
    fn as_any(&self) -> &dyn Any;
}

pub trait ValueType: ValueTypeAsAny + Display {
    fn value_type(&self) -> ValueTypes;
    fn eq(&self, other: &Box<dyn ValueType>) -> bool;

    fn is_valid_bin_op(&self, op: &Token, t: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>>;
    fn is_valid_unary_op(&self, op: &Token) -> Option<Box<dyn ValueType>>;
    fn is_valid_cast(&self, t: &Box<dyn ValueType>) -> bool;

    fn box_clone(&self) -> Box<dyn ValueType>;

    fn get_size(&self) -> u64;
}

impl Clone for Box<dyn ValueType> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}