use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::nodes::if_node::case::IfCase;
use crate::nodes::if_node::elsecase::ElseCase;
use crate::position::Position;

pub mod case;
pub mod elsecase;

pub struct IfNode {
    cases: Vec<IfCase>,
    else_case: Option<ElseCase>,
}

impl IfNode {

    pub fn new(cases: Vec<IfCase>, else_case: Option<ElseCase>) -> Self {
        if cases.is_empty() {
            panic!("No cases were provided!");
        }

        IfNode {
            cases,
            else_case
        }
    }

    pub fn cases(&self) -> &Vec<IfCase> {
        &self.cases
    }
    pub fn else_case(&self) -> &Option<ElseCase> {
        &self.else_case
    }

}

impl Display for IfNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.else_case.is_some() {
            write!(f, "<IfNode>[Cases: [{}], ElseCase: {}]", self.cases.iter().map(|c| format!("{}", c)).collect::<Vec<String>>().join(","), self.else_case.as_ref().unwrap())
        } else {
            write!(f, "<IfNode>[Cases: [{}]]", self.cases.iter().map(|c| format!("{}", c)).collect::<Vec<String>>().join(","))
        }
    }
}

impl NodeToAny for IfNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for IfNode {

    fn pos_start(&self) -> &Position {
        self.cases.first().as_ref().unwrap().condition().pos_start()
    }

    fn pos_end(&self) -> &Position {
        (if self.else_case.is_some() { self.else_case.as_ref().unwrap().statements() } else { self.cases.last().as_ref().unwrap().statements() }).pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::If
    }
}