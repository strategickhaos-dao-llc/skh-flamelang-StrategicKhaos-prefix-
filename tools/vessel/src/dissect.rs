//! Stage 2: DISSECT - Parse DOM into atomic components

use crate::types::{Component, ComponentMetadata, ComponentType};
use anyhow::{Context, Result};
use colored::*;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::path::PathBuf;

/// Dissect HTML into atomic components
pub fn dissect_page(
    file: &PathBuf,
    extract: Option<String>,
    output: Option<PathBuf>,
) -> Result<()> {
    println!("{}", "  Stage 2: DISSECT".bright_yellow().bold());
    
    let html = std::fs::read_to_string(file)
        .context("Failed to read HTML file")?;
    
    let document = Html::parse_document(&html);
    
    let default_extract = "forms,buttons,layout".to_string();
    let extract_str = extract.as_ref().unwrap_or(&default_extract);
    let extract_types = extract_str
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<_>>();
    
    let mut components = Vec::new();
    
    for extract_type in extract_types {
        match extract_type {
            "forms" => components.extend(extract_forms(&document)),
            "buttons" => components.extend(extract_buttons(&document)),
            "layout" => components.extend(extract_layout(&document)),
            "styles" => components.extend(extract_styles(&document)),
            "scripts" => components.extend(extract_scripts(&document)),
            _ => println!("  {} Unknown component type: {}", "⚠".bright_yellow(), extract_type),
        }
    }
    
    println!("  {} Extracted {} components", "✓".bright_green(), components.len());
    
    // Save components
    let output_dir = output.unwrap_or_else(|| PathBuf::from("vessel_components"));
    std::fs::create_dir_all(&output_dir)?;
    
    for (idx, component) in components.iter().enumerate() {
        let filename = format!("{:03}_{}.json", idx, component.id);
        let path = output_dir.join(filename);
        let json = serde_json::to_string_pretty(&component)?;
        std::fs::write(&path, json)?;
    }
    
    println!("  {} Saved to: {}", "✓".bright_green(), output_dir.display());
    
    Ok(())
}

fn extract_forms(document: &Html) -> Vec<Component> {
    let mut components = Vec::new();
    
    if let Ok(selector) = Selector::parse("form") {
        for (idx, element) in document.select(&selector).enumerate() {
            components.push(Component {
                id: format!("form_{}", idx),
                component_type: ComponentType::Form,
                html: element.html(),
                css: String::new(),
                js: None,
                metadata: ComponentMetadata {
                    selectors: vec!["form".to_string()],
                    classes: element
                        .value()
                        .classes()
                        .map(|s| s.to_string())
                        .collect(),
                    attributes: extract_attributes(&element),
                },
            });
        }
    }
    
    components
}

fn extract_buttons(document: &Html) -> Vec<Component> {
    let mut components = Vec::new();
    
    if let Ok(selector) = Selector::parse("button, input[type='button'], input[type='submit']") {
        for (idx, element) in document.select(&selector).enumerate() {
            components.push(Component {
                id: format!("button_{}", idx),
                component_type: ComponentType::Button,
                html: element.html(),
                css: String::new(),
                js: None,
                metadata: ComponentMetadata {
                    selectors: vec!["button".to_string()],
                    classes: element
                        .value()
                        .classes()
                        .map(|s| s.to_string())
                        .collect(),
                    attributes: extract_attributes(&element),
                },
            });
        }
    }
    
    components
}

fn extract_layout(document: &Html) -> Vec<Component> {
    let mut components = Vec::new();
    
    let layout_selectors = vec!["nav", "header", "main", "footer", "aside", "section", "article"];
    
    for selector_str in layout_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for (idx, element) in document.select(&selector).enumerate() {
                components.push(Component {
                    id: format!("{}_{}", selector_str, idx),
                    component_type: ComponentType::Layout,
                    html: element.html(),
                    css: String::new(),
                    js: None,
                    metadata: ComponentMetadata {
                        selectors: vec![selector_str.to_string()],
                        classes: element
                            .value()
                            .classes()
                            .map(|s| s.to_string())
                            .collect(),
                        attributes: extract_attributes(&element),
                    },
                });
            }
        }
    }
    
    components
}

fn extract_styles(document: &Html) -> Vec<Component> {
    let mut components = Vec::new();
    
    if let Ok(selector) = Selector::parse("style") {
        for (idx, element) in document.select(&selector).enumerate() {
            components.push(Component {
                id: format!("style_{}", idx),
                component_type: ComponentType::Style,
                html: element.html(),
                css: element.inner_html(),
                js: None,
                metadata: ComponentMetadata {
                    selectors: vec!["style".to_string()],
                    classes: Vec::new(),
                    attributes: extract_attributes(&element),
                },
            });
        }
    }
    
    components
}

fn extract_scripts(document: &Html) -> Vec<Component> {
    let mut components = Vec::new();
    
    if let Ok(selector) = Selector::parse("script") {
        for (idx, element) in document.select(&selector).enumerate() {
            components.push(Component {
                id: format!("script_{}", idx),
                component_type: ComponentType::Script,
                html: element.html(),
                css: String::new(),
                js: Some(element.inner_html()),
                metadata: ComponentMetadata {
                    selectors: vec!["script".to_string()],
                    classes: Vec::new(),
                    attributes: extract_attributes(&element),
                },
            });
        }
    }
    
    components
}

fn extract_attributes(element: &scraper::ElementRef) -> HashMap<String, String> {
    element
        .value()
        .attrs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}
