use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

#[derive(Clone)]
pub struct CallNode {
    func_to_call: String,
    arg_nodes: Vec<Box<dyn Node>>,
    pos_start: Position,
}

impl CallNode {
    pub fn new(func_to_call: String, arg_nodes: Vec<Box<dyn Node>>, pos_start: Position) -> Self {
        CallNode {
            func_to_call,
            arg_nodes,
            pos_start,
        }
    }

    pub fn func_to_call(&self) -> &str {
        &self.func_to_call
    }
    pub fn arg_nodes(&self) -> &Vec<Box<dyn Node>> {
        &self.arg_nodes
    }
}

impl Display for CallNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<CallNode>[FuncToCall: {}, Args: [{}]]",
            self.func_to_call,
            self.arg_nodes
                .iter()
                .map(|el| format!("{}", el))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl NodeToAny for CallNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for CallNode {
    fn pos_start(&self) -> &Position {
        &self.pos_start
    }

    fn pos_end(&self) -> &Position {
        if !self.arg_nodes.is_empty() {
            self.arg_nodes.last().as_ref().unwrap().pos_end()
        } else {
            &self.pos_start
        }
    }

    fn node_type(&self) -> NodeType {
        NodeType::Call
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
