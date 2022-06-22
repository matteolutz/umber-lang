use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

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
    let now = Instant::now();

    let file = Path::new("E:\\Coding\\Assembly Projects\\umber-output\\test.ub");

    let asm_path = file.parent().unwrap().join(format!("{}.asm", file.file_stem().unwrap().to_str().unwrap()));

    println!("Opening file: {}", file.to_str().unwrap());
    let file_contents = fs::read_to_string(&file).expect("Failed to read file");

    /*println!("Preprocessing file...");
    let (preprocessed, preprocess_error) = preprocessor::preprocess(file.to_str().unwrap(), file_contents, &vec![
        "E:\\Coding\\Umber\\include\\"
    ], &mut vec![], &mut HashMap::new());

    if let Some(error) = preprocess_error {
        panic!("{}", error);
    }*/

    // println!("preprocessed: {}", preprocessed.as_ref().unwrap());

    println!("Lexing file...");
    let mut lexer = umber_lang::lexer::Lexer::new(file.to_path_buf(), file_contents);
    let (tokens, error) = lexer.make_tokens();

    if let Some(error) = error {
        eprintln!("{}", error);
        return;
    }

    println!("Parsing file...");

    let included_paths = vec![
        "E:\\Coding\\Umber\\include\\".to_string()
    ];
    let mut macros = HashMap::new();
    let mut already_included: Vec<PathBuf> = vec![];

    let mut parser = umber_lang::parser::Parser::new(tokens, &included_paths, &mut macros, &mut already_included);

    let (root_node, parse_error) = parser.parse();

    if let Some(error) = parse_error {
        eprintln!("{}", error);
        return;
    }

    let mut ast_root = root_node.as_ref().unwrap();

    println!("Validating file...");
    let mut validator = umber_lang::semantics::Validator::new();
    let validation_res = validator.validate(ast_root);

    if let Some(error) = validation_res.error() {
        eprintln!("{}", error);
        return;
    }

    ast_root = validation_res.node().as_ref().unwrap();

    println!("Compiling file...");
    let mut compiler = umber_lang::compiler::Compiler::new();
    let asm = compiler.compile_to_str(ast_root);

    if asm_path.exists() {
        fs::remove_file(&asm_path).expect("Failed to remove file");
    }
    fs::write(&asm_path, asm).expect("Failed to write file");

    println!("Done! Took: {:.2}s", now.elapsed().as_secs_f64());
}