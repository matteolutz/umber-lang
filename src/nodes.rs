use std::any::Any;
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

#[derive(Debug)]
pub enum NodeType {
    BinOp,
    Call,
    FunctionDef,
    List,
    Break,
    Continue,
    If,
    Return,
    Number,
    While,
    String,
    UnaryOp,
    VarAccess,
    VarAssign,
    VarDeclaration
}

pub trait NodeToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

pub trait Node: NodeToAny + Display {
    fn pos_start(&self) -> &Position;
    fn pos_end(&self) -> &Position;
    fn node_type(&self) -> NodeType;
}