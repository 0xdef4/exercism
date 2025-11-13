use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut output = BTreeMap::new();
    for (number, letters) in h {
        for letter in letters {
            output.entry((*letter).to_lowercase().next().unwrap()).or_insert(*number);
        }
    }
    output
}
