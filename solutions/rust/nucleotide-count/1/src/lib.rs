use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if dna
        .chars()
        .chain([nucleotide].into_iter())
        .any(|e| e != 'A' && e != 'T' && e != 'G' && e != 'C')
    {
        return Err(dna
            .chars()
            .chain([nucleotide].into_iter())
            .skip_while(|x| *x == 'A' || *x == 'T' || *x == 'G' || *x == 'C')
            .next()
            .unwrap());
    }
    Ok(dna.chars().filter(|c| *c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::new();

    if dna.chars().any(|e| e != 'A' && e != 'T' && e != 'G' && e != 'C') {
        return Err('X');
    }
    for char in dna.chars() {
        map.entry(char).and_modify(|e| *e+=1).or_insert(1);
    }
    for char in ['A', 'C', 'G', 'T'].iter() {
        map.entry(*char).or_insert(0);
    }
    Ok(map)
}
