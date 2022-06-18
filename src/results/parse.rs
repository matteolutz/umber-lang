use crate::error::Error;
use crate::nodes::Node;

pub struct ParseResult {
    error: Option<Error>,
    node: Option<Box<dyn Node>>,
    last_registered_advance_count: usize,
    advance_count: usize,
    to_reverse_count: usize,
}

impl ParseResult {

    pub fn new() -> Self {
        ParseResult {
            error: None,
            node: None,
            last_registered_advance_count: 0,
            advance_count: 0,
            to_reverse_count: 0
        }
    }

    pub fn register_advancement(&mut self) -> () {
        self.last_registered_advance_count = 1;
        self.advance_count += 1;
    }

    pub fn register_res(&mut self, res: ParseResult) -> Option<Box<dyn Node>> {
        self.last_registered_advance_count = res.last_registered_advance_count;
        self.advance_count = res.advance_count;

        if res.has_error() {
            self.error = res.error;
            return None;
        }

        if !res.has_node() {
            return None;
        }

        return res.node;
    }

    pub fn try_register_res(&mut self, res: ParseResult) -> (Option<Box<dyn Node>>, Option<Error>) {
        if res.has_error() {
            self.to_reverse_count = res.to_reverse_count;
            return (None, Some(res.error.unwrap()));
        }
        return (self.register_res(res), None);
    }

    pub fn success(&mut self, node: Box<dyn Node>) -> () {
        self.node = Some(node);
    }

    pub fn failure(&mut self, error: Error) -> () {
        self.error = Some(error);
    }

    pub fn has_error(&self) -> bool { return self.error.is_some(); }
    pub fn has_node(&self) -> bool { return self.node.is_some(); }

    pub fn node(&self) -> &Option<Box<dyn Node>> { return &self.node; }
    pub fn error(&self) -> &Option<Error> { return &self.error; }

    pub fn to_reverse_count(&self) -> usize {
        self.to_reverse_count
    }

}