//! FlameLang Formatter (flamefmt)

use clap::Parser;

#[derive(Parser)]
#[command(name = "flamefmt")]
#[command(about = "FlameLang code formatter", long_about = None)]
struct Cli {
    /// Source file to format
    source: String,
}

fn main() {
    let cli = Cli::parse();
    println!("🎨 FlameLang Formatter v2.0.0");
    println!("   Formatting: {}", cli.source);
}
