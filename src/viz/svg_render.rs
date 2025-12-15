//! SVG Rendering Engine for Bar Charts

use crate::{FlameError, FlameResult};
use super::types::{CategoricalData, VisualizationMode};

/// SVG canvas dimensions
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const MARGIN: usize = 80;
const BAR_PADDING: usize = 10;

/// Render categorical data as SVG bar chart
pub fn render_svg(data: &CategoricalData, mode: VisualizationMode) -> FlameResult<String> {
    match mode {
        VisualizationMode::Vertical => render_vertical(data),
        VisualizationMode::Horizontal => render_horizontal(data),
        VisualizationMode::Grouped => render_grouped(data),
        VisualizationMode::Stacked => render_stacked(data),
    }
}

/// Render vertical bar chart
fn render_vertical(data: &CategoricalData) -> FlameResult<String> {
    if data.points.is_empty() {
        return Err(FlameError::Transform {
            layer: 5,
            message: "Cannot render empty data".to_string(),
        });
    }

    let mut svg = String::new();
    
    // SVG header
    svg.push_str(&format!(
        r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
        WIDTH, HEIGHT
    ));
    svg.push('\n');

    // Background
    svg.push_str(&format!(
        r##"  <rect width="{}" height="{}" fill="#ffffff"/>"##,
        WIDTH, HEIGHT
    ));
    svg.push('\n');

    // Title
    svg.push_str(&format!(
        r#"  <text x="{}" y="30" text-anchor="middle" font-size="20" font-weight="bold">{}</text>"#,
        WIDTH / 2,
        escape_xml(&data.title)
    ));
    svg.push('\n');

    // Calculate dimensions
    let chart_width = WIDTH - 2 * MARGIN;
    let chart_height = HEIGHT - 2 * MARGIN;
    let num_bars = data.points.len();
    let bar_width = (chart_width / num_bars).saturating_sub(BAR_PADDING);
    
    let max_value = data.max_value();
    let scale = if max_value > 0.0 {
        chart_height as f64 / max_value
    } else {
        1.0
    };

    // Draw axes
    svg.push_str(&format!(
        r##"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#000" stroke-width="2"/>"##,
        MARGIN, HEIGHT - MARGIN,
        WIDTH - MARGIN, HEIGHT - MARGIN
    ));
    svg.push('\n');
    
    svg.push_str(&format!(
        r##"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#000" stroke-width="2"/>"##,
        MARGIN, MARGIN,
        MARGIN, HEIGHT - MARGIN
    ));
    svg.push('\n');

    // Draw bars
    for (i, point) in data.points.iter().enumerate() {
        let x = MARGIN + i * (bar_width + BAR_PADDING);
        let bar_height = (point.value * scale) as usize;
        let y = HEIGHT - MARGIN - bar_height;

        // Bar
        svg.push_str(&format!(
            r##"  <rect x="{}" y="{}" width="{}" height="{}" fill="#4a90e2" stroke="#2c5aa0" stroke-width="1"/>"##,
            x, y, bar_width, bar_height
        ));
        svg.push('\n');

        // Value label on top of bar
        let value_label = if data.is_relative {
            format!("{:.1}%", point.value)
        } else if point.value >= 1_000_000.0 {
            format!("{:.1}M", point.value / 1_000_000.0)
        } else if point.value >= 1_000.0 {
            format!("{:.1}K", point.value / 1_000.0)
        } else {
            format!("{:.0}", point.value)
        };

        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" text-anchor="middle" font-size="12">{}</text>"#,
            x + bar_width / 2,
            y.saturating_sub(5),
            value_label
        ));
        svg.push('\n');

        // Label below bar with optional glyph
        let label_with_glyph = if let Some(glyph) = point.glyph {
            format!("{} {}", glyph, point.label)
        } else {
            point.label.clone()
        };

        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" text-anchor="middle" font-size="12" transform="rotate(-45 {} {})">{}</text>"#,
            x + bar_width / 2,
            HEIGHT - MARGIN + 20,
            x + bar_width / 2,
            HEIGHT - MARGIN + 20,
            escape_xml(&label_with_glyph)
        ));
        svg.push('\n');
    }

    // Y-axis label
    svg.push_str(&format!(
        r#"  <text x="{}" y="{}" text-anchor="middle" font-size="14" transform="rotate(-90 {} {})">Value</text>"#,
        MARGIN - 40,
        HEIGHT / 2,
        MARGIN - 40,
        HEIGHT / 2
    ));
    svg.push('\n');

    svg.push_str("</svg>\n");
    Ok(svg)
}

/// Render horizontal bar chart (for long labels)
fn render_horizontal(data: &CategoricalData) -> FlameResult<String> {
    if data.points.is_empty() {
        return Err(FlameError::Transform {
            layer: 5,
            message: "Cannot render empty data".to_string(),
        });
    }

    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
        WIDTH, HEIGHT
    ));
    svg.push('\n');

    svg.push_str(&format!(
        r##"  <rect width="{}" height="{}" fill="#ffffff"/>"##,
        WIDTH, HEIGHT
    ));
    svg.push('\n');

    svg.push_str(&format!(
        r#"  <text x="{}" y="30" text-anchor="middle" font-size="20" font-weight="bold">{}</text>"#,
        WIDTH / 2,
        escape_xml(&data.title)
    ));
    svg.push('\n');

    let chart_width = WIDTH - 2 * MARGIN - 100; // Extra space for labels
    let chart_height = HEIGHT - 2 * MARGIN;
    let num_bars = data.points.len();
    let bar_height = (chart_height / num_bars).saturating_sub(BAR_PADDING);
    
    let max_value = data.max_value();
    let scale = if max_value > 0.0 {
        chart_width as f64 / max_value
    } else {
        1.0
    };

    // Draw bars horizontally
    for (i, point) in data.points.iter().enumerate() {
        let y = MARGIN + i * (bar_height + BAR_PADDING);
        let bar_width = (point.value * scale) as usize;
        let x = MARGIN + 100;

        // Bar
        svg.push_str(&format!(
            r##"  <rect x="{}" y="{}" width="{}" height="{}" fill="#4a90e2" stroke="#2c5aa0" stroke-width="1"/>"##,
            x, y, bar_width, bar_height
        ));
        svg.push('\n');

        // Label on left
        let label_with_glyph = if let Some(glyph) = point.glyph {
            format!("{} {}", glyph, point.label)
        } else {
            point.label.clone()
        };

        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" text-anchor="end" font-size="12">{}</text>"#,
            MARGIN + 95,
            y + bar_height / 2 + 5,
            escape_xml(&label_with_glyph)
        ));
        svg.push('\n');

        // Value at end of bar
        let value_label = if data.is_relative {
            format!("{:.1}%", point.value)
        } else if point.value >= 1_000_000.0 {
            format!("{:.1}M", point.value / 1_000_000.0)
        } else if point.value >= 1_000.0 {
            format!("{:.1}K", point.value / 1_000.0)
        } else {
            format!("{:.0}", point.value)
        };

        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" font-size="12">{}</text>"#,
            x + bar_width + 5,
            y + bar_height / 2 + 5,
            value_label
        ));
        svg.push('\n');
    }

    svg.push_str("</svg>\n");
    Ok(svg)
}

/// Render grouped bar chart (simplified version)
fn render_grouped(data: &CategoricalData) -> FlameResult<String> {
    // For now, render as vertical bars
    render_vertical(data)
}

/// Render stacked bar chart (simplified version)
fn render_stacked(data: &CategoricalData) -> FlameResult<String> {
    // For now, render as vertical bars
    render_vertical(data)
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::viz::types::DataPoint;

    #[test]
    fn test_render_vertical() {
        let points = vec![
            DataPoint::new("A".to_string(), 100.0),
            DataPoint::new("B".to_string(), 200.0),
        ];
        let data = CategoricalData::new("Test Chart".to_string(), points);
        let svg = render_svg(&data, VisualizationMode::Vertical).unwrap();
        
        assert!(svg.contains("<svg"));
        assert!(svg.contains("Test Chart"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("A&B"), "A&amp;B");
        assert_eq!(escape_xml("<tag>"), "&lt;tag&gt;");
    }
}
