//! Stage 3: ANALYZE - Extract patterns, styles, structure

use crate::types::{AnalysisResult, Pattern, StructureAnalysis, StyleAnalysis};
use anyhow::{Context, Result};
use colored::*;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

// Pre-compiled regex patterns for efficiency
static HEX_COLOR_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"#[0-9a-fA-F]{3,6}").unwrap());
static RGB_COLOR_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"rgba?\([^)]+\)").unwrap());
static FONT_FAMILY_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"font-family:\s*([^;]+)").unwrap());

/// Analyze page structure and patterns
pub fn analyze_page(file: &PathBuf, verbose: bool) -> Result<()> {
    println!("{}", "  Stage 3: ANALYZE".bright_yellow().bold());
    
    let html = std::fs::read_to_string(file)
        .context("Failed to read HTML file")?;
    
    let document = Html::parse_document(&html);
    
    let structure = analyze_structure(&document);
    let styles = analyze_styles(&document);
    let patterns = find_patterns(&document);
    
    let result = AnalysisResult {
        structure,
        styles,
        patterns,
    };
    
    print_analysis(&result, verbose);
    
    Ok(())
}

fn analyze_structure(document: &Html) -> StructureAnalysis {
    let mut element_count = 0;
    let mut semantic_elements = HashSet::new();
    
    let semantic_tags = vec![
        "article", "aside", "details", "figcaption", "figure",
        "footer", "header", "main", "mark", "nav", "section",
        "summary", "time",
    ];
    
    for tag in &semantic_tags {
        if let Ok(selector) = Selector::parse(tag) {
            for _ in document.select(&selector) {
                semantic_elements.insert(tag.to_string());
            }
        }
    }
    
    // Count all elements
    if let Ok(selector) = Selector::parse("*") {
        element_count = document.select(&selector).count();
    }
    
    let max_depth = calculate_dom_depth(document);
    
    StructureAnalysis {
        dom_depth: max_depth,
        element_count,
        semantic_elements: semantic_elements.into_iter().collect(),
    }
}

fn calculate_dom_depth(document: &Html) -> usize {
    // Simplified depth calculation
    let mut max_depth = 0;
    
    if let Ok(selector) = Selector::parse("body *") {
        for element in document.select(&selector) {
            let mut depth = 0;
            let mut current = element.parent();
            
            while current.is_some() {
                depth += 1;
                current = current.and_then(|n| n.parent());
            }
            
            max_depth = max_depth.max(depth);
        }
    }
    
    max_depth
}

fn analyze_styles(document: &Html) -> StyleAnalysis {
    let mut color_palette = HashSet::new();
    let mut fonts = HashSet::new();
    let dimensions = HashMap::new();
    
    // Extract colors from style attributes and style tags
    if let Ok(selector) = Selector::parse("[style]") {
        for element in document.select(&selector) {
            if let Some(style) = element.value().attr("style") {
                extract_colors_from_style(style, &mut color_palette);
                extract_fonts_from_style(style, &mut fonts);
            }
        }
    }
    
    // Extract from style tags
    if let Ok(selector) = Selector::parse("style") {
        for element in document.select(&selector) {
            let css = element.inner_html();
            extract_colors_from_style(&css, &mut color_palette);
            extract_fonts_from_style(&css, &mut fonts);
        }
    }
    
    StyleAnalysis {
        color_palette: color_palette.into_iter().collect(),
        fonts: fonts.into_iter().collect(),
        dimensions,
    }
}

fn extract_colors_from_style(style: &str, colors: &mut HashSet<String>) {
    // Match hex colors
    for cap in HEX_COLOR_RE.find_iter(style) {
        colors.insert(cap.as_str().to_string());
    }
    
    // Match rgb/rgba colors
    for cap in RGB_COLOR_RE.find_iter(style) {
        colors.insert(cap.as_str().to_string());
    }
}

fn extract_fonts_from_style(style: &str, fonts: &mut HashSet<String>) {
    for cap in FONT_FAMILY_RE.captures_iter(style) {
        if let Some(font) = cap.get(1) {
            fonts.insert(font.as_str().trim().to_string());
        }
    }
}

/// Minimum number of occurrences for a pattern to be considered significant
const PATTERN_THRESHOLD: usize = 3;

fn find_patterns(document: &Html) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    
    // Find repeated class patterns
    let mut class_counts: HashMap<String, usize> = HashMap::new();
    
    if let Ok(selector) = Selector::parse("[class]") {
        for element in document.select(&selector) {
            for class in element.value().classes() {
                *class_counts.entry(class.to_string()).or_insert(0) += 1;
            }
        }
    }
    
    // Convert to patterns (only include classes used PATTERN_THRESHOLD+ times)
    for (class, count) in class_counts {
        if count >= PATTERN_THRESHOLD {
            patterns.push(Pattern {
                name: format!("class pattern: {}", class),
                occurrences: count,
                selector: format!(".{}", class),
            });
        }
    }
    
    patterns.sort_by(|a, b| b.occurrences.cmp(&a.occurrences));
    patterns
}

fn print_analysis(result: &AnalysisResult, verbose: bool) {
    println!("\n  📊 {}", "Structure Analysis".bright_cyan().bold());
    println!("     DOM depth: {}", result.structure.dom_depth);
    println!("     Element count: {}", result.structure.element_count);
    println!("     Semantic elements: {}", result.structure.semantic_elements.len());
    
    if verbose && !result.structure.semantic_elements.is_empty() {
        println!("     {}", result.structure.semantic_elements.join(", "));
    }
    
    println!("\n  🎨 {}", "Style Analysis".bright_cyan().bold());
    println!("     Color palette: {} colors", result.styles.color_palette.len());
    println!("     Fonts: {} families", result.styles.fonts.len());
    
    if verbose {
        if !result.styles.color_palette.is_empty() {
            println!("     Colors: {}", result.styles.color_palette.join(", "));
        }
        if !result.styles.fonts.is_empty() {
            println!("     Fonts: {}", result.styles.fonts.join(", "));
        }
    }
    
    println!("\n  🔍 {}", "Pattern Analysis".bright_cyan().bold());
    println!("     Found {} patterns", result.patterns.len());
    
    if verbose {
        for pattern in result.patterns.iter().take(10) {
            println!("     • {} ({} occurrences)", pattern.name, pattern.occurrences);
        }
    }
}
