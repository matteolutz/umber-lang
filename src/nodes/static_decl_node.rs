use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::values::value_type::ValueType;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct StaticDeclarationNode {
    name: String,
    value_type: Box<dyn ValueType>,
    is_mutable: bool,
    pos_start: Position,
    pos_end: Position,
}

impl StaticDeclarationNode {
    pub fn new(
        name: String,
        value_type: Box<dyn ValueType>,
        is_mutable: bool,
        pos_start: Position,
        pos_end: Position,
    ) -> Self {
        Self {
            name,
            value_type,
            is_mutable,
            pos_start,
            pos_end,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn value_type(&self) -> &Box<dyn ValueType> {
        &self.value_type
    }
    pub fn is_mutable(&self) -> &bool {
        &self.is_mutable
    }
}

impl NodeToAny for StaticDeclarationNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for StaticDeclarationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "static{} {}: {}",
            self.name,
            if self.is_mutable { " mut" } else { "" },
            self.value_type
        )
    }
}

impl Node for StaticDeclarationNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::StaticDecl
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
