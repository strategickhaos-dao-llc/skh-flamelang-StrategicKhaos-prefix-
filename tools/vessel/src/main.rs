//! VesselMirror - Sovereign Web Capture & Synthesis Engine
//! INV-079: Automate F12 DevTools → Save As → Yin-Yang remix workflow
//!
//! © 2025 Strategickhaos DAO LLC

mod capture;
mod dissect;
mod analyze;
mod synthesize;
mod sovereign;
mod prove;
mod types;
mod davinci;

use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "vessel")]
#[command(about = "🚢 VesselMirror - Sovereign web capture & synthesis engine", long_about = None)]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Capture a webpage with full HTML/CSS/JS/assets
    Capture {
        /// URL to capture
        url: String,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Include JavaScript execution (headless browser mode)
        #[arg(short, long, default_value = "false")]
        js: bool,
    },
    
    /// Dissect HTML into atomic components
    Dissect {
        /// Input HTML file
        file: PathBuf,
        
        /// Components to extract (forms,buttons,layout,styles,scripts)
        #[arg(short, long)]
        extract: Option<String>,
        
        /// Output directory for components
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Analyze page structure and patterns
    Analyze {
        /// Input HTML file
        file: PathBuf,
        
        /// Show detailed analysis
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Synthesize multiple sources (Yin-Yang merge)
    Merge {
        /// Source files to merge
        sources: Vec<PathBuf>,
        
        /// Synthesis mode (davinci, golden, fibonacci)
        #[arg(short, long, default_value = "davinci")]
        synthesis: String,
        
        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Purify HTML - strip external dependencies
    Purify {
        /// Input HTML file
        file: PathBuf,
        
        /// Inline all assets (CSS, JS, images)
        #[arg(short, long)]
        inline_all: bool,
        
        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Prove integrity with SHA-256 hash and attestation
    Prove {
        /// Input file to hash and attest
        file: PathBuf,
        
        /// Blockchain for attestation (swarmgate, ethereum, none)
        #[arg(short, long, default_value = "none")]
        chain: String,
        
        /// Show detailed proof
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Print banner
    print_banner();
    
    match cli.command {
        Commands::Capture { url, output, js } => {
            println!("{}", format!("📡 Capturing: {}", url).bright_cyan());
            capture::capture_page(&url, output, js)?;
        }
        
        Commands::Dissect { file, extract, output } => {
            println!("{}", format!("🔬 Dissecting: {}", file.display()).bright_cyan());
            dissect::dissect_page(&file, extract, output)?;
        }
        
        Commands::Analyze { file, verbose } => {
            println!("{}", format!("🔍 Analyzing: {}", file.display()).bright_cyan());
            analyze::analyze_page(&file, verbose)?;
        }
        
        Commands::Merge { sources, synthesis, output } => {
            println!("{}", format!("☯️  Merging {} sources with {} synthesis", sources.len(), synthesis).bright_cyan());
            synthesize::merge_pages(sources, &synthesis, output)?;
        }
        
        Commands::Purify { file, inline_all, output } => {
            println!("{}", format!("✨ Purifying: {}", file.display()).bright_cyan());
            sovereign::purify_page(&file, inline_all, output)?;
        }
        
        Commands::Prove { file, chain, verbose } => {
            println!("{}", format!("🔐 Proving: {}", file.display()).bright_cyan());
            prove::prove_integrity(&file, &chain, verbose)?;
        }
    }
    
    println!("\n{}", "✅ Operation complete".bright_green().bold());
    Ok(())
}

fn print_banner() {
    println!("{}", "┌─────────────────────────────────────────────────────────────────┐".bright_blue());
    println!("{}", "│                    🚢 VESSELMIRROR v1.0.0                       │".bright_blue().bold());
    println!("{}", "│              Sovereign Web Capture & Synthesis Engine           │".bright_blue());
    println!("{}", "│                      INV-079 | Ratio Ex Nihilo                  │".bright_blue());
    println!("{}", "└─────────────────────────────────────────────────────────────────┘".bright_blue());
    println!();
}
