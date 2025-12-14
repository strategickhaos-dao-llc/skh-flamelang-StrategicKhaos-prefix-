/// FlameLang v2.0.0 Compiler CLI
/// Sovereign compiler toolchain with multi-dimensional transformation pipeline
use anyhow::Result;
use clap::{Parser, Subcommand};
use flamelang_lexer::Lexer;
use flamelang_transform::TransformPipeline;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "flamelang")]
#[command(about = "FlameLang v2.0.0 Compiler", long_about = None)]
#[command(version = "2.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a FlameLang source file
    Compile {
        /// Input source file
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Show transformation stages
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Lex a source file and show tokens
    Lex {
        /// Input source file
        #[arg(short, long)]
        input: PathBuf,
        
        /// Show Hebrew transformations
        #[arg(short = 'H', long)]
        show_hebrew: bool,
    },
    
    /// Transform source through the pipeline
    Transform {
        /// Input source file
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output stage: unicode, wave, or dna
        #[arg(short, long, default_value = "dna")]
        stage: String,
        
        /// Output format: json or text
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    
    /// Show version and pipeline information
    Info,
}

fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Compile { input, output, verbose } => {
            compile_file(input, output, verbose)?;
        }
        Commands::Lex { input, show_hebrew } => {
            lex_file(input, show_hebrew)?;
        }
        Commands::Transform { input, stage, format } => {
            transform_file(input, &stage, &format)?;
        }
        Commands::Info => {
            show_info();
        }
    }
    
    Ok(())
}

fn compile_file(input: PathBuf, output: Option<PathBuf>, verbose: bool) -> Result<()> {
    println!("🔥 FlameLang v2.0.0 Compiler");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Read source file
    let source = fs::read_to_string(&input)?;
    println!("📖 Reading: {}", input.display());
    
    // Layer 1: Lexical analysis with English→Hebrew transformation
    println!("\n🔤 Layer 1: Lexical Analysis (English→Hebrew)");
    let mut lexer = Lexer::new(source.clone());
    let tokens = lexer.tokenize()?;
    println!("   Tokens generated: {}", tokens.len());
    
    if verbose {
        for (i, token) in tokens.iter().take(10).enumerate() {
            print!("   [{}] {:?}", i, token.kind);
            if let Some(hebrew) = &token.hebrew_form {
                print!(" ({})", hebrew);
            }
            println!();
        }
        if tokens.len() > 10 {
            println!("   ... and {} more tokens", tokens.len() - 10);
        }
    }
    
    // Layers 2-4: Transformation pipeline
    println!("\n🌊 Layers 2-4: Transformation Pipeline");
    println!("   Layer 2: Hebrew→Unicode normalization");
    println!("   Layer 3: Unicode→Wave physics transform");
    println!("   Layer 4: Wave→DNA biological encoding");
    
    let mut pipeline = TransformPipeline::new();
    let dna_sequence = pipeline.execute(source)?;
    
    println!("   DNA sequence generated: {} bases, {} codons", 
             dna_sequence.bases.len(), 
             dna_sequence.codons.len());
    
    if verbose {
        println!("\n   First 20 bases: {}", 
                 dna_sequence.bases.iter()
                     .take(20)
                     .map(|b| format!("{}", b))
                     .collect::<String>());
    }
    
    // Layer 5: Code generation (DNA→LLVM)
    println!("\n⚙️  Layer 5: Code Generation (DNA→LLVM)");
    println!("   [Not yet implemented - requires LLVM backend]");
    
    let output_path = output.unwrap_or_else(|| {
        let mut p = input.clone();
        p.set_extension("ll");
        p
    });
    
    println!("\n✅ Compilation complete!");
    println!("   Output: {} (placeholder)", output_path.display());
    
    Ok(())
}

fn lex_file(input: PathBuf, show_hebrew: bool) -> Result<()> {
    let source = fs::read_to_string(&input)?;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    
    println!("🔤 Lexical Analysis Results");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("File: {}", input.display());
    println!("Tokens: {}\n", tokens.len());
    
    for (i, token) in tokens.iter().enumerate() {
        print!("[{:4}] {:20} \"{}\"", i, format!("{:?}", token.kind), token.lexeme);
        
        if show_hebrew {
            if let Some(hebrew) = &token.hebrew_form {
                print!(" → {}", hebrew);
            }
        }
        
        println!(" @ {}:{}", token.span.line, token.span.column);
    }
    
    Ok(())
}

fn transform_file(input: PathBuf, stage: &str, format: &str) -> Result<()> {
    let source = fs::read_to_string(&input)?;
    let mut pipeline = TransformPipeline::new();
    let dna_sequence = pipeline.execute(source)?;
    
    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&dna_sequence)?;
            println!("{}", json);
        }
        "text" => {
            match stage {
                "dna" => {
                    println!("DNA Sequence ({} bases, {} codons):", 
                             dna_sequence.bases.len(), 
                             dna_sequence.codons.len());
                    println!("\nBases:");
                    println!("{}", dna_sequence);
                    println!("\nCodons:");
                    for (i, codon) in dna_sequence.codons.iter().enumerate() {
                        print!("{} ", codon);
                        if (i + 1) % 20 == 0 {
                            println!();
                        }
                    }
                    println!();
                }
                _ => {
                    println!("Stage '{}' not supported for text output", stage);
                }
            }
        }
        _ => {
            println!("Unknown format: {}", format);
        }
    }
    
    Ok(())
}

fn show_info() {
    println!("🔥 FlameLang v2.0.0 Compiler Information");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Sovereign compiler toolchain with multi-dimensional transformation pipeline");
    println!();
    println!("📊 Transformation Pipeline (5 Layers):");
    println!();
    println!("  Layer 1: English → Hebrew");
    println!("           Linguistic transformation of keywords and identifiers");
    println!();
    println!("  Layer 2: Hebrew → Unicode");
    println!("           Unicode normalization (NFC) for consistent representation");
    println!();
    println!("  Layer 3: Unicode → Wave");
    println!("           Physics-validated dimensional analysis");
    println!("           Properties: frequency, amplitude, phase, wavelength, energy");
    println!();
    println!("  Layer 4: Wave → DNA");
    println!("           Biological encoding using DNA base sequences");
    println!("           Bases: A (Adenine), T (Thymine), C (Cytosine), G (Guanine)");
    println!();
    println!("  Layer 5: DNA → LLVM");
    println!("           Machine code generation via LLVM backend");
    println!();
    println!("🔬 Native Language Features:");
    println!("  • Quantum primitives: quantum, superpose, entangle, measure");
    println!("  • Wave primitives: wave, frequency, amplitude, phase");
    println!("  • DNA primitives: dna, encode, decode, sequence");
    println!();
    println!("🏢 Maintained by: StrategicKhaos DAO LLC");
    println!("📦 Part of: Khaos Catalyst Swarm Intelligence");
    println!();
}
