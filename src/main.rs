use std::any::Any;
use std::time::Instant;
use umber_lang::lexer::Lexer;
use umber_lang::parser::Parser;
use umber_lang::{compiler, runtime, semantics};
use umber_lang::semantics::Validator;
use umber_lang::symboltable::SymbolTable;

static TEXT_TO_LEX: &'static str = "\

fun main(b: number[1]): return {
    return;
};

fun b(a: number): void {
    return;
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

    /*let validation_res = semantics::validate(parse_res.node().as_ref().unwrap(), &mut SymbolTable::new(None));
    if validation_res.has_error() {
        panic!("semantic validation error: {}", validation_res.error().as_ref().unwrap());
    }

    println!("semantics ok!");*/

    let mut validator = Validator::new();

    let validation_res = validator.validate(parse_res.node().as_ref().unwrap());

    if validation_res.has_error() {
        panic!("semantic validation error: {}", validation_res.error().as_ref().unwrap());
    }

    println!("semantics ok!");

}
