//! Example: Using the Gemini API
//!
//! This example demonstrates how to use the Gemini API client to generate
//! AI-powered explanations.
//!
//! Usage:
//!   1. Set the GEMINI_API_KEY environment variable:
//!      export GEMINI_API_KEY="your-api-key-here"
//!
//!   2. Run the example:
//!      cargo run --example gemini_api_example

use flamelang::ai::GeminiClient;

#[tokio::main]
async fn main() {
    println!("🔥 FlameLang - Gemini API Example\n");

    // Create client from environment variable
    let client = match GeminiClient::from_env() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("\nPlease set the GEMINI_API_KEY environment variable:");
            eprintln!("  export GEMINI_API_KEY=\"your-api-key-here\"");
            eprintln!("\nGet your API key from: https://console.cloud.google.com/");
            std::process::exit(1);
        }
    };

    println!("✓ API client initialized");
    println!("📝 Sending prompt: \"Explain how AI works in a few words\"\n");

    // Generate content
    match client
        .generate_content("Explain how AI works in a few words")
        .await
    {
        Ok(response) => {
            println!("🤖 Response from Gemini:\n");
            println!("{}", response);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}
