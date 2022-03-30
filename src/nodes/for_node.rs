use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

pub struct ForNode {
    init_stmt: Box<dyn Node>,
    condition: Box<dyn Node>,
    next_expr: Box<dyn Node>,
    body: Box<dyn Node>,
}

impl ForNode {
    pub fn new(
        init_stmt: Box<dyn Node>,
        condition: Box<dyn Node>,
        next_expr: Box<dyn Node>,
        body: Box<dyn Node>,
    ) -> Self {
        Self {
            init_stmt,
            condition,
            next_expr,
            body,
        }
    }

    pub fn init_stmt(&self) -> &Box<dyn Node> {
        &self.init_stmt
    }
    pub fn condition(&self) -> &Box<dyn Node> {
        &self.condition
    }
    pub fn next_expr(&self) -> &Box<dyn Node> {
        &self.next_expr
    }
    pub fn body(&self) -> &Box<dyn Node> {
        &self.body
    }

}

impl NodeToAny for ForNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for ForNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ForNode>[Init: {}, Condition: {}, Next: {}, Body: {}]", self.init_stmt, self.condition, self.next_expr, self.body)
    }
}

impl Node for ForNode {
    fn pos_start(&self) -> &Position {
        self.init_stmt.pos_start()
    }

    fn pos_end(&self) -> &Position {
        self.body.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::For
    }
}