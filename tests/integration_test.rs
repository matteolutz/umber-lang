use std::fs;
use std::path::Path;
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
    let hello: string = \"Hello, World!secret message\";
    let mut hello_len: number = 13;

    print_str(hello, hello_len);

    hello_len = hello_len + 15;

    print_str(hello, hello_len);

    return 0;
};

";

#[test]
#[ignore]
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

#[test]
pub fn test_file() {
    let file = Path::new("E:\\Coding\\Assembly Projects\\umber-output\\test.ub");

    let asm_path = file.parent().unwrap().join(format!("{}.asm", file.file_stem().unwrap().to_str().unwrap()));

    let file_contents = fs::read_to_string(&file).expect("Failed to read file");

    let mut lexer = umber_lang::lexer::Lexer::new(Box::new("umber_lang test!".to_string()), Box::new(file_contents));
    let (tokens, error) = lexer.make_tokens();

    if let Some(error) = error {
        eprintln!("{}", error);
        return;
    }

    let mut parser = umber_lang::parser::Parser::new(tokens);
    let parse_res = parser.parse();

    if let Some(error) = parse_res.error() {
        eprintln!("{}", error);
        return;
    }

    let ast_root = parse_res.node().as_ref().unwrap();

    let mut validator = umber_lang::semantics::Validator::new();
    let validation_res = validator.validate(ast_root);

    if let Some(error) = validation_res.error() {
        eprintln!("{}", error);
        return;
    }

    let mut compiler = umber_lang::compiler::Compiler::new();
    let asm = compiler.compile_to_str(ast_root);

    if asm_path.exists() {
        fs::remove_file(&asm_path).expect("Failed to remove file");
    }
    fs::write(&asm_path, asm).expect("Failed to write file");

    println!("Done!");
}