use std::collections::HashMap;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut map = HashMap::new();
    for char in ['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'] {
        map.insert(char, 1);
    }
    for char in ['D', 'G'] {
        map.insert(char, 2);
    }
    for char in ['B', 'C', 'M', 'P'] {
        map.insert(char, 3);
    }
    for char in ['F', 'H', 'V', 'W', 'Y'] {
        map.insert(char, 4);
    }
    for char in ['K'] {
        map.insert(char, 5);
    }
    for char in ['J', 'X'] {
        map.insert(char, 8);
    }
    for char in ['Q', 'Z'] {
        map.insert(char, 10);
    }
    let mut sum = 0;
    for c in word.chars().filter(|e| e.is_ascii_alphabetic()) {
        sum += map.get(&c.to_uppercase().next().unwrap()).unwrap();
    }
    sum
}
