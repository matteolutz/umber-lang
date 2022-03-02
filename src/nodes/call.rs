use std::fmt::{Display, Formatter};
use crate::nodes::Node;
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

}

impl Display for CallNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<CallNode>[NodeToCall: {}, Args: [{}]]", self.node_to_call, self.arg_nodes.iter().map(|el| format!("{}", el)).collect::<Vec<String>>().join(","))
    }
}

impl Node for CallNode {
    fn pos_start(&self) -> &Position {
        self.node_to_call.pos_start()
    }

    fn pos_end(&self) -> &Position {
        if !self.arg_nodes.is_empty() { self.arg_nodes.last().as_ref().unwrap().pos_end() } else { self.node_to_call.pos_end() }
    }
}