use std::env;
use std::fs;
use std::fs::remove_file;
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        return;
    }

    let file_name = args[1].clone();
    let file = Path::new(file_name.as_str());

    let asm_path = file.parent().unwrap().join(format!("{}.asm", file.file_stem().unwrap().to_str().unwrap()));

    let file_contents = fs::read_to_string(&file).expect("Failed to read file");

    let (preprocessed, preprocess_error) = umber_lang::preprocessor::preprocess(file.to_str().unwrap(), file_contents, &vec![
        "/usr/bin/umber/include"
    ], &mut vec![], &mut HashMap::new());

    if let Some(error) = preprocess_error {
        panic!("{}", error);
    }

    let mut lexer = umber_lang::lexer::Lexer::new(file.to_path_buf(), preprocessed.unwrap());
    let (tokens, error) = lexer.make_tokens();

    if let Some(error) = error {
        eprintln!("{}", error);
        return;
    }

    let mut parser = umber_lang::parser::Parser::new(tokens, vec![
        "/usr/bin/umber/include".to_string()
    ]);
    let (root_node, parse_error) = parser.parse();

    if let Some(error) = parse_error {
        eprintln!("{}", error);
        return;
    }

    let ast_root = root_node.as_ref().unwrap();

    let mut validator = umber_lang::semantics::Validator::new();
    let validation_res = validator.validate(ast_root);

    if let Some(error) = validation_res.error() {
        eprintln!("{}", error);
        return;
    }

    let mut compiler = umber_lang::compiler::Compiler::new();
    let asm = compiler.compile_to_str(ast_root);

    fs::write(&asm_path, asm).expect("Failed to write file");

    println!("Done!");
}
