//! Stage 4: SYNTHESIZE - Yin-Yang merge from multiple sources

use crate::davinci;
use anyhow::{Context, Result};
use colored::*;
use scraper::{Html, Selector};
use std::path::PathBuf;

/// Merge multiple HTML sources using specified synthesis method
pub fn merge_pages(
    sources: Vec<PathBuf>,
    synthesis: &str,
    output: Option<PathBuf>,
) -> Result<()> {
    println!("{}", "  Stage 4: SYNTHESIZE".bright_yellow().bold());
    
    if sources.len() < 2 {
        anyhow::bail!("Merge requires at least 2 source files, but only {} provided. Please specify 2 or more HTML files to merge.", sources.len());
    }
    
    // Read all source files
    let mut documents = Vec::new();
    for source in &sources {
        let html = std::fs::read_to_string(source)
            .with_context(|| format!("Failed to read {}", source.display()))?;
        documents.push(html);
    }
    
    println!("  {} Loaded {} source documents", "✓".bright_green(), documents.len());
    
    // Perform synthesis based on method
    let merged = match synthesis {
        "davinci" => davinci_synthesis(&documents)?,
        "golden" => golden_synthesis(&documents)?,
        "fibonacci" => fibonacci_synthesis(&documents)?,
        _ => anyhow::bail!("Unknown synthesis method: {}", synthesis),
    };
    
    // Write output
    let output_path = output.unwrap_or_else(|| PathBuf::from("vessel_merged.html"));
    std::fs::write(&output_path, &merged)
        .context("Failed to write merged HTML")?;
    
    println!("  {} Synthesized using {} method", "✓".bright_green(), synthesis);
    println!("  {} Saved to: {}", "✓".bright_green(), output_path.display());
    
    Ok(())
}

/// Da Vinci synthesis: Golden ratio + harmonic colors + Fibonacci timing
fn davinci_synthesis(documents: &[String]) -> Result<String> {
    println!("  {} Applying Da Vinci synthesis (Yin-Yang fusion)...", "→".bright_blue());
    
    // Extract components from each source
    let structure = extract_structure(&documents[0]);
    let styles = if documents.len() > 1 {
        extract_styles(&documents[1])
    } else {
        extract_styles(&documents[0])
    };
    let scripts = if documents.len() > 2 {
        extract_scripts(&documents[2])
    } else {
        String::new()
    };
    
    // Apply Da Vinci transformations
    let enhanced_structure = davinci::golden_section_grid(&structure);
    let harmonic_styles = davinci::harmonic_colors(&styles);
    let fibonacci_scripts = davinci::fibonacci_animation(&scripts);
    
    // Combine into unified document
    let merged = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="generator" content="VesselMirror v1.0.0 - Da Vinci Synthesis">
    <title>VesselMirror Synthesis</title>
    <style>
/* Da Vinci Synthesis - Harmonic Styles */
{}
    </style>
</head>
<body>
<!-- Da Vinci Synthesis - Golden Section Structure -->
{}

<script>
// Da Vinci Synthesis - Fibonacci Animations
{}
</script>
</body>
</html>"#,
        harmonic_styles, enhanced_structure, fibonacci_scripts
    );
    
    Ok(merged)
}

/// Golden synthesis: Focus on proportions and layout
fn golden_synthesis(documents: &[String]) -> Result<String> {
    println!("  {} Applying golden ratio synthesis...", "→".bright_blue());
    
    let structure = extract_structure(&documents[0]);
    let enhanced = davinci::golden_section_grid(&structure);
    
    let merged = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="generator" content="VesselMirror v1.0.0 - Golden Synthesis">
    <title>VesselMirror Golden</title>
</head>
<body>
{}
</body>
</html>"#,
        enhanced
    );
    
    Ok(merged)
}

/// Fibonacci synthesis: Focus on timing and sequences
fn fibonacci_synthesis(documents: &[String]) -> Result<String> {
    println!("  {} Applying Fibonacci synthesis...", "→".bright_blue());
    
    let structure = extract_structure(&documents[0]);
    let scripts = extract_scripts(&documents[0]);
    let fibonacci_scripts = davinci::fibonacci_animation(&scripts);
    
    let merged = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="generator" content="VesselMirror v1.0.0 - Fibonacci Synthesis">
    <title>VesselMirror Fibonacci</title>
</head>
<body>
{}

<script>
{}
</script>
</body>
</html>"#,
        structure, fibonacci_scripts
    );
    
    Ok(merged)
}

fn extract_structure(html: &str) -> String {
    let document = Html::parse_document(html);
    
    // Extract body content
    if let Ok(selector) = Selector::parse("body") {
        if let Some(body) = document.select(&selector).next() {
            return body.inner_html();
        }
    }
    
    html.to_string()
}

fn extract_styles(html: &str) -> String {
    let document = Html::parse_document(html);
    let mut styles = String::new();
    
    // Extract all style tags
    if let Ok(selector) = Selector::parse("style") {
        for element in document.select(&selector) {
            styles.push_str(&element.inner_html());
            styles.push('\n');
        }
    }
    
    // Extract inline styles (simplified)
    if let Ok(selector) = Selector::parse("[style]") {
        for element in document.select(&selector) {
            if let Some(style) = element.value().attr("style") {
                styles.push_str("/* Inline style */\n");
                styles.push_str(style);
                styles.push('\n');
            }
        }
    }
    
    styles
}

fn extract_scripts(html: &str) -> String {
    let document = Html::parse_document(html);
    let mut scripts = String::new();
    
    // Extract all script tags
    if let Ok(selector) = Selector::parse("script") {
        for element in document.select(&selector) {
            scripts.push_str(&element.inner_html());
            scripts.push('\n');
        }
    }
    
    scripts
}
