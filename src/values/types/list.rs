use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::values::vtype::{ValueType, ValueTypeAsAny, ValueTypes};

pub struct ListType {
    element_type: Box<dyn ValueType>
}

impl ListType {

    pub fn new(element_type: Box<dyn ValueType>) -> Self {
        ListType {
            element_type
        }
    }

    pub fn element_type(&self) -> &Box<dyn ValueType> { &self.element_type }

}

impl ValueTypeAsAny for ListType {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PartialEq for ListType {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Display for ListType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ListType>[{}]", self.element_type)
    }
}

impl ValueType for ListType {
    fn value_type(&self) -> ValueTypes {
        ValueTypes::List
    }

    fn eq(&self, other: &Box<dyn ValueType>) -> bool {
        self.value_type() == other.value_type() && self.element_type.eq(&other.as_any().downcast_ref::<ListType>().as_ref().unwrap().element_type)
    }
}