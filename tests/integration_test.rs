use std::collections::HashMap;
use std::fs;
use std::path::Path;

use umber_lang;
use umber_lang::compiler::Compiler;
use umber_lang::lexer::Lexer;
use umber_lang::nodes::{Node, NodeType};
use umber_lang::nodes::unaryop_node::UnaryOpNode;
use umber_lang::parser::Parser;
use umber_lang::preprocessor;
use umber_lang::semantics::Validator;

#[test]
pub fn test_file() {
    let file = Path::new("E:\\Coding\\Assembly Projects\\umber-output\\test.ub");

    let asm_path = file.parent().unwrap().join(format!("{}.asm", file.file_stem().unwrap().to_str().unwrap()));

    let file_contents = fs::read_to_string(&file).expect("Failed to read file");

    println!("Preprocessing file...");
    let (preprocessed, preprocess_error) = preprocessor::preprocess(file.to_str().unwrap(), file_contents, &vec![
        "E:\\Coding\\Umber\\include\\"
    ], &mut vec![], &mut HashMap::new());

    if let Some(error) = preprocess_error {
        panic!("{}", error);
    }

    // println!("preprocessed: {}", preprocessed.as_ref().unwrap());

    println!("Lexing file...");
    let mut lexer = umber_lang::lexer::Lexer::new(Box::new(file.to_str().unwrap().to_string()), Box::new(preprocessed.unwrap()));
    let (tokens, error) = lexer.make_tokens();

    if let Some(error) = error {
        eprintln!("{}", error);
        return;
    }

    println!("Parsing file...");
    let mut parser = umber_lang::parser::Parser::new(tokens);
    let parse_res = parser.parse();

    if let Some(error) = parse_res.error() {
        eprintln!("{}", error);
        return;
    }

    let ast_root = parse_res.node().as_ref().unwrap();

    println!("Validating file...");
    let mut validator = umber_lang::semantics::Validator::new();
    let validation_res = validator.validate(ast_root);

    if let Some(error) = validation_res.error() {
        eprintln!("{}", error);
        return;
    }

    println!("Compiling file...");
    let mut compiler = umber_lang::compiler::Compiler::new();
    let asm = compiler.compile_to_str(ast_root);

    if asm_path.exists() {
        fs::remove_file(&asm_path).expect("Failed to remove file");
    }
    fs::write(&asm_path, asm).expect("Failed to write file");

    println!("Done!");
}