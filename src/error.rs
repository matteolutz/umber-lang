use std::fmt::{Debug, Display, Formatter};
use colored::Colorize;

use crate::position::Position;

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
        write!(f, "{}:{}:{}: {}: {}\n", self.pos_start.file_name().to_str().unwrap().purple().italic(), (self.pos_start.line() + 1).to_string().yellow().italic(), (self.pos_start.col() + 1).to_string().green().italic(),  self.error_name.red().bold(), self.details)?;

        if let Some(parent) = &self.parent {
            write!(f, "{} {}", "Caused by:".bright_black().italic(), parent)?;
        }

        Ok(())
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
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

// region IOError
pub fn io_error(pos_start: Position, pos_end: Position, details: &str) -> Error {
    Error::new(pos_start, pos_end, String::from("IOError"), String::from(details))
}

pub fn io_error_with_parent(pos_start: Position, pos_end: Position, details: &str, parent: Error) -> Error {
    Error::from_parent(pos_start, pos_end, String::from("IOError"), String::from(details), parent)
}
// endregion

// region IOError
pub fn not_yet_implemented_error(pos_start: Position, pos_end: Position, details: &str) -> Error {
    Error::new(pos_start, pos_end, String::from("NotYetImplementedError"), String::from(details))
}

pub fn not_yet_implemented_parent(pos_start: Position, pos_end: Position, details: &str, parent: Error) -> Error {
    Error::from_parent(pos_start, pos_end, String::from("NotYetImplementedError"), String::from(details), parent)
}
// endregion
