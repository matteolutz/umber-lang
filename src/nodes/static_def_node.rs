use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct StaticDefinitionNode {
    name: String,
    value_type: Box<dyn ValueType>,
    value: Box<dyn Node>,
    is_mutable: bool,
    pos_start: Position,
}

impl StaticDefinitionNode {
    pub fn new(
        name: String,
        value_type: Box<dyn ValueType>,
        value: Box<dyn Node>,
        is_mutable: bool,
        pos_start: Position,
    ) -> Self {
        Self {
            name,
            value_type,
            value,
            is_mutable,
            pos_start,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn value_type(&self) -> &Box<dyn ValueType> {
        &self.value_type
    }
    pub fn value(&self) -> &Box<dyn Node> {
        &self.value
    }
    pub fn is_mutable(&self) -> &bool {
        &self.is_mutable
    }
}

impl NodeToAny for StaticDefinitionNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for StaticDefinitionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<StaticDefinitionNode>[Name: {}, ValueType: {}, Value: {}, IsMutable?: {}]",
            self.name, self.value_type, self.value, self.is_mutable
        )
    }
}

impl Node for StaticDefinitionNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        self.value.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::StaticDef
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
