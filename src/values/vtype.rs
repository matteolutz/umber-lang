use std::any::Any;
use std::fmt::Display;
use crate::token::Token;

#[derive(PartialEq, Debug)]
pub enum ValueTypes {
    Number,
    Bool,
    String,
    Function,
}

pub trait ValueTypeAsAny {
    fn as_any(&self) -> &dyn Any;
}

pub trait ValueType: ValueTypeAsAny + Display {

    fn value_type(&self) -> ValueTypes;
    fn eq(&self, other: &Box<dyn ValueType>) -> bool;

    fn is_valid_bin_op(&self, op: &Token, t: &Box<dyn ValueType>) -> Option<Box<dyn ValueType>>;
    fn is_valid_unary_op(&self, op: &Token) -> Option<Box<dyn ValueType>>;

}