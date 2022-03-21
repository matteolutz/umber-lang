use std::fmt::Write;

use crate::nodes::Node;

const GENERAL_REGS: [&str; 7] = [
    "rbx", "r10", "r11", "r12", "r13", "r14", "r15"
];

pub struct Compiler {
    gen_regs: u8,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            gen_regs: 0
        }
    }

    // region Register distribution
    pub fn res_reg(&mut self) -> u8 {
        if self.gen_regs ^ 0b1111111 == 0 {
            panic!("No free general purpose registers were found!");
        }

        let mut i = 0;
        let mut current = self.gen_regs;
        while current & 1 == 1 {
            i += 1;
            current >>= 1;
        }

        self.gen_regs |= 1 << i;

        i
    }

    pub fn free_reg(&mut self, reg: u8) {
        self.gen_regs = !(!self.gen_regs | (1 << reg));
    }
    // endregion

    pub fn gen_regs(&self) -> &u8 { &self.gen_regs }
}

impl Compiler {
    fn code_gen() -> u8 {
        0
    }


    pub fn compile_to_str(node: &Box<dyn Node>) -> String {
        "".to_string()
    }
}