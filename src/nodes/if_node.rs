use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::if_node::case::IfCase;
use crate::nodes::if_node::elsecase::ElseCase;
use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;

pub mod case;
pub mod elsecase;

#[derive(Clone)]
pub struct IfNode {
    cases: Vec<IfCase>,
    else_case: Option<ElseCase>,
}

impl IfNode {
    pub fn new(cases: Vec<IfCase>, else_case: Option<ElseCase>) -> Self {
        if cases.is_empty() {
            panic!("No cases were provided!");
        }

        IfNode { cases, else_case }
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
        for (idx, case) in self.cases.iter().enumerate() {
            write!(
                f,
                "{} ({}) {{ {} }}",
                if idx == 0 { "if" } else { "else if" },
                case.condition(),
                case.statements()
            )?;
        }

        if let Some(else_case) = self.else_case.as_ref() {
            write!(f, "else {{ {} }}", else_case.statements())?;
        }

        Ok(())
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
        (if self.else_case.is_some() {
            self.else_case.as_ref().unwrap().statements()
        } else {
            self.cases.last().as_ref().unwrap().statements()
        })
        .pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::If
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
