use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();
    let word_map = count_char(word);

    // filter out words that are themselves
    let possible_anagrams_filtered = possible_anagrams
        .iter()
        .filter(|s| (**s).to_lowercase() != word.to_lowercase());

    for string_slice_ref in possible_anagrams_filtered {
        if count_char(*string_slice_ref) == word_map {
            set.insert(*string_slice_ref);
        };
    }
    set
}

fn count_char(string_slice: &str) -> HashMap<char, u8> {
    let mut map = HashMap::new();

    for char in string_slice.to_lowercase().chars() {
        if map.contains_key(&char) {
            // if the character does exist, modify the value by +1
            if let Some(value) = map.get_mut(&char) {
                *value += 1;
            }
        } else {
            // if the character DOES NOT exist in the map, insert it and give it the value 0
            map.insert(char, 0);
        }
    }
    map
}
