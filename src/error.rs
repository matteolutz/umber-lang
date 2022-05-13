use std::fmt::{Display, Formatter};

use crate::position::Position;
use crate::utils;

#[derive(Clone)]
pub struct Error {
    pos_start: Position,
    pos_end: Position,
    error_name: String,
    details: String,
    parent: Option<Box<Error>>
}

impl Error {
    pub fn new(pos_start: Position, pos_end: Position, error_name: String, details: String) -> Self {
        Error {
            pos_start,
            pos_end,
            error_name,
            details,
            parent: None
        }
    }

    pub fn from_parent(pos_start: Position, pos_end: Position, error_name: String, details: String, parent: Error) -> Self {
        Error {
            pos_start,
            pos_end,
            error_name,
            details,
            parent: Some(Box::new(parent))
        }
    }

    pub fn pos_start(&self) -> &Position {
        &self.pos_start
    }
    pub fn pos_end(&self) -> &Position {
        &self.pos_end
        }
    pub fn error_name(&self) -> &str {
        &self.error_name
    }
    pub fn details(&self) -> &str {
        &self.details
    }

}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(parent) = &self.parent {
            write!(f, "{}", parent);
        }
        // write!(f, "Error: {}: {}\n{}:{}:{}\n\n{}\n", self.error_name, self.details, self.pos_start.file_name(), self.pos_start.line() + 1, self.pos_start.col() + 1, utils::string_with_arrows(self.pos_start.file_text(), &self.pos_start, &self.pos_end))
        write!(f, "{}:{}:{}: {}: {}\n", self.pos_start.file_name(), self.pos_start.line() + 1, self.pos_start.col() + 1, self.error_name, self.details)
    }
}

// region IllegalCharacterError
pub fn illegal_character_error(pos_start: Position, pos_end: Position, details: &str) -> Error {
    Error::new(pos_start, pos_end, String::from("IllegalCharacterError"), String::from(details))
}

pub fn illegal_character_error_with_parent(pos_start: Position, pos_end: Position, details: &str, parent: Error) -> Error {
    Error::from_parent(pos_start, pos_end, String::from("IllegalCharacterError"), String::from(details), parent)
}
// endregion

// region ExpectedCharacterError
pub fn expected_character_error(pos_start: Position, pos_end: Position, details: &str) -> Error {
    Error::new(pos_start, pos_end, String::from("ExpectedCharacterError"), String::from(details))
}

pub fn expected_character_error_with_parent(pos_start: Position, pos_end: Position, details: &str, parent: Error) -> Error {
    Error::from_parent(pos_start, pos_end, String::from("ExpectedCharacterError"), String::from(details), parent)
}
// endregion

// region InvalidSyntaxError
pub fn invalid_syntax_error(pos_start: Position, pos_end: Position, details: &str) -> Error {
    Error::new(pos_start, pos_end, String::from("InvalidSyntaxError"), String::from(details))
}

pub fn invalid_syntax_error_with_parent(pos_start: Position, pos_end: Position, details: &str, parent: Error) -> Error {
    Error::from_parent(pos_start, pos_end, String::from("InvalidSyntaxError"), String::from(details), parent)
}
// endregion

// region SemanticError
pub fn semantic_error(pos_start: Position, pos_end: Position, details: &str) -> Error {
    Error::new(pos_start, pos_end, String::from("SemanticError"), String::from(details))
}

pub fn semantic_error_with_parent(pos_start: Position, pos_end: Position, details: &str, parent: Error) -> Error {
    Error::from_parent(pos_start, pos_end, String::from("SemanticError"), String::from(details), parent)
}
// endregion