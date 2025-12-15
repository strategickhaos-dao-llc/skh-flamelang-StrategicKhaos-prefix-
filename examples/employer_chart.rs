//! Example: Generate Employer Bar Chart with FlameViz
//!
//! This example demonstrates creating a relative frequency bar chart
//! from categorical data using FlameViz v1.0.

use flamelang::{FlameViz, CategoricalData, DataEntry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 FlameViz v1.0 - Employer Chart Example");
    println!("=========================================\n");

    // Create FlameViz instance
    let flameviz = FlameViz::new();

    // Example 1: Parse from text (Zybooks-style data)
    println!("Example 1: Parse from text");
    println!("--------------------------");
    let text_input = "Walmart Stores: 2300000, Amazon: 566000, Yum! Brands: 450000, Kroger: 435000";
    println!("Input: {}\n", text_input);

    let viz = flameviz.from_text(text_input)?;
    
    // Save SVG chart
    std::fs::write("employer_chart.svg", &viz.svg)?;
    println!("✓ SVG chart saved to: employer_chart.svg");
    
    // Display explanation
    println!("\nExplanation:");
    println!("{}", viz.explanation);
    
    // Display provenance hash
    println!("\nProvenance Hash (SHA-256):");
    println!("{}", viz.hash);
    
    // Display sonic fingerprint
    if let Some(audio) = &viz.audio_wav {
        println!("\nSonic Fingerprint:");
        println!("{}", audio);
    }
    
    // Display on-chain payload
    println!("\nOn-Chain Payload:");
    println!("{}", viz.on_chain_payload);

    println!("\n\n");

    // Example 2: Use structured data
    println!("Example 2: Structured data input");
    println!("----------------------------------");
    
    let data = CategoricalData {
        title: "Top 4 US Employers (2023)".to_string(),
        entries: vec![
            DataEntry { 
                label: "Walmart Stores".to_string(), 
                value: 2300000.0 
            },
            DataEntry { 
                label: "Amazon".to_string(), 
                value: 566000.0 
            },
            DataEntry { 
                label: "Yum! Brands".to_string(), 
                value: 450000.0 
            },
            DataEntry { 
                label: "Kroger".to_string(), 
                value: 435000.0 
            },
        ],
        unit: Some("employees".to_string()),
    };

    let viz2 = flameviz.from_data(&data)?;
    
    std::fs::write("employer_chart_structured.svg", &viz2.svg)?;
    println!("✓ SVG chart saved to: employer_chart_structured.svg");
    
    println!("\nData Summary:");
    println!("  Title: {}", data.title);
    println!("  Entries: {}", data.entries.len());
    println!("  Total: {}", data.entries.iter().map(|e| e.value).sum::<f64>());
    
    println!("\nVisualization Hash: {}", viz2.hash);

    println!("\n🖤🔥 FlameViz: Ancient text → modern data → sovereign visualization\n");

    Ok(())
}
