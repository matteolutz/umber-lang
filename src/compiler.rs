use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Write;
use crate::nodes::{Node, NodeType};
use crate::nodes::binop::BinOpNode;
use crate::nodes::list::ListNode;
use crate::nodes::number::NumberNode;
use crate::nodes::string::StringNode;
use crate::nodes::unaryop::UnaryOpNode;
use crate::nodes::var::assign::VarAssignNode;
use crate::nodes::var::declare::VarDeclarationNode;
use crate::token::{Token, TokenType};

const WORD_SIZE: u8 = 4;

#[derive(Debug)]
#[repr(u8)]
enum Instructions {
    Fetch = 0,
    Store = 1,
    Push = 2,
    Add = 3,
    Sub = 4,
    Mul = 5,
    Div = 6,
    Mod = 7,
    Lt = 8,
    Gt = 9,
    Lte = 10,
    Gte = 11,
    Eq = 12,
    Ne = 13,
    And = 14,
    Or = 15,
    Not = 16,
    Jmp = 17,
    Jz = 18,
    Prtc = 19,
    Prts = 20,
    Prti = 21,
    Halt = 22,
}

fn get_binary_op(t: &Token) -> Instructions {
    match t.token_type() {
        TokenType::Plus => Instructions::Add,
        TokenType::Minus => Instructions::Sub,
        TokenType::Mul => Instructions::Mul,
        TokenType::Div => Instructions::Div,
        TokenType::Modulo => Instructions::Mod,
        TokenType::Ee => Instructions::Eq,
        TokenType::Gt => Instructions::Gt,
        TokenType::Lt => Instructions::Lt,
        TokenType::Gte => Instructions::Gte,
        TokenType::Lte => Instructions::Lte,
        TokenType::Ne => Instructions::Ne,
        _ => panic!("Not a valid bin_op token!")
    }
}

fn emit_byte(c: &mut Vec<u8>, b: u8) {
    c.push(b);
}

fn emit_word(c: &mut Vec<u8>, w: u32) {
    for b in w.to_ne_bytes() {
        c.push(b);
    }
}

fn emit_word_at(c: &mut Vec<u8>, w: u32, n: usize) {
    if n >= c.len() {
        emit_word(c, w);
    }

    c.splice(n..n + WORD_SIZE as usize, w.to_ne_bytes());
}

fn hole(c: &mut Vec<u8>) -> usize {
    emit_word(c, 0);
    c.len()
}

fn code_gen(code: &mut Vec<u8>, string_pool: &mut HashMap<String, u32>, node: &Box<dyn Node>) {
    if node.node_type() == NodeType::VarAccess {
        emit_byte(code, Instructions::Fetch as u8);
        emit_word(code, 0);
    } else if node.node_type() == NodeType::VarAssign {
        code_gen(code, string_pool, node.as_any().downcast_ref::<VarAssignNode>().unwrap().value_node());
        emit_byte(code, Instructions::Store as u8);
        emit_word(code, 0);
    } else if node.node_type() == NodeType::VarDeclaration {
        code_gen(code, string_pool, node.as_any().downcast_ref::<VarDeclarationNode>().unwrap().value_node());
        emit_byte(code, Instructions::Store as u8);
        emit_word(code, 0);
    } else if node.node_type() == NodeType::Number {
        emit_byte(code, Instructions::Push as u8);
        emit_word(code, node.as_any().downcast_ref::<NumberNode>().unwrap().get_number());
    } else if node.node_type() == NodeType::String {
        let string = node.as_any().downcast_ref::<StringNode>().unwrap().get_string();
        emit_byte(code, Instructions::Push as u8);

        if string_pool.contains_key(&string) {
            emit_word(code, *string_pool.get(&string).unwrap());
        } else {
            let prev_len = string_pool.len();
            string_pool.insert(string, prev_len as u32);
            emit_word(code, prev_len as u32);
        }

    } else if node.node_type() == NodeType::BinOp {
        let bin_op_node = node.as_any().downcast_ref::<BinOpNode>().unwrap();
        code_gen(code, string_pool, bin_op_node.left_node());
        code_gen(code, string_pool, bin_op_node.right_node());
        emit_byte(code, get_binary_op(bin_op_node.op_token()) as u8);
    } else if node.node_type() == NodeType::UnaryOp {
        let unary_op = node.as_any().downcast_ref::<UnaryOpNode>().unwrap();

        code_gen(code, string_pool, unary_op.node());

        if unary_op.op_token().token_type() == TokenType::Minus {
            panic!("negative numbers not implemented yet!")
        } else if unary_op.op_token().token_type() == TokenType::Plus {
            // do nothing.
        } else if unary_op.op_token().token_type() == TokenType::Not {
            emit_byte(code, Instructions::Not as u8)
        } else {
            panic!("Not a valid unary_op token!");
        }
    } else if node.node_type() == NodeType::List {
        for el in node.as_any().downcast_ref::<ListNode>().unwrap().element_nodes() {
            code_gen(code, string_pool, el);
        }
    } else {
        panic!("No code gen method for node type {:?}!", node.node_type());
    }
}

pub fn compile(node: &Box<dyn Node>) -> (Vec<u8>, HashMap<String, u32>) {
    let mut code: Vec<u8> = vec![];
    let mut string_pool: HashMap<String, u32> = HashMap::new();

    code_gen(&mut code, &mut string_pool, node);
    emit_byte(&mut code, Instructions::Halt as u8);

    (code, string_pool)
}

pub fn to_virtual_bin(code: &Vec<u8>) -> String {
    let mut temp = String::new();

    let mut pc: usize = 0;
    while pc < code.len() {
        write!(&mut temp, "{:04} ", pc);

        let op = code[pc];
        pc += 1;

        if op == Instructions::Fetch as u8 {
            let x = u32::from_ne_bytes(code[pc..pc+WORD_SIZE as usize].try_into().expect("Couldn't convert!"));
            writeln!(&mut temp, "fetch [{}]", x);
            pc += WORD_SIZE as usize;
        } else if op == Instructions::Store as u8 {
            let x = u32::from_ne_bytes(code[pc..pc+WORD_SIZE as usize].try_into().expect("Couldn't convert!"));
            writeln!(&mut temp, "store [{}]", x);
            pc += WORD_SIZE as usize;
        } else if op == Instructions::Push as u8 {
            let x = u32::from_ne_bytes(code[pc..pc+WORD_SIZE as usize].try_into().expect("Couldn't convert!"));
            writeln!(&mut temp, "push  {}", x);
            pc += WORD_SIZE as usize;
        } else if op == Instructions::Add as u8 {
            writeln!(&mut temp, "add");
        } else if op == Instructions::Sub as u8 {
            writeln!(&mut temp, "sub");
        } else if op == Instructions::Mul as u8 {
            writeln!(&mut temp, "mul");
        } else if op == Instructions::Div as u8 {
            writeln!(&mut temp, "div");
        } else if op == Instructions::Mod as u8 {
            writeln!(&mut temp, "mod");
        } else if op == Instructions::Gt as u8 {
            writeln!(&mut temp, "gt");
        } else if op == Instructions::Lt as u8 {
            writeln!(&mut temp, "lt");
        } else if op == Instructions::Gte as u8 {
            writeln!(&mut temp, "gte");
        } else if op == Instructions::Lte as u8 {
            writeln!(&mut temp, "lte");
        } else if op == Instructions::Eq as u8 {
            writeln!(&mut temp, "eq");
        } else if op == Instructions::Ne as u8 {
            writeln!(&mut temp, "ne");
        } else if op == Instructions::And as u8 {
            writeln!(&mut temp, "and");
        } else if op == Instructions::Or as u8 {
            writeln!(&mut temp, "or");
        } else if op == Instructions::Not as u8 {
            writeln!(&mut temp, "not");
        } else if op == Instructions::Jmp as u8 {
            let x = u32::from_ne_bytes(code[pc..pc+WORD_SIZE as usize].try_into().expect("Couldn't convert!"));
            writeln!(&mut temp, "jmp   ({}) {}", x, pc + x as usize);
            pc += WORD_SIZE as usize;
        } else if op == Instructions::Jz as u8 {
            let x = u32::from_ne_bytes(code[pc..pc+WORD_SIZE as usize].try_into().expect("Couldn't convert!"));
            writeln!(&mut temp, "jz   ({}) {}", x, pc + x as usize);
            pc += WORD_SIZE as usize;
        } else if op == Instructions::Halt as u8 {
            writeln!(&mut temp, "halt");
        } else {
            panic!("unknown instruction: {:?}!", op);
        }
    }

    temp
}