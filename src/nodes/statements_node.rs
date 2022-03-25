use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

pub struct StatementsNode {
    statement_nodes: Vec<Box<dyn Node>>,
    pos_start: Position,
    pos_end: Position
}

impl StatementsNode {

    pub fn new(statement_nodes: Vec<Box<dyn Node>>, pos_start: Position, pos_end: Position) -> Self {
        StatementsNode {
            statement_nodes,
            pos_start,
            pos_end
        }
    }

    pub fn statement_nodes(&self) -> &Vec<Box<dyn Node>> { &self.statement_nodes }

}

impl Display for StatementsNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<StatementNodes>[{}]", self.statement_nodes.iter().map(|el| format!("{}", el)).collect::<Vec<String>>().join(","))
    }
}

impl NodeToAny for StatementsNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for StatementsNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        &self.pos_end
    }

    fn node_type(&self) -> NodeType {
        NodeType::Statements
    }
}