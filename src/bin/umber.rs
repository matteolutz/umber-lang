use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};
use std::collections::HashMap;
use std::time::Instant;
use clap::{Parser, Subcommand, Args};
use umber_lang::error;
use umber_lang::error::Error;
use umber_lang::position::Position;

#[derive(Subcommand)]
enum Subcommands {
    Com(SubCompile)
}

#[derive(Args)]
struct SubCompile {
    #[clap(value_parser)]
    name: String,

    #[clap(short, long, value_parser)]
    include: Option<String>
}


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct BinaryArgs {

    #[clap(subcommand)]
    command: Subcommands

}

fn compile(file: String, include: Option<String>) -> Result<(), Error> {
    let now = Instant::now();

    let file = Path::new(file.as_str());
    let file_stem = file.file_stem().unwrap().to_str().unwrap();

    let build_output = file.parent().unwrap().join("build");

    let asm_path = build_output.join(format!("{}.asm", file_stem));
    let obj_path = build_output.join(format!("{}.o", file_stem));
    let bin_path = build_output.join(file_stem);

    let file_read_res = fs::read_to_string(&file);
    if let Err(file_err) = file_read_res {
        return Err(error::io_error(Position::new(file.to_path_buf()), Position::new(file.to_path_buf()), format!("Could not read file: {}", file_err).as_str()));
    }

    println!("Successfully read file: {}", file.to_str().unwrap());
    let file_contents = file_read_res.unwrap();

    print!("Lexing...");
    let mut lexer = umber_lang::lexer::Lexer::new(file.to_path_buf(), file_contents);
    let tokens = lexer.make_tokens()?;
    println!("Done");

    let mut include_paths: Vec<String> = vec![];
    if let Some(ips) = include {
        include_paths = ips.split(';').map(|s| s.to_string()).collect();
    }

    let mut macros = HashMap::new();
    let mut already_included: Vec<PathBuf> = vec![];

    print!("Parsing...");
    let mut parser = umber_lang::parser::Parser::new(tokens, &include_paths, &mut macros, &mut already_included);
    let mut ast_root = &parser.parse()?;
    println!("Done");

    print!("Validating...");
    let mut validator = umber_lang::semantics::Validator::new();
    let validation_res = validator.validate(ast_root);

    if let Some(error) = validation_res.error() {
        return Err(error.clone())
    }

    ast_root = validation_res.node().as_ref().unwrap();

    println!("Done");

    print!("Generating assembly...");
    let mut compiler = umber_lang::compiler::Compiler::new();
    let asm = compiler.compile_to_str(ast_root);

    if let Err(fmt_error) = asm {
        return Err(error::io_error(Position::empty(), Position::empty(), format!("Could not format assembly: {}", fmt_error).as_str()));
    }

    println!("Done");

    if !build_output.exists() || !build_output.is_dir() {
        if let Err(fs_error) = fs::create_dir(&build_output) {
            return Err(error::io_error(Position::new(file.to_path_buf()), Position::new(file.to_path_buf()), format!("Could not create build directory: {}", fs_error).as_str()));
        }
    }

    if let Err(fs_error) = fs::write(&asm_path, asm.unwrap()) {
        return Err(error::io_error(Position::new(file.to_path_buf()), Position::new(file.to_path_buf()), format!("Could not format assembly: {}", fs_error).as_str()));
    }

    print!("Compiling assembly...");
    if let Err(nasm_err) = Command::new("nasm")
        .args(["-f", "elf64", "-o", obj_path.to_str().unwrap(), asm_path.to_str().unwrap()])
        .output() {
        return Err(error::io_error(Position::new(file.to_path_buf()), Position::new(file.to_path_buf()), format!("Failed to execute 'nasm'-command: {}", nasm_err).as_str()));
    }
    println!("Done");

    print!("Linking...");
    if let Err(linker_err) = Command::new("ld")
        .args(["-o", bin_path.to_str().unwrap(), obj_path.to_str().unwrap()])
        .output() {
        return Err(error::io_error(Position::new(file.to_path_buf()), Position::new(file.to_path_buf()), format!("Failed to run 'ld'-command: {}", linker_err).as_str()));
    }
    println!("Done");

    println!("All done! Took: {}ms", now.elapsed().as_millis());

    Ok(())
}

fn main() {
    let args = BinaryArgs::parse();

    if let Err(err) = match args.command {
        Subcommands::Com(subcommand) => {
            compile(subcommand.name, subcommand.include)
        }
    } {
        println!("\n{}", err);
        exit(-1);
    }
}
