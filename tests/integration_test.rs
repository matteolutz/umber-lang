use std::time::Instant;

use umber_lang;
use umber_lang::compiler::Compiler;
use umber_lang::lexer::Lexer;
use umber_lang::parser::Parser;
use umber_lang::semantics::Validator;

static TEXT_TO_LEX: &'static str = "\

fun print_str(str: string, len: number): void {
    syscall 1, 0, str, len;
    return;
};

fun main(): number {

    let hello: string = \"Hello, World!\";
    let hello_len: number = 13;

    print_str(hello, hello_len);

    return 0;
};

";

#[test]
pub fn test_complete() {
    println!("starting!");

    let mut l = Lexer::new(Box::new("Test".to_string()), Box::new(TEXT_TO_LEX.to_string()));

    let start_time = Instant::now();

    let (tokens, error) = l.make_tokens();

    assert!(error.is_none());

    let mut parser = Parser::new(tokens);
    let parse_res = parser.parse();

    assert!(!parse_res.has_error());

    println!("took: {}ms", start_time.elapsed().as_millis());
    println!("node: {}", parse_res.node().as_ref().unwrap());

    let mut validator = Validator::new();

    let validation_res = validator.validate(parse_res.node().as_ref().unwrap());

    // println!("{}", validation_res.error().as_ref().unwrap());
    assert!(!validation_res.has_error());

    println!("semantics ok!");

    let mut compiler = Compiler::new();
    let asm = compiler.compile_to_str(parse_res.node().as_ref().unwrap());
    println!("asm:\n\n{}", asm);
}