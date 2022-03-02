use std::cmp::max;
use crate::position::Position;
use crate::token::Token;

pub fn string_with_arrows<'a>(text: &'a str, pos_start: &'a Position, pos_end: &'a Position) -> String {
    let mut result = String::new();

    let idx_start = max(text.find("\n").or(Some(0)).expect("Error, while creating string with arrows!"), 0);

    let idx_end_opt = text[idx_start + 1..].find("\n");
    let idx_end = if idx_end_opt.is_some() { idx_end_opt.expect("Error, while creating string with arrows!") } else { text.len() };

    return result;
}