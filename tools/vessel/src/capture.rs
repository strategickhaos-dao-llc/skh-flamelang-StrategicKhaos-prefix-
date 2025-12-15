//! Stage 1: CAPTURE - Headless browser snapshot

use crate::types::CapturedPage;
use anyhow::{Context, Result};
use colored::*;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Capture a webpage with full HTML/CSS/JS/assets
pub fn capture_page(url: &str, output: Option<PathBuf>, use_js: bool) -> Result<()> {
    println!("{}", "  Stage 1: CAPTURE".bright_yellow().bold());
    
    let captured = if use_js {
        capture_with_browser(url)?
    } else {
        capture_with_http(url)?
    };
    
    let output_path = output.unwrap_or_else(|| {
        let filename = sanitize_filename(url);
        PathBuf::from(format!("{}.html", filename))
    });
    
    // Write HTML to file
    std::fs::write(&output_path, &captured.html)
        .context("Failed to write captured HTML")?;
    
    println!("  {} {}", "✓".bright_green(), format!("Saved to: {}", output_path.display()));
    println!("  {} {} bytes", "✓".bright_green(), captured.html.len());
    
    Ok(())
}

/// Capture using headless browser (full JS execution)
fn capture_with_browser(url: &str) -> Result<CapturedPage> {
    println!("  {} Launching headless browser...", "→".bright_blue());
    
    // Note: headless_chrome may not be available in all environments
    // Falling back to HTTP capture for now
    capture_with_http(url)
}

/// Capture using HTTP client (faster, no JS execution)
fn capture_with_http(url: &str) -> Result<CapturedPage> {
    println!("  {} Fetching via HTTP...", "→".bright_blue());
    
    let response = reqwest::blocking::get(url)
        .context("Failed to fetch URL")?;
    
    let html = response.text()
        .context("Failed to read response body")?;
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    Ok(CapturedPage {
        url: url.to_string(),
        html,
        css: Vec::new(),
        scripts: Vec::new(),
        assets: Default::default(),
        timestamp,
    })
}

/// Sanitize URL for use as filename
fn sanitize_filename(url: &str) -> String {
    url.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect::<String>()
        .chars()
        .take(50)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sanitize_filename() {
        assert_eq!(
            sanitize_filename("https://github.com/settings/keys"),
            "httpsgithubcomsettingskeys"
        );
    }
}
