//! Merge module - Da Vinci / Yin-Yang synthesis of multiple HTML sources
//! The core innovation of VesselMirror

use scraper::{Html, Selector};
use std::fs;
use std::path::Path;
use sha2::{Sha256, Digest};

/// Synthesis modes inspired by Renaissance masters
#[derive(Debug, Clone, Copy)]
pub enum SynthesisMode {
    /// Yin-Yang: Balance structure from A with aesthetics from B
    YinYang,
    /// Da Vinci: Golden ratio composition, anatomical precision
    DaVinci,
    /// Fibonacci: Spiral layout, natural growth patterns
    Fibonacci,
    /// Golden: 1.618 ratio applied to all proportions
    Golden,
}

impl From<&str> for SynthesisMode {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "yinyang" | "yin-yang" => SynthesisMode::YinYang,
            "davinci" | "da-vinci" | "leonardo" => SynthesisMode::DaVinci,
            "fibonacci" | "spiral" => SynthesisMode::Fibonacci,
            "golden" | "phi" => SynthesisMode::Golden,
            _ => SynthesisMode::DaVinci,
        }
    }
}

pub fn run(
    sources: &[std::path::PathBuf],
    synthesis: &str,
    output: &Path,
    structure_from: usize,
    style_from: usize,
) {
    if sources.len() < 2 {
        eprintln!("  ├─ ❌ Need at least 2 sources for synthesis");
        return;
    }
    
    let mode = SynthesisMode::from(synthesis);
    println!("  ├─ Mode: {:?}", mode);
    
    // Load sources
    let mut documents: Vec<Html> = Vec::new();
    let mut raw_sources: Vec<String> = Vec::new();
    
    for (i, path) in sources.iter().enumerate() {
        let content = fs::read_to_string(path).expect("Failed to read source");
        println!("  ├─ Source {}: {:?} ({} bytes)", i, path, content.len());
        raw_sources.push(content.clone());
        documents.push(Html::parse_document(&content));
    }
    
    // Extract components based on mode
    // Clamp indices to valid range
    let structure_doc = &documents[structure_from.min(documents.len() - 1)];
    let style_doc = &documents[style_from.min(documents.len() - 1)];
    
    let merged = match mode {
        SynthesisMode::YinYang => synthesize_yinyang(structure_doc, style_doc),
        SynthesisMode::DaVinci => synthesize_davinci(&documents),
        SynthesisMode::Fibonacci => synthesize_fibonacci(&documents),
        SynthesisMode::Golden => synthesize_golden(&documents),
    };
    
    // Compute synthesis hash
    let mut hasher = Sha256::new();
    for src in &raw_sources {
        hasher.update(src.as_bytes());
    }
    hasher.update(merged.as_bytes());
    let hash = hex::encode(hasher.finalize());
    
    println!("  ├─ Synthesis hash: {}", &hash[..16]);
    
    // Write output
    fs::write(output, &merged).expect("Failed to write merged output");
    println!("  └─ ✅ Merged: {:?} ({} bytes)", output, merged.len());
}

/// Yin-Yang synthesis: Structure from A, Style from B
fn synthesize_yinyang(structure: &Html, style: &Html) -> String {
    // Extract body structure from first source
    let body_selector = Selector::parse("body").unwrap();
    let structure_body = structure.select(&body_selector).next()
        .map(|e| e.inner_html())
        .unwrap_or_default();
    
    // Extract styles from second source
    let style_selector = Selector::parse("style").unwrap();
    
    let mut styles = String::new();
    for element in style.select(&style_selector) {
        styles.push_str(&element.inner_html());
        styles.push('\n');
    }
    
    // Combine
    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>VesselMirror Synthesis (Yin-Yang)</title>
    <meta name="generator" content="VesselMirror 0.1.0 - Strategickhaos DAO LLC">
    <style>
/* === YIN-YANG SYNTHESIS === */
/* Style source applied to structure source */
{}
    </style>
</head>
<body>
{}
</body>
</html>"#, styles, structure_body)
}

/// Da Vinci synthesis: Golden ratio composition, anatomical precision
fn synthesize_davinci(documents: &[Html]) -> String {
    const PHI: f64 = 1.618033988749895;
    
    // Extract all unique CSS classes and their rules
    let mut all_styles = String::new();
    let mut all_content = String::new();
    
    for (i, doc) in documents.iter().enumerate() {
        // Extract styles
        let style_selector = Selector::parse("style").unwrap();
        for element in doc.select(&style_selector) {
            all_styles.push_str(&format!("/* Source {} */\n", i));
            all_styles.push_str(&element.inner_html());
            all_styles.push('\n');
        }
        
        // Extract main content areas
        for selector_str in &["main", "article", ".content", "#content", ".container"] {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in doc.select(&selector) {
                    all_content.push_str(&format!("\n<!-- Source {} ({}) -->\n", i, selector_str));
                    all_content.push_str(&element.inner_html());
                }
            }
        }
    }
    
    // Apply golden ratio layout
    let golden_layout = format!(r#"
/* === DA VINCI GOLDEN RATIO LAYOUT === */
:root {{
    --phi: {};
    --phi-inverse: {};
    --golden-small: calc(100% / (1 + var(--phi)));
    --golden-large: calc(100% - var(--golden-small));
}}

.davinci-grid {{
    display: grid;
    grid-template-columns: var(--golden-large) var(--golden-small);
    gap: calc(1rem * var(--phi-inverse));
}}

.davinci-section {{
    padding: calc(1rem * var(--phi));
}}
"#, PHI, 1.0 / PHI);
    
    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>VesselMirror Synthesis (Da Vinci)</title>
    <meta name="generator" content="VesselMirror 0.1.0 - Strategickhaos DAO LLC">
    <style>
{}
{}
    </style>
</head>
<body>
<div class="davinci-grid">
    <main class="davinci-section">
{}
    </main>
    <aside class="davinci-section">
        <p>Synthesized by VesselMirror using Golden Ratio (φ = 1.618)</p>
    </aside>
</div>
</body>
</html>"#, golden_layout, all_styles, all_content)
}

/// Fibonacci synthesis: Spiral layout, natural growth
fn synthesize_fibonacci(documents: &[Html]) -> String {
    const FIB: [u32; 10] = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    
    let mut sections = String::new();
    
    for (i, doc) in documents.iter().enumerate() {
        let body_selector = Selector::parse("body").unwrap();
        if let Some(body) = doc.select(&body_selector).next() {
            let fib_size = FIB[i.min(FIB.len() - 1)];
            sections.push_str(&format!(
                r#"<section class="fib-section" style="flex: {};">{}</section>"#,
                fib_size,
                body.inner_html()
            ));
        }
    }
    
    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>VesselMirror Synthesis (Fibonacci)</title>
    <meta name="generator" content="VesselMirror 0.1.0 - Strategickhaos DAO LLC">
    <style>
/* === FIBONACCI SPIRAL LAYOUT === */
.fib-container {{
    display: flex;
    flex-wrap: wrap;
}}
.fib-section {{
    padding: 1rem;
    border: 1px solid #333;
}}
    </style>
</head>
<body>
<div class="fib-container">
{}
</div>
</body>
</html>"#, sections)
}

/// Golden synthesis: Pure φ ratio everywhere
fn synthesize_golden(documents: &[Html]) -> String {
    // Similar to DaVinci but more aggressive golden ratio application
    let davinci_result = synthesize_davinci(documents);
    
    // Replace specific CSS class names and title only
    davinci_result
        .replace("class=\"davinci-grid\"", "class=\"golden-grid\"")
        .replace("class=\"davinci-section\"", "class=\"golden-section\"")
        .replace("<title>VesselMirror Synthesis (Da Vinci)</title>", "<title>VesselMirror Synthesis (Golden Ratio)</title>")
        .replace("/* === DA VINCI GOLDEN RATIO LAYOUT === */", "/* === GOLDEN RATIO LAYOUT === */")
        .replace(".davinci-grid {", ".golden-grid {")
        .replace(".davinci-section {", ".golden-section {")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_synthesis_mode_parsing() {
        assert!(matches!(SynthesisMode::from("yinyang"), SynthesisMode::YinYang));
        assert!(matches!(SynthesisMode::from("davinci"), SynthesisMode::DaVinci));
        assert!(matches!(SynthesisMode::from("Leonardo"), SynthesisMode::DaVinci));
    }
}
