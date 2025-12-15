//! Stage 5: SOVEREIGN - Output clean, dependency-free HTML

use anyhow::{Context, Result};
use colored::*;
use regex::Regex;
use scraper::{Html, Selector};
use std::path::PathBuf;

/// Purify HTML by removing external dependencies
pub fn purify_page(
    file: &PathBuf,
    inline_all: bool,
    output: Option<PathBuf>,
) -> Result<()> {
    println!("{}", "  Stage 5: SOVEREIGN".bright_yellow().bold());
    
    let html = std::fs::read_to_string(file)
        .context("Failed to read HTML file")?;
    
    let mut purified = html.clone();
    
    // Remove tracking scripts
    purified = remove_tracking_scripts(&purified);
    println!("  {} Removed tracking scripts", "✓".bright_green());
    
    // Remove external CDN dependencies if inline_all is true
    if inline_all {
        purified = remove_cdn_links(&purified);
        println!("  {} Removed CDN dependencies", "✓".bright_green());
    }
    
    // Remove analytics and ads
    purified = remove_analytics(&purified);
    println!("  {} Removed analytics and ads", "✓".bright_green());
    
    // Add sovereignty metadata
    purified = add_sovereignty_metadata(&purified);
    
    // Write output
    let output_path = output.unwrap_or_else(|| {
        let mut path = file.clone();
        path.set_file_name(format!(
            "{}_sovereign.html",
            file.file_stem().unwrap().to_str().unwrap()
        ));
        path
    });
    
    std::fs::write(&output_path, &purified)
        .context("Failed to write purified HTML")?;
    
    println!("  {} Saved sovereign HTML: {}", "✓".bright_green(), output_path.display());
    
    Ok(())
}

fn remove_tracking_scripts(html: &str) -> String {
    let tracking_patterns = vec![
        r"google-analytics\.com",
        r"googletagmanager\.com",
        r"facebook\.net/\w+/fbevents\.js",
        r"analytics\.js",
        r"ga\.js",
        r"gtag\.js",
        r"tag\.js",
        r"doubleclick\.net",
        r"scorecardresearch\.com",
        r"quantserve\.com",
        r"hotjar\.com",
        r"mixpanel\.com",
        r"segment\.com",
    ];
    
    let document = Html::parse_document(html);
    let mut result = html.to_string();
    
    // Remove script tags with tracking URLs
    if let Ok(selector) = Selector::parse("script[src]") {
        for element in document.select(&selector) {
            if let Some(src) = element.value().attr("src") {
                for pattern in &tracking_patterns {
                    if Regex::new(pattern).unwrap().is_match(src) {
                        // Remove this script tag
                        let script_html = element.html();
                        result = result.replace(&script_html, "<!-- Removed tracking script -->");
                        break;
                    }
                }
            }
        }
    }
    
    // Remove inline tracking scripts
    let inline_tracking = vec![
        r"ga\('create'",
        r"gtag\('config'",
        r"fbq\('init'",
        r"_paq\.push",
    ];
    
    for pattern in inline_tracking {
        let re = Regex::new(&format!(r"<script[^>]*>.*?{}.*?</script>", pattern)).unwrap();
        result = re.replace_all(&result, "<!-- Removed inline tracking -->").to_string();
    }
    
    result
}

fn remove_cdn_links(html: &str) -> String {
    let cdn_patterns = vec![
        r"cdn\.jsdelivr\.net",
        r"cdnjs\.cloudflare\.com",
        r"unpkg\.com",
        r"code\.jquery\.com",
        r"maxcdn\.bootstrapcdn\.com",
        r"stackpath\.bootstrapcdn\.com",
        r"fonts\.googleapis\.com",
        r"fonts\.gstatic\.com",
    ];
    
    let document = Html::parse_document(html);
    let mut result = html.to_string();
    
    // Remove CDN script/link tags
    if let Ok(selector) = Selector::parse("script[src], link[href]") {
        for element in document.select(&selector) {
            let url = element
                .value()
                .attr("src")
                .or_else(|| element.value().attr("href"))
                .unwrap_or("");
            
            for pattern in &cdn_patterns {
                if Regex::new(pattern).unwrap().is_match(url) {
                    let tag_html = element.html();
                    result = result.replace(
                        &tag_html,
                        &format!("<!-- Removed CDN dependency: {} -->", url),
                    );
                    break;
                }
            }
        }
    }
    
    result
}

fn remove_analytics(html: &str) -> String {
    let mut result = html.to_string();
    
    // Remove common analytics iframes
    let iframe_re = Regex::new(r#"<iframe[^>]*(?:google|facebook|doubleclick)[^>]*>.*?</iframe>"#).unwrap();
    result = iframe_re.replace_all(&result, "<!-- Removed analytics iframe -->").to_string();
    
    // Remove noscript tracking pixels
    let noscript_re = Regex::new(r#"<noscript>.*?<img[^>]*(?:analytics|tracking|pixel)[^>]*>.*?</noscript>"#).unwrap();
    result = noscript_re.replace_all(&result, "<!-- Removed tracking pixel -->").to_string();
    
    result
}

fn add_sovereignty_metadata(html: &str) -> String {
    let sovereignty_comment = r#"
<!--
═══════════════════════════════════════════════════════════════════
    VESSELMIRROR SOVEREIGN DOCUMENT
    
    This document has been purified of external dependencies
    and tracking mechanisms to ensure full offline capability
    and user sovereignty.
    
    Generated by: VesselMirror v1.0.0
    Methodology: Ratio Ex Nihilo
    INV-079: Sovereign Web Capture & Synthesis Engine
    
    © 2025 Strategickhaos DAO LLC
═══════════════════════════════════════════════════════════════════
-->
"#;
    
    // Insert after <!DOCTYPE> or at the beginning
    if let Some(pos) = html.find("<html") {
        let mut result = html.to_string();
        result.insert_str(pos, sovereignty_comment);
        result
    } else {
        format!("{}{}", sovereignty_comment, html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_remove_tracking() {
        let html = r#"<script src="https://www.google-analytics.com/analytics.js"></script>"#;
        let result = remove_tracking_scripts(html);
        assert!(result.contains("Removed tracking script"));
    }
}
