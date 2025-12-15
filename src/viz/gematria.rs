//! Layer 2: Gematria-based ranking
//! 
//! Apply Hebrew gematria weighting for sacred ordering of data

use super::types::CategoricalData;

/// Hebrew gematria values for letters
fn gematria_value(c: char) -> u32 {
    match c.to_ascii_lowercase() {
        'a' => 1, 'b' => 2, 'c' => 3, 'd' => 4, 'e' => 5,
        'f' => 6, 'g' => 7, 'h' => 8, 'i' => 9, 'j' => 10,
        'k' => 20, 'l' => 30, 'm' => 40, 'n' => 50, 'o' => 60,
        'p' => 70, 'q' => 80, 'r' => 90, 's' => 100, 't' => 200,
        'u' => 300, 'v' => 400, 'w' => 500, 'x' => 600, 'y' => 700, 'z' => 800,
        _ => 0,
    }
}

/// Calculate gematria sum for a string
fn calculate_gematria(s: &str) -> u32 {
    s.chars().map(gematria_value).sum()
}

/// Rank categorical data by gematria values (optional sacred ordering)
/// 
/// This reorders data points based on their gematria weight,
/// providing a mystical/sacred ordering dimension beyond simple alphabetical or numerical sort
pub fn rank_data(mut data: CategoricalData) -> CategoricalData {
    // Calculate gematria for each label
    let mut indexed: Vec<(usize, u32)> = data.points
        .iter()
        .enumerate()
        .map(|(i, p)| (i, calculate_gematria(&p.label)))
        .collect();
    
    // Sort by gematria value (descending - highest sacred value first)
    indexed.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Reorder points based on gematria ranking
    let reordered = indexed
        .into_iter()
        .map(|(i, _)| data.points[i].clone())
        .collect();
    
    data.points = reordered;
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::viz::types::DataPoint;

    #[test]
    fn test_gematria_values() {
        assert_eq!(gematria_value('a'), 1);
        assert_eq!(gematria_value('z'), 800);
        assert_eq!(gematria_value('m'), 40);
    }

    #[test]
    fn test_calculate_gematria() {
        // "abc" = 1 + 2 + 3 = 6
        assert_eq!(calculate_gematria("abc"), 6);
        // "Walmart" = w(500) + a(1) + l(30) + m(40) + a(1) + r(90) + t(200) = 862
        assert_eq!(calculate_gematria("Walmart"), 862);
    }

    #[test]
    fn test_rank_data() {
        let points = vec![
            DataPoint::new("Apple".to_string(), 100.0),
            DataPoint::new("Zebra".to_string(), 200.0),
            DataPoint::new("Banana".to_string(), 150.0),
        ];
        let data = CategoricalData::new("Test".to_string(), points);
        let ranked = rank_data(data);
        
        // Zebra has highest gematria, should be first
        assert_eq!(ranked.points[0].label, "Zebra");
    }
}
