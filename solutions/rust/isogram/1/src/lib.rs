use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut map = HashMap::new();
    for char in candidate
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|e| e.to_ascii_lowercase())
    {
        map.entry(char).and_modify(|e| *e += 1).or_insert(1);
    }
    !map.values().any(|e| *e != 1)
}
