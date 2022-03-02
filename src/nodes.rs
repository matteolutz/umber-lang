use std::fmt::{Display, Formatter};
use crate::position::Position;
use crate::token::Token;

pub mod number;
pub mod string;
pub mod list;
pub mod var;
pub mod binop;
pub mod nbreak;
pub mod call;
pub mod ncontinue;
pub mod functiondef;
pub mod nif;
pub mod nreturn;
pub mod unaryop;
pub mod nwhile;

pub trait Node: Display {
    fn pos_start(&self) -> &Position;
    fn pos_end(&self) -> &Position;
}