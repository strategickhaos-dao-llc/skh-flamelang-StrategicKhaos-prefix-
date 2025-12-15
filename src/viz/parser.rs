//! Layer 1: Categorical Data Parser
//! 
//! Parse categorical data from English text (Zybooks format)

use crate::{FlameError, FlameResult};
use super::types::{CategoricalData, DataPoint};

/// Parse categorical data from text
/// 
/// Example input: "4 largest private employers in U.S. (2017): Walmart 2.3M, Amazon 566k, Yum! 450k, Kroger 449k"
pub fn parse_categorical(text: &str) -> FlameResult<CategoricalData> {
    // Split by colon to separate title from data
    let parts: Vec<&str> = text.splitn(2, ':').collect();
    
    if parts.len() < 2 {
        return Err(FlameError::Parser(
            "Invalid format: expected 'title: data1, data2, ...'".to_string()
        ));
    }

    let title = parts[0].trim().to_string();
    let data_str = parts[1].trim();

    // Check if this is relative frequency data
    let is_relative = text.to_lowercase().contains("percent")
        || text.to_lowercase().contains("frequency")
        || data_str.contains('%');

    // Parse data points separated by commas
    let mut points = Vec::new();
    
    for item in data_str.split(',') {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }

        // Parse "Label Value" format (e.g., "Walmart 2.3M")
        if let Some(point) = parse_data_point(item) {
            points.push(point);
        }
    }

    if points.is_empty() {
        return Err(FlameError::Parser(
            "No data points found".to_string()
        ));
    }

    Ok(CategoricalData::new(title, points).with_relative(is_relative))
}

/// Parse a single data point from text
fn parse_data_point(text: &str) -> Option<DataPoint> {
    // Find the last space to split label and value
    let parts: Vec<&str> = text.rsplitn(2, ' ').collect();
    
    if parts.len() < 2 {
        return None;
    }

    let value_str = parts[0].trim();
    let label = parts[1].trim().to_string();

    // Parse value with suffix support (K, M, B for thousands, millions, billions)
    if let Some(value) = parse_numeric_value(value_str) {
        Some(DataPoint::new(label, value))
    } else {
        None
    }
}

/// Parse numeric value with K/M/B suffixes
fn parse_numeric_value(s: &str) -> Option<f64> {
    let s = s.trim();
    
    // Remove percentage sign if present
    let (s, multiplier) = if s.ends_with('%') {
        (&s[..s.len()-1], 1.0)
    } else {
        (s, 1.0)
    };

    // Check for suffix
    let (num_str, suffix_multiplier) = if let Some(last_char) = s.chars().last() {
        match last_char.to_ascii_uppercase() {
            'K' => (&s[..s.len()-1], 1_000.0),
            'M' => (&s[..s.len()-1], 1_000_000.0),
            'B' => (&s[..s.len()-1], 1_000_000_000.0),
            _ => (s, 1.0),
        }
    } else {
        (s, 1.0)
    };

    // Parse the base number and check for overflow/infinity
    num_str.parse::<f64>().ok().and_then(|n| {
        let result = n * suffix_multiplier * multiplier;
        // Check if result is finite (not infinity or NaN)
        if result.is_finite() {
            Some(result)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_employer_data() {
        let input = "4 largest private employers in U.S. (2017): Walmart 2.3M, Amazon 566k, Yum! 450k, Kroger 449k";
        let result = parse_categorical(input).unwrap();
        
        assert_eq!(result.title, "4 largest private employers in U.S. (2017)");
        assert_eq!(result.points.len(), 4);
        assert_eq!(result.points[0].label, "Walmart");
        assert_eq!(result.points[0].value, 2_300_000.0);
    }

    #[test]
    fn test_parse_numeric_values() {
        assert_eq!(parse_numeric_value("100"), Some(100.0));
        assert_eq!(parse_numeric_value("2.3M"), Some(2_300_000.0));
        assert_eq!(parse_numeric_value("566k"), Some(566_000.0));
        assert_eq!(parse_numeric_value("1.5B"), Some(1_500_000_000.0));
        assert_eq!(parse_numeric_value("25%"), Some(25.0));
    }

    #[test]
    fn test_detect_relative() {
        let input = "Relative frequency: Category A 25%, Category B 75%";
        let result = parse_categorical(input).unwrap();
        assert!(result.is_relative);
    }
}
