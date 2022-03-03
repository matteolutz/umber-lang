use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use crate::context::Context;
use crate::position::Position;

pub struct Error {
    pub pos_start: Position,
    pub pos_end: Position,
    pub error_name: String,
    pub details: String
}

impl Error {

    pub fn new(pos_start: Position, pos_end: Position, error_name: String, details: String) -> Self {
        Error {
            pos_start,
            pos_end,
            error_name,
            details
        }
    }

}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}\n  File '{}', line {}\n\n", self.error_name, self.details, self.pos_start.file_name(), self.pos_start.line() + 1)
    }
}

pub struct RTError<'a> {
    error: Error,
    context: &'a Context<'a>
}

impl Display for RTError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}, {}\n  File '{}', line {}\n\n", self.generate_traceback(), self.error.error_name, self.error.details, self.error.pos_start.file_name(), self.error.pos_start.line() +1)
    }
}

impl<'a> RTError<'a> {
    pub fn generate_traceback(&self) -> String {

        let mut result = String::new();
        let mut pos = self.error.pos_start;
        let mut ctx = Some(self.context);

        while ctx.is_some() && ctx.as_ref().unwrap().parent_entry_pos().is_some() {
            result = format!("  File '{}', line '{}', in '{}'\n{}", pos.file_name(), pos.line() + 1, ctx.unwrap().display_name(), result);
            pos = ctx.as_ref().unwrap().parent_entry_pos().unwrap();

            if ctx.as_ref().unwrap().parent().is_none() {
                break;
            }

            ctx = Some(ctx.as_ref().unwrap().parent().unwrap());
        }

        return result;
    }

    pub fn new(pos_start: Position, pos_end: Position, details: String, context: &'a Context ) -> Self {
        RTError {
            error: Error::new(pos_start, pos_end, String::from("RuntimeError"), details),
            context
        }
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