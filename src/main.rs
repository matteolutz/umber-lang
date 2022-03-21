use std::time::Instant;
use umber_lang::compiler::Compiler;

use umber_lang::lexer::Lexer;
use umber_lang::parser::Parser;
use umber_lang::semantics::Validator;

static TEXT_TO_LEX: &'static str = "\

fun main(b: bool): number {
    1+1;
};


";

fn main() {
    let mut l = Lexer::new("Test", TEXT_TO_LEX);

    let start_time = Instant::now();

    let (tokens, error) = l.make_tokens();

    if error.is_some() {
        panic!("Couldn't make tokens: {}", error.unwrap());
    }

    let mut parser = Parser::new(tokens);
    let parse_res = parser.parse();

    if parse_res.has_error() {
        panic!("parser err: {}!", parse_res.error().as_ref().unwrap());
    }

    println!("took: {}ms", start_time.elapsed().as_millis());
    println!("node: {}", parse_res.node().as_ref().unwrap());

    let mut validator = Validator::new();

    let validation_res = validator.validate(parse_res.node().as_ref().unwrap());

    if validation_res.has_error() {
        panic!("semantic validation error: {}", validation_res.error().as_ref().unwrap());
    }

    println!("semantics ok!");

    let mut compiler = Compiler::new();
    let asm = compiler.compile_to_str(parse_res.node().as_ref().unwrap());
    println!("asm:\n\n{}", asm);
}
