use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use crate::position::Position;
use crate::utils;

pub struct Error {
    pub pos_start: Position,
    pub pos_end: Position,
    pub error_name: String,
    pub details: String,
}

impl Error {
    pub fn new(pos_start: Position, pos_end: Position, error_name: String, details: String) -> Self {
        Error {
            pos_start,
            pos_end,
            error_name,
            details,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}\n  File '{}', line {}\n\n{}\n", self.error_name, self.details, self.pos_start.file_name(), self.pos_start.line() + 1, utils::string_with_arrows(self.pos_start.file_text(), &self.pos_start, &self.pos_end))
    }
}

pub fn illegal_character_error(pos_start: Position, pos_end: Position, details: &str) -> Error {
    Error::new(pos_start, pos_end, String::from("IllegalCharacterError"), String::from(details))
}

pub fn expected_character_error(pos_start: Position, pos_end: Position, details: &str) -> Error {
    Error::new(pos_start, pos_end, String::from("ExpectedCharacterError"), String::from(details))
}

pub fn invalid_syntax_error(pos_start: Position, pos_end: Position, details: &str) -> Error {
    Error::new(pos_start, pos_end, String::from("InvalidSyntaxError"), String::from(details))
}

pub fn semantic_error(pos_start: Position, pos_end: Position, details: &str) -> Error {
    Error::new(pos_start, pos_end, String::from("SemanticError"), String::from(details))
}