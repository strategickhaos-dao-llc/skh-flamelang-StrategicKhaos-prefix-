//! Layer 2: Numeric Transformation  
//! Hebrew roots -> Gematria values

pub fn transform(roots: &[[char; 3]]) -> Vec<u64> {
    roots.iter().map(|r| gematria(r)).collect()
}

fn gematria(root: &[char; 3]) -> u64 {
    root.iter().map(|c| char_value(*c)).sum()
}

fn char_value(c: char) -> u64 {
    match c {
        '\u{05D0}' => 1,   // Aleph
        '\u{05D1}' => 2,   // Bet
        '\u{05D2}' => 3,   // Gimel
        // ... more mappings
        _ => 0,
    }
}
