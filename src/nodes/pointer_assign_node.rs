use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct PointerAssignNode {
    ptr: Box<dyn Node>,
    pointee_type: Box<dyn ValueType>,
    value: Box<dyn Node>,
}

impl PointerAssignNode {
    pub fn new(ptr: Box<dyn Node>, pointee_type: Box<dyn ValueType>, value: Box<dyn Node>) -> Self {
        Self {
            ptr,
            pointee_type,
            value,
        }
    }

    pub fn ptr(&self) -> &Box<dyn Node> {
        &self.ptr
    }
    pub fn pointee_type(&self) -> &Box<dyn ValueType> {
        &self.pointee_type
    }
    pub fn value(&self) -> &Box<dyn Node> {
        &self.value
    }
}

impl NodeToAny for PointerAssignNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for PointerAssignNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<PointerAssignNode>")
    }
}

impl Node for PointerAssignNode {
    fn pos_start(&self) -> &Position {
        self.ptr.pos_start()
    }

    fn pos_end(&self) -> &Position {
        self.value.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::PointerAssign
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
