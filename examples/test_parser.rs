// Test parser with example files
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hello_source = fs::read_to_string("examples/hello.flame")?;
    println!("Parsing hello.flame...");
    
    let program = flamelang_parser::parse(&hello_source)?;
    println!("✓ Successfully parsed hello.flame");
    println!("  Items: {}", program.items.len());
    
    let fib_source = fs::read_to_string("examples/fibonacci.flame")?;
    println!("\nParsing fibonacci.flame...");
    
    let program = flamelang_parser::parse(&fib_source)?;
    println!("✓ Successfully parsed fibonacci.flame");
    println!("  Items: {}", program.items.len());
    
    Ok(())
}
