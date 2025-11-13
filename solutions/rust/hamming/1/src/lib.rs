/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    Some(s1.char_indices().filter(|(i,c)| s2.chars().nth(*i).unwrap() != *c).count())
}
