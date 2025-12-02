use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut set = HashSet::new();    
    for e in 2..=upper_bound {
        for e in (2..=upper_bound / e).map(|n| n*e) {
            set.insert(e);
        }
    }
    (2..=upper_bound).filter(|e| !set.contains(e)).collect::<Vec<u64>>()
}
