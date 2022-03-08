use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

pub struct CallNode {
    node_to_call: Box<dyn Node>,
    arg_nodes: Vec<Box<dyn Node>>
}

impl CallNode {

    pub fn new(node_to_call: Box<dyn Node>, arg_nodes: Vec<Box<dyn Node>>) -> Self {
        CallNode {
            node_to_call,
            arg_nodes
        }
    }

    pub fn node_to_call(&self) -> &Box<dyn Node> {
        &self.node_to_call
    }
    pub fn arg_nodes(&self) -> &Vec<Box<dyn Node>> {
        &self.arg_nodes
    }

}

impl Display for CallNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<CallNode>[NodeToCall: {}, Args: [{}]]", self.node_to_call, self.arg_nodes.iter().map(|el| format!("{}", el)).collect::<Vec<String>>().join(","))
    }
}

impl NodeToAny for CallNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for CallNode {
    fn pos_start(&self) -> &Position {
        self.node_to_call.pos_start()
    }

    fn pos_end(&self) -> &Position {
        if !self.arg_nodes.is_empty() { self.arg_nodes.last().as_ref().unwrap().pos_end() } else { self.node_to_call.pos_end() }
    }

    fn node_type(&self) -> NodeType {
        NodeType::Call
    }
}