# Gemini API Integration

FlameLang now includes integration with Google's Vertex AI Gemini API, allowing you to leverage AI-powered explanations and code generation directly from the FlameLang toolchain.

## Overview

The Gemini API integration provides access to Google's state-of-the-art AI models for natural language understanding and generation. This can be used for:

- Generating code explanations
- AI-assisted code completion
- Natural language to code translation
- Documentation generation

## Getting Started

### 1. Get Your API Key

To use the Gemini API, you need an API key from Google Cloud:

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Navigate to **Settings** → **API Keys**
3. Click **Create API Key**
4. Copy your API key

### 2. Set Environment Variable

Set your API key as an environment variable:

```bash
export GEMINI_API_KEY="your-api-key-here"
```

### 3. Using the API

#### Basic Example

```rust
use flamelang::ai::GeminiClient;

#[tokio::main]
async fn main() {
    // Create client from environment variable
    let client = GeminiClient::from_env().unwrap();
    
    // Generate content
    let response = client.generate_content("Explain how AI works in a few words").await.unwrap();
    
    println!("Response: {}", response);
}
```

#### Synchronous API

For blocking/synchronous contexts:

```rust
use flamelang::ai::GeminiClient;

fn main() {
    let client = GeminiClient::from_env().unwrap();
    let response = client.generate_content_blocking("Explain how AI works").unwrap();
    println!("Response: {}", response);
}
```

#### Custom Configuration

```rust
use flamelang::ai::GeminiClient;

#[tokio::main]
async fn main() {
    // Create client with custom configuration
    let client = GeminiClient::new(
        "your-api-key".to_string(),
        "gemini-2.5-flash-lite".to_string()
    );
    
    let response = client.generate_content("Your prompt here").await.unwrap();
    println!("{}", response);
}
```

## Running the Example

An example program is provided in the `examples` directory:

```bash
# Set your API key
export GEMINI_API_KEY="your-api-key-here"

# Run the example
cargo run --example gemini_api_example
```

## API Reference

### `GeminiClient`

The main client for interacting with the Gemini API.

#### Methods

- `new(api_key: String, model: String) -> Self`  
  Create a new client with explicit configuration.

- `from_env() -> Result<Self, String>`  
  Create a client using the `GEMINI_API_KEY` environment variable.

- `async generate_content(&self, prompt: &str) -> Result<String, String>`  
  Generate content asynchronously.

- `generate_content_blocking(&self, prompt: &str) -> Result<String, String>`  
  Generate content synchronously (blocking).

## cURL Example

You can also call the API directly using cURL:

```bash
curl "https://aiplatform.googleapis.com/v1/publishers/google/models/gemini-2.5-flash-lite:streamGenerateContent?key=${GEMINI_API_KEY}" \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "contents": [
      {
        "role": "user",
        "parts": [
          {
            "text": "Explain how AI works in a few words"
          }
        ]
      }
    ]
  }'
```

## Available Models

The default model is `gemini-2.5-flash-lite`, which provides fast responses with good quality. Other available models include:

- `gemini-2.5-flash-lite` - Fast, lightweight model (default)
- `gemini-2.5-flash` - Balanced performance and quality
- `gemini-2.5-pro` - Highest quality, slower responses

To use a different model, specify it when creating the client:

```rust
let client = GeminiClient::new(api_key, "gemini-2.5-pro".to_string());
```

## Error Handling

The API methods return `Result<String, String>` for easy error handling:

```rust
match client.generate_content("prompt").await {
    Ok(response) => println!("Success: {}", response),
    Err(error) => eprintln!("Error: {}", error),
}
```

Common errors include:
- Invalid API key
- Rate limiting
- Network connectivity issues
- Invalid model name

## Security Best Practices

1. **Never hardcode API keys** in your source code
2. **Use environment variables** to store sensitive credentials
3. **Restrict API key permissions** in Google Cloud Console
4. **Rotate keys regularly** for security
5. **Monitor API usage** to detect anomalies

## Rate Limits and Quotas

Be aware of the API rate limits:
- Free tier: Limited requests per minute
- Paid tier: Higher limits based on your plan

Check your quota in the [Google Cloud Console](https://console.cloud.google.com/apis/dashboard).

## Troubleshooting

### "GEMINI_API_KEY environment variable not set"

Set the environment variable before running:
```bash
export GEMINI_API_KEY="your-key"
```

### "API request failed with status: 401"

Your API key is invalid or expired. Generate a new one from the Cloud Console.

### "API request failed with status: 429"

You've exceeded the rate limit. Wait a moment and try again, or upgrade your quota.

## License

This integration is part of FlameLang and is licensed under the MIT License.

© 2025 Strategickhaos DAO LLC
