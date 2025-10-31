use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .flat_map(|factor| (1..limit+1).map(|e| *factor * e).filter(|e| *e < limit))
        .collect::<HashSet<_>>()
        .iter()
        .sum()
}
