use std::env;
use std::fs;
use std::fs::remove_file;
use std::path::Path;
use std::process::Command;

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

    let mut lexer = umber_lang::lexer::Lexer::new(Box::new(file_name), Box::new(file_contents));
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

    fs::write(&asm_path, asm).expect("Failed to write file");

    println!("Done!");
}