use std::collections::HashMap;

pub fn factors(n: u64) -> Vec<u64> {
    let mut map = HashMap::new();
    let mut num = n;
    for prime in (2..).filter(|a| is_prime(*a)) {
        while num % prime == 0 {
            num /= prime;
            update_map(&mut map, &prime);
        }
        if num == 1 {
            break;
        }
    }
    let mut output: Vec<u64> = map
        .iter()
        .flat_map(|(&k, &v)| std::iter::repeat(k).take(v as usize))
        .collect();
    output.sort();
    output
}

fn is_prime(n: u64) -> bool {
    n > 1 && (2..=n.isqrt()).all(|k| n % k != 0)
}

fn update_map(m: &mut HashMap<u64, i32>, p: &u64) {
    m.entry(*p).and_modify(|x| *x += 1).or_insert(1);
}
