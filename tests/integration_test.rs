use std::time::Instant;

use umber_lang;
use umber_lang::compiler::Compiler;
use umber_lang::lexer::Lexer;
use umber_lang::parser::Parser;
use umber_lang::semantics::Validator;

static TEXT_TO_LEX: &'static str = "\

fun print(a: number): void {
    asm__(\"
    MOVQ    $1, %rax,
    MOVQ    $0, %rdi,
    MOVQ    $123, %rsi
    MOVQ    %rdx, 4
    SYSCALL
    \");
    return;
};

fun add(a: number, b: number, c: number, d: number): number {
    asm__(\"
        MOVQ rax, 1234
        \");
    return 1234;
};

fun main(b: bool): number {
    return 1+1+add(1, 2, 3, 4);
};

";

#[test]
pub fn test_complete() {
    println!("starting!");

    let mut l = Lexer::new("Test", TEXT_TO_LEX);

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

    assert!(!validation_res.has_error());

    println!("semantics ok!");

    let mut compiler = Compiler::new();
    let asm = compiler.compile_to_str(parse_res.node().as_ref().unwrap());
    println!("asm:\n\n{}", asm);
}