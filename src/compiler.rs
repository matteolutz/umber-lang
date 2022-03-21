use std::fmt::Write;

use crate::nodes::{Node, NodeType};
use crate::nodes::binop::BinOpNode;
use crate::nodes::functiondef::FunctionDefinitionNode;
use crate::nodes::number::NumberNode;
use crate::nodes::statements::StatementsNode;
use crate::token::TokenType;
use crate::values::types::function::FunctionType;

const SCRATCH_REGS: [&str; 7] = [
    "%rbx", "%r10", "%r11", "%r12", "%r13", "%r14", "%r15"
];

pub struct Compiler {
    scratch_regs: u8,
    label_count: u128
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            scratch_regs: 0,
            label_count: 0
        }
    }

    // region Register distribution
    pub fn res_scratch(&mut self) -> u8 {
        if self.scratch_regs ^ 0b1111111 == 0 {
            panic!("No free general purpose registers were found!");
        }

        let mut i = 0;
        let mut current = self.scratch_regs;
        while current & 1 == 1 {
            i += 1;
            current >>= 1;
        }

        self.scratch_regs |= 1 << i;

        i
    }

    pub fn scratch_name(&self, i: u8) -> &str { SCRATCH_REGS[i as usize] }

    pub fn free_scratch(&mut self, reg: u8) {
        self.scratch_regs = !(!self.scratch_regs | (1 << reg));
    }
    // endregion

    // region Label creation
    pub fn label_create(&mut self) -> u128 {
        self.label_count += 1;
        self.label_count
    }

    pub fn label_name(&mut self, label: &u128) -> String {
        return format!(".L{}", label);
    }
    //endregion

    pub fn scratch_regs(&self) -> &u8 { &self.scratch_regs }
}

impl Compiler {
    fn code_gen(&mut self, node: &Box<dyn Node>, w: &mut String) -> Option<u8> {
        if node.node_type() == NodeType::Statements {
            for n in node.as_any().downcast_ref::<StatementsNode>().unwrap().statement_nodes() {
                let reg = self.code_gen(n, w);
                if reg.is_some() {
                    self.free_scratch(reg.unwrap());
                }
            }
            return None;
        }

        if node.node_type() == NodeType::Number {
            let reg = self.res_scratch();
            writeln!(w, "\tMOVQ    {}, {}", node.as_any().downcast_ref::<NumberNode>().unwrap().get_number(), self.scratch_name(reg));
            return Some(reg);
        }

        if node.node_type() == NodeType::BinOp {
            let bin_op_node = node.as_any().downcast_ref::<BinOpNode>().unwrap();

            let left_reg = self.code_gen(bin_op_node.left_node(), w).unwrap();
            let right_reg = self.code_gen(bin_op_node.right_node(), w).unwrap();

            let mut res_reg = left_reg;

            if bin_op_node.op_token().token_type() == TokenType::Plus {
                writeln!(w, "\tADDQ    {}, {}", self.scratch_name(left_reg), self.scratch_name(right_reg));
            } else if bin_op_node.op_token().token_type() == TokenType::Minus {
                writeln!(w, "\tSUBQ    {}, {}", self.scratch_name(left_reg), self.scratch_name(right_reg));
            } else if bin_op_node.op_token().token_type() == TokenType::Mul {
                writeln!(w, "\tMOVQ    {}, %rax", self.scratch_name(left_reg));
                writeln!(w, "\tIMUL    {}", self.scratch_name(right_reg));
                res_reg = self.res_scratch();
                writeln!(w, "\tMOVQ    %rax, {}", self.scratch_name(res_reg));
                self.free_scratch(left_reg);
            } else {
                panic!("Token '{:?}' not supported as a binary operation yet!", bin_op_node.op_token().token_type());
            }


            self.free_scratch(right_reg);

            return Some(res_reg);
        }

        if node.node_type() == NodeType::FunctionDef {
            let func_def_node = node.as_any().downcast_ref::<FunctionDefinitionNode>().unwrap();
            let func_label = self.label_create();

            writeln!(w, "{}", self.label_name(&func_label));
            self.code_gen(func_def_node.body_node(), w);

            return None
        }


        None
    }


    pub fn compile_to_str(&mut self, node: &Box<dyn Node>) -> String {
        let mut res = String::new();
        self.code_gen(node, &mut res);
        res
    }
}