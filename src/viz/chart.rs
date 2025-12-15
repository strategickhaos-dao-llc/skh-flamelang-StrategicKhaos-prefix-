//! SVG Chart Generation Module
//!
//! Generates SVG bar charts with relative frequency visualization

use super::CategoricalData;

pub struct ChartGenerator {
    width: u32,
    height: u32,
    margin: u32,
}

impl ChartGenerator {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
            margin: 60,
        }
    }

    /// Generate a relative frequency bar chart in SVG format
    pub fn generate_bar_chart(&self, data: &CategoricalData) -> Result<String, String> {
        if data.entries.is_empty() {
            return Err("Cannot generate chart from empty dataset".to_string());
        }

        let total: f64 = data.entries.iter().map(|e| e.value).sum();
        if total <= 0.0 {
            return Err("Total value must be positive".to_string());
        }

        let chart_width = self.width - (2 * self.margin);
        let chart_height = self.height - (2 * self.margin);
        
        let bar_spacing = 20;
        let num_bars = data.entries.len();
        let spacing_total = if num_bars > 1 {
            bar_spacing * (num_bars as u32 - 1)
        } else {
            0
        };
        let bar_width = (chart_width - spacing_total) / num_bars as u32;

        let mut svg = String::new();
        
        // SVG header
        svg.push_str(&format!(
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",
            self.width, self.height
        ));
        svg.push('\n');

        // Background
        svg.push_str(&format!(
            "  <rect width=\"{}\" height=\"{}\" fill=\"#1a1a1a\"/>",
            self.width, self.height
        ));
        svg.push('\n');

        // Title
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"30\" fill=\"#ffffff\" font-size=\"20\" font-weight=\"bold\" text-anchor=\"middle\">{}</text>",
            self.width / 2, data.title
        ));
        svg.push('\n');

        // Y-axis label
        svg.push_str("  <text x=\"20\" y=\"300\" fill=\"#888888\" font-size=\"14\" writing-mode=\"tb\">Relative Frequency (%)</text>");
        svg.push('\n');

        // Generate bars
        for (i, entry) in data.entries.iter().enumerate() {
            let relative_freq = (entry.value / total) * 100.0;
            let bar_height = (relative_freq / 100.0) * chart_height as f64;
            
            let x = self.margin + (i as u32 * (bar_width + bar_spacing));
            let y = self.height - self.margin - bar_height as u32;

            // Bar with gradient
            let color = self.get_bar_color(i);
            svg.push_str(&format!(
                "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" opacity=\"0.9\" rx=\"4\"/>",
                x, y, bar_width, bar_height as u32, color
            ));
            svg.push('\n');

            // Value label on top of bar
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" fill=\"#ffffff\" font-size=\"14\" font-weight=\"bold\" text-anchor=\"middle\">{:.1}%</text>",
                x + bar_width / 2, y - 5, relative_freq
            ));
            svg.push('\n');

            // X-axis label (rotated for long labels)
            let label_x = x + bar_width / 2;
            let label_y = self.height - self.margin + 20;
            
            if entry.label.len() > 10 {
                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"{}\" fill=\"#cccccc\" font-size=\"12\" text-anchor=\"end\" transform=\"rotate(-45 {} {})\">{}</text>",
                    label_x, label_y, label_x, label_y, entry.label
                ));
            } else {
                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"{}\" fill=\"#cccccc\" font-size=\"12\" text-anchor=\"middle\">{}</text>",
                    label_x, label_y, entry.label
                ));
            }
            svg.push('\n');
        }

        // X-axis line
        svg.push_str(&format!(
            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#666666\" stroke-width=\"2\"/>",
            self.margin, self.height - self.margin,
            self.width - self.margin, self.height - self.margin
        ));
        svg.push('\n');

        // Y-axis line
        svg.push_str(&format!(
            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#666666\" stroke-width=\"2\"/>",
            self.margin, self.margin,
            self.margin, self.height - self.margin
        ));
        svg.push('\n');

        // Y-axis ticks and labels
        for i in 0..=5 {
            let percent = i * 20;
            let y_pos = self.height - self.margin - (chart_height * i / 5);
            
            svg.push_str(&format!(
                "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#666666\" stroke-width=\"1\"/>",
                self.margin - 5, y_pos, self.margin, y_pos
            ));
            svg.push('\n');
            
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" fill=\"#888888\" font-size=\"12\" text-anchor=\"end\">{}%</text>",
                self.margin - 10, y_pos + 4, percent
            ));
            svg.push('\n');
        }

        svg.push_str("</svg>");

        Ok(svg)
    }

    /// Get color for bar based on index
    fn get_bar_color(&self, index: usize) -> &str {
        let colors = [
            "#ff6b6b", // Red
            "#4ecdc4", // Cyan
            "#45b7d1", // Blue
            "#f7b731", // Yellow
            "#5f27cd", // Purple
            "#00d2d3", // Teal
            "#fd79a8", // Pink
            "#fdcb6e", // Orange
        ];
        colors[index % colors.len()]
    }
}

impl Default for ChartGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::viz::DataEntry;

    #[test]
    fn test_chart_generation() {
        let gen = ChartGenerator::new();
        let data = CategoricalData {
            title: "Test Chart".to_string(),
            entries: vec![
                DataEntry { label: "A".to_string(), value: 100.0 },
                DataEntry { label: "B".to_string(), value: 50.0 },
            ],
            unit: None,
        };

        let result = gen.generate_bar_chart(&data);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("Test Chart"));
    }

    #[test]
    fn test_empty_data_error() {
        let gen = ChartGenerator::new();
        let data = CategoricalData {
            title: "Empty".to_string(),
            entries: vec![],
            unit: None,
        };

        let result = gen.generate_bar_chart(&data);
        assert!(result.is_err());
    }
}
