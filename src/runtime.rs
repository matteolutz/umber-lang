use std::convert::TryInto;

const WORD_SIZE: usize = 4;

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

fn get_instruction(name: &str) -> Option<Instructions> {
    match name {
        "fetch" => Some(Instructions::Fetch),
        "store" => Some(Instructions::Store),
        "push" => Some(Instructions::Push),
        "add" => Some(Instructions::Add),
        "sub" => Some(Instructions::Sub),
        "mul" => Some(Instructions::Mul),
        "div" => Some(Instructions::Div),
        "mod" => Some(Instructions::Mod),
        "lt" => Some(Instructions::Lt),
        "gt" => Some(Instructions::Gt),
        "lte" => Some(Instructions::Lte),
        "gte" => Some(Instructions::Gte),
        "eq" => Some(Instructions::Eq),
        "ne" => Some(Instructions::Ne),
        "and" => Some(Instructions::And),
        "or" => Some(Instructions::Or),
        "not" => Some(Instructions::Not),
        "jmp" => Some(Instructions::Jmp),
        "jz" => Some(Instructions::Jz),
        "prtc" => Some(Instructions::Prtc),
        "prts" => Some(Instructions::Prts),
        "prti" => Some(Instructions::Prti),
        "halt" => Some(Instructions::Halt),
        _ => None
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

fn vm(code: &Vec<u8>, data_size: usize, _string_pool: Vec<String>) -> Vec<u32> {
    let mut stack: Vec<u32> = vec![0; data_size];
    let mut pc: usize = 0;

    loop {
        let len = stack.len();
        let op = code[pc];
        pc += 1;

        if op == Instructions::Fetch as u8 {} else if op == Instructions::Store as u8 {

        } else if op == Instructions::Push as u8 {
            stack.push(u32::from_ne_bytes(code[pc..pc+WORD_SIZE].try_into().unwrap()));
            pc += WORD_SIZE;
        } else if op == Instructions::Add as u8 {
            stack[len - 2] += stack[len - 1];
            stack.pop();
        } else if op == Instructions::Sub as u8 {
            stack[len - 2] -= stack[len - 1];
            stack.pop();
        } else if op == Instructions::Mul as u8 {
            stack[len - 2] *= stack[len - 1];
            stack.pop();
        } else if op == Instructions::Div as u8 {
            stack[len - 2] /= stack[len - 1];
            stack.pop();
        } else if op == Instructions::Mod as u8 {
            stack[len - 2] %= stack[len - 1];
            stack.pop();
        } else if op == Instructions::Eq as u8 {
            stack[len - 2] = (stack[len - 2] == stack[len - 1]) as u32;
            stack.pop();
        }


        else if op == Instructions::Halt as u8 {
            break;
        }
    }

    stack
}

pub fn run_code(code: &str) {
    let mut lines: Vec<&str> = code.trim().split('\n').collect();

    if lines.len() == 0 {
        return;
    }

    let conf_segments: Vec<&str> = lines[0].split(',').collect();
    lines.remove(0);

    let data_size: usize = conf_segments[0].parse::<usize>().unwrap();
    let _string_size: usize = conf_segments[1].parse::<usize>().unwrap();

    let mut code: Vec<u8> = vec![];
    for line in lines {
        let segms: Vec<&str> = line.split_whitespace().collect();

        let offset = segms[0].parse::<usize>().unwrap();
        let instr = segms[1];

        let op = get_instruction(instr);
        if op.is_none() {
            panic!("Unknown instruction {} at {}!", instr, offset);
        }
        let op_code = op.unwrap() as u8;

        emit_byte(&mut code, op_code as u8);
        if op_code == Instructions::Jmp as u8 || op_code == Instructions::Jz as u8 {
            let p = segms[3].parse::<usize>().unwrap();
            emit_word(&mut code, (p - (offset + 1)) as u32);
        } else if op_code == Instructions::Push as u8 {
            let value = segms[2].parse::<u32>().unwrap();
            emit_word(&mut code, value);
        } else if op_code == Instructions::Fetch as u8 || op_code == Instructions::Store as u8 {
            let value = segms[2].strip_prefix('[').unwrap().strip_suffix(']').unwrap().parse::<u32>().unwrap();
            emit_word(&mut code, value);
        }
    }

    println!("code:\n\n{:?}", code);
    println!("stack: \n\n{:?}", vm(&code, data_size, vec![]));
}