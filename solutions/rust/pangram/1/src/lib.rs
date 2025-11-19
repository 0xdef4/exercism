use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut set = HashSet::new();
    for e in sentence.chars().filter(|c| c.is_ascii_alphabetic()) {
        set.insert(e.to_ascii_lowercase());
    }
    set.len() == 26
}
