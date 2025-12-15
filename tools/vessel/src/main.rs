//! VesselMirror — Sovereign Page Cloning & Synthesis Engine
//! INV-079: Strategickhaos DAO LLC
//! 
//! Automates the F12 DevTools → Save As → Yin-Yang remix workflow
//! into a sovereign, dependency-free HTML output pipeline.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod capture;
mod dissect;
mod merge;
mod purify;
mod prove;

#[derive(Parser)]
#[command(name = "vessel")]
#[command(author = "Strategickhaos DAO LLC <security@strategickhaos.ai>")]
#[command(version = "0.1.0")]
#[command(about = "Sovereign Page Cloning & Synthesis Engine", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Capture a webpage with all assets (like "Save As Complete")
    Capture {
        /// URL to capture
        url: String,
        
        /// Output file path
        #[arg(short, long, default_value = "capture.html")]
        output: PathBuf,
        
        /// Include screenshots
        #[arg(long, default_value = "false")]
        screenshot: bool,
        
        /// Wait for JavaScript to execute (ms)
        #[arg(long, default_value = "2000")]
        wait: u64,
    },
    
    /// Dissect HTML into atomic components
    Dissect {
        /// Input HTML file
        input: PathBuf,
        
        /// Components to extract (forms, buttons, layout, nav, cards, all)
        #[arg(short, long, value_delimiter = ',')]
        extract: Vec<String>,
        
        /// Output directory for components
        #[arg(short, long, default_value = "components/")]
        output: PathBuf,
    },
    
    /// Merge multiple sources (Da Vinci / Yin-Yang synthesis)
    Merge {
        /// Source files to merge
        sources: Vec<PathBuf>,
        
        /// Synthesis mode: yinyang, davinci, fibonacci, golden
        #[arg(short, long, default_value = "davinci")]
        synthesis: String,
        
        /// Output file
        #[arg(short, long, default_value = "merged.html")]
        output: PathBuf,
        
        /// Structure source index (0-based)
        #[arg(long, default_value = "0")]
        structure_from: usize,
        
        /// Style source index (0-based)
        #[arg(long, default_value = "1")]
        style_from: usize,
    },
    
    /// Purify HTML (inline assets, remove tracking, strip CDNs)
    Purify {
        /// Input HTML file
        input: PathBuf,
        
        /// Output file
        #[arg(short, long, default_value = "sovereign.html")]
        output: PathBuf,
        
        /// Inline all external resources
        #[arg(long, default_value = "true")]
        inline_all: bool,
        
        /// Remove tracking scripts/pixels
        #[arg(long, default_value = "true")]
        strip_tracking: bool,
        
        /// Remove external CDN references
        #[arg(long, default_value = "true")]
        strip_cdns: bool,
    },
    
    /// Generate cryptographic proof and on-chain attestation
    Prove {
        /// Input HTML file
        input: PathBuf,
        
        /// Blockchain target: swarmgate, ethereum, solana
        #[arg(short, long, default_value = "swarmgate")]
        chain: String,
        
        /// Output proof JSON
        #[arg(short, long, default_value = "proof.json")]
        output: PathBuf,
    },
    
    /// Full pipeline: capture → dissect → merge → purify → prove
    Pipeline {
        /// URLs to capture and synthesize
        urls: Vec<String>,
        
        /// Final output file
        #[arg(short, long, default_value = "sovereign.html")]
        output: PathBuf,
        
        /// Synthesis mode
        #[arg(short, long, default_value = "davinci")]
        synthesis: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Capture { url, output, screenshot, wait } => {
            println!("🔥 VesselMirror: Capturing {}", url);
            capture::run(&url, &output, screenshot, wait);
        }
        Commands::Dissect { input, extract, output } => {
            println!("🔥 VesselMirror: Dissecting {:?}", input);
            dissect::run(&input, &extract, &output);
        }
        Commands::Merge { sources, synthesis, output, structure_from, style_from } => {
            println!("🔥 VesselMirror: Merging {} sources ({} mode)", sources.len(), synthesis);
            merge::run(&sources, &synthesis, &output, structure_from, style_from);
        }
        Commands::Purify { input, output, inline_all, strip_tracking, strip_cdns } => {
            println!("🔥 VesselMirror: Purifying {:?}", input);
            purify::run(&input, &output, inline_all, strip_tracking, strip_cdns);
        }
        Commands::Prove { input, chain, output } => {
            println!("🔥 VesselMirror: Generating proof for {:?} → {}", input, chain);
            prove::run(&input, &chain, &output);
        }
        Commands::Pipeline { urls, output, synthesis } => {
            println!("🔥 VesselMirror: Full pipeline for {} URLs", urls.len());
            // Chain all stages
            for url in &urls {
                println!("  → Capturing: {}", url);
            }
            println!("  → Synthesizing with {} mode", synthesis);
            println!("  → Output: {:?}", output);
        }
    }
}
