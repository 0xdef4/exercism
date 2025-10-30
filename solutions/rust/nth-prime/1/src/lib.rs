pub fn nth(n: u32) -> u32 {
    (1..)
        .filter(|n| is_prime(*n as u64))
        .nth(n as usize)
        .expect("there must be a prime number")
}

fn is_prime(n: u64) -> bool {
    n > 1 && (2..=n.isqrt()).all(|k| n % k != 0)
}
