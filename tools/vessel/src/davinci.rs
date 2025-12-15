//! Da Vinci Synthesis - Golden ratio and Fibonacci-based composition

use scraper::Html;

/// Golden ratio constant (φ)
/// The golden ratio, also known as phi (φ), is an irrational number approximately equal to 1.618.
/// It appears throughout nature and is used for creating harmonious proportions.
/// Calculated as (1 + √5) / 2 with 15 decimal places of precision.
pub const PHI: f64 = 1.618033988749895;

/// Fibonacci sequence up to 21
pub const FIBONACCI: &[usize] = &[0, 1, 1, 2, 3, 5, 8, 13, 21];

/// Apply golden ratio proportions to layout
pub fn golden_section_grid(html: &str) -> String {
    // Parse HTML
    let _document = Html::parse_document(html);
    
    // Extract structure (simplified)
    let mut result = html.to_string();
    
    // Add golden ratio CSS variables
    let golden_css = format!(
        r#"
<style>
:root {{
    --golden-ratio: {};
    --golden-small: calc(100% / (1 + var(--golden-ratio)));
    --golden-large: calc(100% * var(--golden-ratio) / (1 + var(--golden-ratio)));
    --phi: {};
}}

/* Golden section grid */
.vessel-golden-container {{
    display: grid;
    grid-template-columns: var(--golden-large) var(--golden-small);
    gap: calc(1rem * var(--golden-ratio));
}}

.vessel-golden-reverse {{
    grid-template-columns: var(--golden-small) var(--golden-large);
}}

/* Fibonacci spacing */
.vessel-fib-1 {{ margin: 1px; }}
.vessel-fib-2 {{ margin: 2px; }}
.vessel-fib-3 {{ margin: 3px; }}
.vessel-fib-5 {{ margin: 5px; }}
.vessel-fib-8 {{ margin: 8px; }}
.vessel-fib-13 {{ margin: 13px; }}
.vessel-fib-21 {{ margin: 21px; }}
</style>
"#,
        PHI, PHI
    );
    
    // Insert golden CSS before closing head tag
    if let Some(pos) = result.find("</head>") {
        result.insert_str(pos, &golden_css);
    } else if let Some(pos) = result.find("<body") {
        result.insert_str(pos, &golden_css);
    }
    
    result
}

/// Extract harmonic colors based on golden ratio
pub fn harmonic_colors(css: &str) -> String {
    let mut result = css.to_string();
    
    // Add harmonic color palette based on golden ratio
    let harmonic_palette = r#"
/* Harmonic color palette (golden ratio hue rotation) */
:root {
    --vessel-primary: hsl(0, 70%, 50%);
    --vessel-secondary: hsl(222, 70%, 50%);  /* 360° / φ ≈ 222° */
    --vessel-tertiary: hsl(137, 70%, 50%);   /* 222° / φ ≈ 137° */
    --vessel-accent: hsl(85, 70%, 50%);      /* 137° / φ ≈ 85° */
}
"#;
    
    result.push_str(harmonic_palette);
    result
}

/// Apply Fibonacci sequence to animations
pub fn fibonacci_animation(js: &str) -> String {
    let mut result = js.to_string();
    
    // Add Fibonacci timing utilities
    let fib_js = r#"
// Fibonacci animation timing
const vesselFibonacci = [0, 1, 1, 2, 3, 5, 8, 13, 21];

function vesselFibDelay(index) {
    return vesselFibonacci[Math.min(index, vesselFibonacci.length - 1)] * 100;
}

function vesselFibDuration(index) {
    return vesselFibonacci[Math.min(index, vesselFibonacci.length - 1)] * 50;
}

function vesselAnimateSequence(elements, callback) {
    elements.forEach((el, i) => {
        setTimeout(() => {
            callback(el, i);
        }, vesselFibDelay(i));
    });
}
"#;
    
    result.push_str(fib_js);
    result
}

/// Calculate golden ratio between two values
pub fn golden_ratio_split(total: f64) -> (f64, f64) {
    let larger = total * PHI / (1.0 + PHI);
    let smaller = total - larger;
    (larger, smaller)
}

/// Get Fibonacci value at index
pub fn fibonacci_at(index: usize) -> usize {
    if index < FIBONACCI.len() {
        FIBONACCI[index]
    } else {
        // Calculate if beyond precomputed sequence
        let mut a = FIBONACCI[FIBONACCI.len() - 2];
        let mut b = FIBONACCI[FIBONACCI.len() - 1];
        for _ in FIBONACCI.len()..=index {
            let next = a + b;
            a = b;
            b = next;
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_golden_ratio() {
        let (larger, smaller) = golden_ratio_split(100.0);
        assert!((larger / smaller - PHI).abs() < 0.01);
    }
    
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci_at(0), 0);
        assert_eq!(fibonacci_at(5), 5);
        assert_eq!(fibonacci_at(7), 13);
    }
}
