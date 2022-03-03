use std::any::Any;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum ValueTypes {
    List,
    Number,
}

pub trait ValueTypeAsAny {
    fn as_any(&self) -> &dyn Any;
}

pub trait ValueType: ValueTypeAsAny + Display {

    fn value_type(&self) -> ValueTypes;
    fn eq(&self, other: &Box<dyn ValueType>) -> bool;

}