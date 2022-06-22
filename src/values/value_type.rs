use std::any::Any;
use std::fmt::Display;

use crate::token::OldToken;
use crate::values::value_size::ValueSize;

pub mod u64_type;
pub mod bool_type;
pub mod string_type;
pub mod function_type;
pub mod void_type;
pub mod extern_type;
pub mod pointer_type;
pub mod char_type;
pub mod struct_type;
pub mod ignored_type;
pub mod u32_type;
pub mod u16_type;
pub mod u8_type;

#[derive(PartialEq, Debug)]
pub enum ValueTypes {
    U64,
    U32,
    U16,
    U8,
    Bool,
    Char,
    String,
    Function,
    Void,
    Extern,
    Pointer,
    Struct,
    Ignored,
}

pub trait ValueTypeAsAny {
    fn as_any(&self) -> &dyn Any;
}

pub trait ValueType: ValueTypeAsAny + Display {
    fn value_type(&self) -> ValueTypes;
    fn eq(&self, other: &Box<dyn ValueType>) -> bool;

    fn is_valid_bin_op(&self, op: &OldToken, t: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>>;
    fn is_valid_unary_op(&self, op: &OldToken) -> Option<Box<dyn ValueType>>;
    fn is_valid_cast(&self, t: &Box<dyn ValueType>) -> bool;

    fn box_clone(&self) -> Box<dyn ValueType>;

    fn get_size(&self) -> ValueSize;
}

impl Clone for Box<dyn ValueType> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}