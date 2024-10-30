use crate::nodes::Node;

#[derive(Clone)]
pub struct ElseCase {
    statements: Box<dyn Node>,
}

impl ElseCase {
    pub fn new(statements: Box<dyn Node>) -> Self {
        ElseCase { statements }
    }

    pub fn statements(&self) -> &Box<dyn Node> {
        &self.statements
    }
}
