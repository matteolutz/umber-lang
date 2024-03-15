use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use umber_lang;
use umber_lang::error;
use umber_lang::error::Error;
use umber_lang::position::Position;

#[test]
pub fn test_file() -> Result<(), Error> {
    let now = Instant::now();

    let file = Path::new("E:\\Coding\\Assembly Projects\\umber-output\\test.ub");

    let asm_path = file.parent().unwrap().join(format!(
        "{}.asm",
        file.file_stem().unwrap().to_str().unwrap()
    ));

    println!("Opening file: {}", file.to_str().unwrap());
    let file_contents = fs::read_to_string(&file).expect("Failed to read file");

    println!("Lexing file...");
    let mut lexer = umber_lang::lexer::Lexer::new(file.to_path_buf(), file_contents);
    let tokens = lexer.make_tokens()?;

    println!("Parsing file...");

    let included_paths = vec!["E:\\Coding\\Umber\\include\\".to_string()];
    let mut macros = HashMap::new();
    let mut already_included: Vec<PathBuf> = vec![];

    let mut parser = umber_lang::parser::Parser::new(
        tokens,
        &included_paths,
        &mut macros,
        &mut already_included,
    );

    let mut ast_root = &parser.parse()?;

    println!("Validating file...");
    let mut validator = umber_lang::semantics::Validator::new();
    let validation_res = validator.validate(ast_root);

    if let Some(error) = validation_res.error() {
        return Err(error.clone());
    }

    ast_root = validation_res.node().as_ref().unwrap();

    println!("Compiling file...");
    let mut compiler = umber_lang::compiler::Compiler::new();
    let asm = compiler.compile_to_str(ast_root, false);

    if let Err(fmt_error) = asm {
        return Err(error::io_error(
            Position::empty(),
            Position::empty(),
            format!("Could not format assembly: {}", fmt_error).as_str(),
        ));
    }

    if asm_path.exists() {
        fs::remove_file(&asm_path).expect("Failed to remove file");
    }
    fs::write(&asm_path, asm.unwrap()).expect("Failed to write file");

    println!("Done! Took: {:.2}s", now.elapsed().as_secs_f64());

    Ok(())
}
