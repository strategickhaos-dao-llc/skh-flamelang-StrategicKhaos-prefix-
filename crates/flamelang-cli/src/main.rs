//! FlameLang Compiler CLI
//!
//! Command-line interface for the FlameLang v2.0.0 compiler toolchain

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "flamecc")]
#[command(version = "2.0.0")]
#[command(about = "FlameLang v2.0.0 Sovereign Compiler Toolchain", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile FlameLang source files
    Compile {
        /// Input source file
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Output file path
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Emit LLVM IR instead of object code
        #[arg(long)]
        emit_llvm: bool,

        /// Optimization level (0-3)
        #[arg(short = 'O', long, default_value = "0")]
        opt_level: u8,
    },

    /// Run a FlameLang program
    Run {
        /// Input source file
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Arguments to pass to the program
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Format FlameLang source code
    Fmt {
        /// Input source file
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Write output in-place
        #[arg(short, long)]
        in_place: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile {
            input,
            output,
            emit_llvm,
            opt_level,
        } => {
            compile_file(input, output, emit_llvm, opt_level)?;
        }
        Commands::Run { input, args } => {
            run_file(input, args)?;
        }
        Commands::Fmt { input, in_place } => {
            format_file(input, in_place)?;
        }
    }

    Ok(())
}

fn compile_file(
    input: PathBuf,
    output: Option<PathBuf>,
    emit_llvm: bool,
    opt_level: u8,
) -> anyhow::Result<()> {
    println!("Compiling: {:?}", input);
    println!("Output: {:?}", output.as_ref().unwrap_or(&PathBuf::from("a.out")));
    println!("Emit LLVM: {}", emit_llvm);
    println!("Optimization level: {}", opt_level);

    // Read source file
    let source = std::fs::read_to_string(&input)?;

    // Parse the source code
    let program = flamelang_parser::parse(&source)?;
    println!("Parsing completed successfully");

    // Lower AST to HIR
    let mut hir_lowering = flamelang_hir::HirLowering::new();
    let hir_program = hir_lowering.lower_program(&program)?;
    println!("HIR lowering completed successfully");

    // Lower HIR to MIR
    let mut mir_lowering = flamelang_mir::MirLowering::new();
    let mir_program = mir_lowering.lower_program(&hir_program)?;
    println!("MIR lowering completed successfully");

    // Generate LLVM IR
    let context = inkwell::context::Context::create();
    let module_name = input.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    
    let mut codegen = flamelang_codegen::CodeGen::new(&context, module_name);
    codegen.generate(&mir_program)?;
    println!("Code generation completed successfully");

    if emit_llvm {
        let llvm_ir = codegen.to_string();
        let output_path = output.unwrap_or_else(|| {
            let mut path = input.clone();
            path.set_extension("ll");
            path
        });
        std::fs::write(output_path, llvm_ir)?;
        println!("LLVM IR written successfully");
    } else {
        println!("Native compilation not yet implemented - use --emit-llvm for now");
    }

    Ok(())
}

fn run_file(input: PathBuf, args: Vec<String>) -> anyhow::Result<()> {
    println!("Running: {:?}", input);
    println!("Arguments: {:?}", args);

    // First compile the program
    compile_file(input.clone(), None, false, 2)?;

    println!("Execution not yet implemented");
    Ok(())
}

fn format_file(input: PathBuf, in_place: bool) -> anyhow::Result<()> {
    println!("Formatting: {:?}", input);
    println!("In-place: {}", in_place);

    let source = std::fs::read_to_string(&input)?;

    // Parse to validate syntax
    let _program = flamelang_parser::parse(&source)?;

    // For now, just output the source as-is (formatter not implemented)
    println!("Formatting not yet fully implemented");
    
    if !in_place {
        println!("{}", source);
    }

    Ok(())
}

