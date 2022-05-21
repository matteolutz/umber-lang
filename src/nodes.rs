use std::any::Any;
use std::fmt::Display;

use crate::position::Position;

pub mod number_node;
pub mod string_node;
pub mod list_node;
pub mod var_node;
pub mod binop_node;
pub mod break_node;
pub mod call_node;
pub mod continue_node;
pub mod functiondef_node;
pub mod if_node;
pub mod return_node;
pub mod unaryop_node;
pub mod while_node;
pub mod statements_node;
pub mod asm_node;
pub mod syscall_node;
pub mod for_node;
pub mod cast_node;
pub mod char_node;
pub mod const_def_node;
pub mod sizeof_node;
pub mod static_def_node;
pub mod struct_def_node;
pub mod read_bytes_node;
pub mod import_node;

#[derive(Debug, PartialEq)]
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
    For,
    String,
    Char,
    UnaryOp,
    VarAccess,
    VarAssign,
    VarDeclaration,
    Statements,
    Assembly,
    Syscall,
    Cast,
    ConstDef,
    SizeOf,
    StaticDef,
    StructDef,
    ReadBytes,
    Import
}

pub trait NodeToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

pub trait Node: NodeToAny + Display {
    fn pos_start(&self) -> &Position;
    fn pos_end(&self) -> &Position;
    fn node_type(&self) -> NodeType;
}