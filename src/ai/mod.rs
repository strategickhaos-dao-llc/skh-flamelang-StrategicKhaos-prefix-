//! AI Integration Module
//!
//! Provides integration with Google Vertex AI Gemini API for AI-powered explanations
//! and code generation assistance.

use serde::{Deserialize, Serialize};
use std::env;

/// Request structure for Gemini API
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
}

/// Content structure for a message
#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub role: String,
    pub parts: Vec<Part>,
}

/// Part of a message (text)
#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    pub text: String,
}

/// Response from Gemini API
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Option<Vec<Candidate>>,
    pub error: Option<ApiError>,
}

/// Candidate response
#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    pub content: Content,
}

/// API Error structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
    pub status: String,
}

/// Gemini API Client
pub struct GeminiClient {
    api_key: String,
    base_url: String,
    model: String,
}

impl GeminiClient {
    /// Create a new Gemini client
    ///
    /// # Arguments
    /// * `api_key` - The API key for authentication
    /// * `model` - The model to use (e.g., "gemini-2.5-flash-lite")
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            base_url: "https://aiplatform.googleapis.com/v1".to_string(),
            model,
        }
    }

    /// Create a client from environment variable
    ///
    /// Reads the API key from the `GEMINI_API_KEY` environment variable
    pub fn from_env() -> Result<Self, String> {
        let api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| "GEMINI_API_KEY environment variable not set".to_string())?;
        Ok(Self::new(api_key, "gemini-2.5-flash-lite".to_string()))
    }

    /// Generate content using the Gemini API
    ///
    /// # Arguments
    /// * `prompt` - The text prompt to send to the AI
    ///
    /// # Returns
    /// The generated text response or an error
    pub async fn generate_content(&self, prompt: &str) -> Result<String, String> {
        let url = format!(
            "{}/publishers/google/models/{}:streamGenerateContent?key={}",
            self.base_url, self.model, self.api_key
        );

        let request = GeminiRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "API request failed with status: {}",
                response.status()
            ));
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        if let Some(error) = gemini_response.error {
            return Err(format!("API error: {} - {}", error.code, error.message));
        }

        if let Some(candidates) = gemini_response.candidates {
            if let Some(candidate) = candidates.first() {
                if let Some(part) = candidate.content.parts.first() {
                    return Ok(part.text.clone());
                }
            }
        }

        Err("No response from API".to_string())
    }

    /// Synchronous version of generate_content
    ///
    /// # Arguments
    /// * `prompt` - The text prompt to send to the AI
    ///
    /// # Returns
    /// The generated text response or an error
    pub fn generate_content_blocking(&self, prompt: &str) -> Result<String, String> {
        let url = format!(
            "{}/publishers/google/models/{}:streamGenerateContent?key={}",
            self.base_url, self.model, self.api_key
        );

        let request = GeminiRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
        };

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "API request failed with status: {}",
                response.status()
            ));
        }

        let gemini_response: GeminiResponse = response
            .json()
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        if let Some(error) = gemini_response.error {
            return Err(format!("API error: {} - {}", error.code, error.message));
        }

        if let Some(candidates) = gemini_response.candidates {
            if let Some(candidate) = candidates.first() {
                if let Some(part) = candidate.content.parts.first() {
                    return Ok(part.text.clone());
                }
            }
        }

        Err("No response from API".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gemini_client_creation() {
        let client = GeminiClient::new(
            "test-api-key".to_string(),
            "gemini-2.5-flash-lite".to_string(),
        );
        assert_eq!(client.api_key, "test-api-key");
        assert_eq!(client.model, "gemini-2.5-flash-lite");
    }

    #[test]
    fn test_request_serialization() {
        let request = GeminiRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: "Test prompt".to_string(),
                }],
            }],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"role\":\"user\""));
        assert!(json.contains("\"text\":\"Test prompt\""));
    }
}
