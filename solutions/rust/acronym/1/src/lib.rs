pub fn abbreviate(phrase: &str) -> String {
    phrase
        .replace("-", " ")
        .chars()
        .filter(|char| *char == ' ' || (*char).is_ascii_alphabetic())
        .collect::<String>()
        .split_whitespace()
        .flat_map(|word| {
            if word.chars().all(|c| c.is_uppercase()) {
                Box::new(word.chars().next().unwrap().to_uppercase())
                    as Box<dyn Iterator<Item = char>>
            } else {
                Box::new(
                    word.chars()
                        .next()
                        .unwrap()
                        .to_uppercase()
                        .chain(word[1..].chars().filter(|c| c.is_uppercase())),
                ) as Box<dyn Iterator<Item = char>>
            }
        })
        .collect::<String>()
}
