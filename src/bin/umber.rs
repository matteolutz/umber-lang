use clap::{Args, Parser, Subcommand};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::time::Instant;
use std::{env, fs};
use umber_lang::error;
use umber_lang::error::Error;
use umber_lang::position::Position;
use umber_lang::syscall::TargetObjectType;

#[derive(Subcommand)]
enum Subcommands {
    /// Compile a file to an executable binary.
    Com(SubCompile),
}

#[derive(Args)]
struct SubCompile {
    /// The file to compile.
    #[clap(value_parser)]
    name: Vec<String>,

    /// Options for the assembler.
    #[clap(short, long, value_parser)]
    asm: Option<String>,

    /// Options for the linker.
    #[clap(short, long, value_parser)]
    linker: Option<String>,

    /// Include paths seperated by ';'
    #[clap(short, long, value_parser)]
    include: Option<Vec<String>>,

    /// Show verbose output
    #[clap(short, long, action)]
    verbose: bool,

    /// Dont add entry point '_start'
    #[clap(short, long, action)]
    no_entry: bool,

    /// Only compile and assemble, dont link
    #[clap(short, long, action)]
    compile_only: bool,

    /// Object format to compile to
    #[clap(short, long, value_enum, default_value_t=TargetObjectType::X86_64)]
    target: TargetObjectType,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct BinaryArgs {
    #[clap(subcommand)]
    command: Subcommands,
}

fn compile(
    files: Vec<String>,
    include: Option<Vec<String>>,
    assembler_options: Option<String>,
    linker_options: Option<String>,
    verbose: bool,
    no_entry: bool,
    compile_only: bool,
    arch: TargetObjectType,
) -> Result<(), Error> {
    let now = Instant::now();

    let build_output = env::current_dir().unwrap().join("build");

    for file in files {
        if verbose {
            println!("Building for {:?}", arch)
        }

        let file = Path::new(file.as_str());
        let file_stem = file.file_stem().unwrap().to_str().unwrap();

        // let build_output = file.parent().unwrap().join("build");

        let asm_path = build_output.join(format!("{}.asm", file_stem));
        let obj_path = build_output.join(format!("{}.{}", file_stem, arch.object_file_extension()));
        let bin_path = build_output.join(file_stem);

        let file_read_res = fs::read_to_string(&file);
        if let Err(file_err) = file_read_res {
            return Err(error::io_error(
                Position::new(file.to_path_buf()),
                Position::new(file.to_path_buf()),
                format!("Could not read file: {}", file_err).as_str(),
            ));
        }

        if verbose {
            println!("Successfully read file: {}", file.to_str().unwrap())
        }
        let file_contents = file_read_res.unwrap();

        if verbose {
            print!("Lexing...")
        }
        let mut lexer = umber_lang::lexer::Lexer::new(file.to_path_buf(), file_contents);
        let tokens = lexer.make_tokens()?;
        if verbose {
            println!("Done")
        }

        let mut macros = HashMap::new();
        let mut already_included: Vec<PathBuf> = vec![];

        let mut include_paths = &vec![];
        if let Some(include) = include.as_ref() {
            include_paths = include;
        }

        if verbose {
            print!("Parsing...")
        }
        let mut parser = umber_lang::parser::Parser::new(
            tokens,
            include_paths,
            &mut macros,
            &mut already_included,
        );
        let mut ast_root = &parser.parse()?;
        if verbose {
            println!("Done")
        }

        if verbose {
            print!("Validating...")
        }
        let mut validator = umber_lang::semantics::Validator::new();
        let validation_res = validator.validate(ast_root);

        if let Some(error) = validation_res.error() {
            return Err(error.clone());
        }

        ast_root = validation_res.node().as_ref().unwrap();

        if verbose {
            println!("Done")
        }

        if verbose {
            print!("Generating assembly...")
        }
        let mut compiler = umber_lang::compiler::Compiler::new(arch);
        let asm = compiler.compile_to_str(ast_root, no_entry, arch);

        if let Err(fmt_error) = asm {
            return Err(error::io_error(
                Position::empty(),
                Position::empty(),
                format!("Could not format assembly: {}", fmt_error).as_str(),
            ));
        }

        if verbose {
            println!("Done")
        }

        if !build_output.exists() || !build_output.is_dir() {
            if let Err(fs_error) = fs::create_dir(&build_output) {
                return Err(error::io_error(
                    Position::new(file.to_path_buf()),
                    Position::new(file.to_path_buf()),
                    format!("Could not create build directory: {}", fs_error).as_str(),
                ));
            }
        }

        if let Err(fs_error) = fs::write(&asm_path, asm.unwrap()) {
            return Err(error::io_error(
                Position::new(file.to_path_buf()),
                Position::new(file.to_path_buf()),
                format!("Could not format assembly: {}", fs_error).as_str(),
            ));
        }

        if verbose {
            print!("Compiling assembly...")
        }

        let mut assembler_cmd = Command::new("nasm");
        assembler_cmd.args([
            "-f",
            arch.object_format(),
            "-o",
            obj_path.to_str().unwrap(),
            asm_path.to_str().unwrap(),
        ]);

        if let Some(ao) = assembler_options.as_ref() {
            assembler_cmd.args(ao.split(' '));
        }

        let assembler_cmd_output = assembler_cmd.output();

        if let Err(nasm_err) = assembler_cmd_output {
            return Err(error::io_error(
                Position::new(file.to_path_buf()),
                Position::new(file.to_path_buf()),
                format!("Failed to execute 'nasm'-command: {}", nasm_err).as_str(),
            ));
        }

        if !assembler_cmd_output.as_ref().unwrap().status.success() {
            return Err(error::io_error_with_parent(
                Position::new(file.to_path_buf()),
                Position::new(file.to_path_buf()),
                "Assembling (NASM) failed with non-zero exit code",
                error::io_error(
                    Position::new(file.to_path_buf()),
                    Position::new(file.to_path_buf()),
                    format!(
                        "\n\"{}\"",
                        String::from_utf8_lossy(&*assembler_cmd_output.unwrap().stderr)
                    )
                    .as_str(),
                ),
            ));
        }

        if verbose {
            println!("Done")
        }

        if !compile_only {
            if verbose {
                print!("Linking...")
            }

            let mut linker_cmd = Command::new("ld");
            linker_cmd.args(["-o", bin_path.to_str().unwrap(), obj_path.to_str().unwrap()]);

            if let Some(lo) = linker_options.as_ref() {
                linker_cmd.args(lo.split(' '));
            }

            let linker_cmd_output = linker_cmd.output();

            if let Err(linker_err) = linker_cmd_output {
                return Err(error::io_error(
                    Position::new(file.to_path_buf()),
                    Position::new(file.to_path_buf()),
                    format!("Failed to run 'ld'-command: {}", linker_err).as_str(),
                ));
            }

            if !linker_cmd_output.as_ref().unwrap().status.success() {
                return Err(error::io_error_with_parent(
                    Position::new(file.to_path_buf()),
                    Position::new(file.to_path_buf()),
                    "Linking (ld) failed with non-zero exit code",
                    error::io_error(
                        Position::new(file.to_path_buf()),
                        Position::new(file.to_path_buf()),
                        format!(
                            "\n\"{}\"",
                            String::from_utf8_lossy(&*linker_cmd_output.unwrap().stderr)
                        )
                        .as_str(),
                    ),
                ));
            }
        }

        if verbose {
            println!("Done")
        }
    }

    println!("All done! Took: {}ms", now.elapsed().as_millis());

    Ok(())
}

fn main() {
    let args = BinaryArgs::parse();

    if let Err(err) = match args.command {
        Subcommands::Com(subcommand) => compile(
            subcommand.name,
            subcommand.include,
            subcommand.asm,
            subcommand.linker,
            subcommand.verbose,
            subcommand.no_entry,
            subcommand.compile_only,
            subcommand.target,
        ),
    } {
        println!("\n{}", err);
        exit(-1);
    }
}
