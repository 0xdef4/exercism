use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|e| e.is_ascii_alphabetic())
        .all(|e| set.insert(e))
}