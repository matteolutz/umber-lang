use std::any::Any;
use std::time::Instant;
use umber_lang::context::Context;
use umber_lang::lexer::Lexer;
use umber_lang::parser::Parser;
use umber_lang::{compiler, runtime, semantics};

static TEXT_TO_LEX: &'static str = "\
fun a(b) {
    b;
    1;
};

fun main() {
    let test = 123;
    a(test);
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

    let validation_res = semantics::validate(parse_res.node().as_ref().unwrap(), &mut Context::new_with_symbol_table("Test", None, None));
    if validation_res.has_error() {
        panic!("semantic validation error: {}", validation_res.error().as_ref().unwrap());
    }

    println!("semantics ok!");

    /*let (code, string_pool, var_pool) = compiler::compile(parse_res.node().as_ref().unwrap());
    println!("code:\n{:?}", code);
    println!("string pool: {:?}", string_pool);
    println!("var pool: {:?}", var_pool);

    let virtual_bin = compiler::to_virtual_bin(&code, &string_pool, &var_pool);

    println!("virtual bin\n\n{}", virtual_bin);

    runtime::run_code(&virtual_bin);*/
    println!("asm:\n\n{}", compiler::to_asm(parse_res.node().as_ref().unwrap()));

}
