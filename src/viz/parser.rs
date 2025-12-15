//! Text Parser Module
//!
//! Extracts categorical data from text descriptions

use super::{CategoricalData, DataEntry};

pub struct TextParser {
    // Parser configuration
}

impl TextParser {
    pub fn new() -> Self {
        Self {}
    }

    /// Parse categorical data from text
    /// 
    /// Supports formats:
    /// - "Label1: value1, Label2: value2, ..."
    /// - "Label1 value1, Label2 value2, ..."
    /// - Tabular text with headers
    pub fn parse_categorical(&self, text: &str) -> Result<CategoricalData, String> {
        if text.trim().is_empty() {
            return Err("Empty input text".to_string());
        }

        // Try different parsing strategies
        if let Ok(data) = self.parse_colon_format(text) {
            return Ok(data);
        }

        if let Ok(data) = self.parse_table_format(text) {
            return Ok(data);
        }

        // Fallback: try to extract any numbers with labels
        self.parse_flexible_format(text)
    }

    /// Parse "Label: value, Label: value" format
    fn parse_colon_format(&self, text: &str) -> Result<CategoricalData, String> {
        let mut entries = Vec::new();
        
        // Strategy: Find all colons, then work backwards and forwards to extract label and value
        // This properly handles numbers with commas
        let mut pos = 0;
        while let Some(colon_idx) = text[pos..].find(':') {
            let absolute_colon = pos + colon_idx;
            
            // Extract label: work backwards to find the start (after previous comma or start of string)
            let label_start = text[..absolute_colon]
                .rfind(',')
                .map(|i| i + 1)
                .unwrap_or(0);
            let label = text[label_start..absolute_colon].trim();
            
            // Extract value: work forwards to find the end
            // Look for ", [Letter]+ :" pattern (next entry) or end of string
            let value_start = absolute_colon + 1;
            let mut value_end = text.len();
            
            // Search for next entry pattern (", word:")
            if let Some(rest_text) = text.get(value_start..) {
                // Look for pattern: comma, then at least one letter, then colon
                let mut in_potential_label = false;
                let mut potential_label_start = 0;
                
                for (i, ch) in rest_text.char_indices() {
                    if ch == ',' && !in_potential_label {
                        in_potential_label = true;
                        potential_label_start = i;
                    } else if in_potential_label && ch == ':' {
                        // Check if there's text between comma and colon with letters
                        let between = &rest_text[potential_label_start + 1..i].trim();
                        if !between.is_empty() && between.chars().any(|c| c.is_alphabetic()) {
                            // This looks like a new entry
                            value_end = value_start + potential_label_start;
                            break;
                        }
                        in_potential_label = false;
                    } else if in_potential_label && !ch.is_whitespace() && !ch.is_alphabetic() && ch != ':' {
                        // Reset if we see non-label characters
                        in_potential_label = false;
                    }
                }
            }
            
            let value_str = text[value_start..value_end].trim();
            
            // Extract numeric value (remove commas, units, etc.)
            if let Some(value) = self.extract_number(value_str) {
                if !label.is_empty() {
                    entries.push(DataEntry {
                        label: label.to_string(),
                        value,
                    });
                }
            }
            
            pos = absolute_colon + 1;
        }

        if entries.is_empty() {
            return Err("No valid entries found in colon format".to_string());
        }

        Ok(CategoricalData {
            title: "Categorical Data".to_string(),
            entries,
            unit: None,
        })
    }

    /// Parse table format (e.g., from Zybooks)
    fn parse_table_format(&self, text: &str) -> Result<CategoricalData, String> {
        let lines: Vec<&str> = text.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        if lines.len() < 2 {
            return Err("Not enough lines for table format".to_string());
        }

        let mut entries = Vec::new();

        // Skip header line and parse data rows
        for line in lines.iter().skip(1) {
            // Split by whitespace or tabs
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            if parts.len() >= 2 {
                // Last part is typically the value
                if let Some(value) = self.extract_number(parts[parts.len() - 1]) {
                    // Everything before is the label
                    let label = parts[..parts.len() - 1].join(" ");
                    entries.push(DataEntry {
                        label,
                        value,
                    });
                }
            }
        }

        if entries.is_empty() {
            return Err("No valid entries found in table format".to_string());
        }

        Ok(CategoricalData {
            title: lines[0].to_string(),
            entries,
            unit: None,
        })
    }

    /// Flexible parser that tries to find patterns
    fn parse_flexible_format(&self, text: &str) -> Result<CategoricalData, String> {
        let mut entries = Vec::new();
        let mut current_label: Option<String> = None;

        // Tokenize by whitespace
        let tokens: Vec<&str> = text.split_whitespace().collect();

        for i in 0..tokens.len() {
            if let Some(value) = self.extract_number(tokens[i]) {
                // Found a number, use previous tokens as label
                if i > 0 {
                    let label = if let Some(ref label) = current_label {
                        label.clone()
                    } else {
                        tokens[i.saturating_sub(1)].to_string()
                    };
                    
                    entries.push(DataEntry {
                        label: self.clean_label(&label),
                        value,
                    });
                    current_label = None;
                }
            } else {
                // Accumulate label tokens
                let token = tokens[i].trim_matches(&[',', ':', ';', '.'][..]);
                current_label = Some(match current_label {
                    Some(ref label) => format!("{} {}", label, token),
                    None => token.to_string(),
                });
            }
        }

        if entries.is_empty() {
            return Err("Could not parse any valid data entries".to_string());
        }

        Ok(CategoricalData {
            title: "Parsed Data".to_string(),
            entries,
            unit: None,
        })
    }

    /// Extract numeric value from string, removing commas and units
    fn extract_number(&self, text: &str) -> Option<f64> {
        // Remove commas and common units
        let cleaned = text
            .replace(',', "")
            .replace('$', "")
            .replace('%', "")
            .trim()
            .to_string();

        // Try to parse as float
        cleaned.parse::<f64>().ok()
    }

    /// Clean label by removing unwanted characters
    fn clean_label(&self, label: &str) -> String {
        label
            .trim_matches(&[',', ':', ';', '.', '!', '?'][..])
            .trim()
            .to_string()
    }
}

impl Default for TextParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colon_format() {
        let parser = TextParser::new();
        let text = "Walmart: 2300000, Amazon: 566000, Kroger: 435000";
        let result = parser.parse_categorical(text);
        
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.entries.len(), 3);
        assert_eq!(data.entries[0].label, "Walmart");
        assert_eq!(data.entries[0].value, 2300000.0);
    }

    #[test]
    fn test_comma_separated_numbers() {
        let parser = TextParser::new();
        let text = "Company A: 1,250,000, Company B: 500,000";
        let result = parser.parse_categorical(text);
        
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.entries[0].value, 1250000.0);
    }

    #[test]
    fn test_empty_text() {
        let parser = TextParser::new();
        let result = parser.parse_categorical("");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_number() {
        let parser = TextParser::new();
        assert_eq!(parser.extract_number("1,234,567"), Some(1234567.0));
        assert_eq!(parser.extract_number("$100.50"), Some(100.5));
        assert_eq!(parser.extract_number("75%"), Some(75.0));
    }
}
