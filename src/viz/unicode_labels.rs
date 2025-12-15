//! Layer 3: Unicode Glyph Labeling
//! 
//! Add Unicode glyphs/emoji as visual proxies for categorical labels

use super::types::CategoricalData;

/// Add Unicode glyphs to data points based on their labels
pub fn add_glyphs(mut data: CategoricalData) -> CategoricalData {
    for point in &mut data.points {
        point.glyph = match_glyph(&point.label);
    }
    data
}

/// Match a label to an appropriate Unicode glyph/emoji
fn match_glyph(label: &str) -> Option<char> {
    let label_lower = label.to_lowercase();
    
    // Company/brand glyphs
    if label_lower.contains("walmart") || label_lower.contains("retail") {
        Some('🏪') // Convenience store
    } else if label_lower.contains("amazon") || label_lower.contains("shop") {
        Some('📦') // Package
    } else if label_lower.contains("food") || label_lower.contains("yum") || label_lower.contains("restaurant") {
        Some('🍔') // Burger
    } else if label_lower.contains("kroger") || label_lower.contains("grocery") {
        Some('🛒') // Shopping cart
    } else if label_lower.contains("tech") || label_lower.contains("software") {
        Some('💻') // Laptop
    } else if label_lower.contains("car") || label_lower.contains("auto") {
        Some('🚗') // Car
    } else if label_lower.contains("health") || label_lower.contains("medical") {
        Some('🏥') // Hospital
    } else if label_lower.contains("bank") || label_lower.contains("finance") {
        Some('🏦') // Bank
    } else if label_lower.contains("education") || label_lower.contains("school") {
        Some('🎓') // Graduation cap
    } else if label_lower.contains("energy") || label_lower.contains("power") {
        Some('⚡') // Lightning
    } else if label_lower.contains("transport") || label_lower.contains("logistics") {
        Some('🚚') // Truck
    } else if label_lower.contains("communication") || label_lower.contains("telecom") {
        Some('📡') // Satellite
    } else {
        // Default glyphs for categories
        Some('●') // Bullet point as default
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::viz::types::DataPoint;

    #[test]
    fn test_match_glyphs() {
        assert_eq!(match_glyph("Walmart"), Some('🏪'));
        assert_eq!(match_glyph("Amazon"), Some('📦'));
        assert_eq!(match_glyph("Yum! Brands"), Some('🍔'));
        assert_eq!(match_glyph("Kroger"), Some('🛒'));
    }

    #[test]
    fn test_add_glyphs() {
        let points = vec![
            DataPoint::new("Walmart".to_string(), 2300000.0),
            DataPoint::new("Amazon".to_string(), 566000.0),
        ];
        let data = CategoricalData::new("Test".to_string(), points);
        let labeled = add_glyphs(data);
        
        assert_eq!(labeled.points[0].glyph, Some('🏪'));
        assert_eq!(labeled.points[1].glyph, Some('📦'));
    }
}
