use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min >= max {
        return None;
    }
    let mut set = HashSet::new();
    let factors_iter = (min..=max).flat_map(|n1| (min..=max).map(move |n2| (n1, n2)));

    for (a, b) in factors_iter {
        let (x, y) = if a <= b { (a, b) } else { (b, a) };
        set.insert((x, y));
    }
    let palindrome_factors = set
        .into_iter()
        .filter(|(n1, n2)| is_palindrome(n1 * n2))
        .collect::<Vec<(u64, u64)>>();
    let mut palindromes = palindrome_factors
        .iter()
        .map(|(n1, n2)| n1 * n2)
        .collect::<Vec<_>>();
    palindromes.sort();

    if palindromes.is_empty() {
        return None;
    }

    Some((
        Palindrome {
            value: *palindromes.first().unwrap(),
            factors: HashSet::from_iter(
                palindrome_factors
                    .iter()
                    .filter(|(e1, e2)| e1 * e2 == *palindromes.first().unwrap())
                    .cloned(),
            ),
        },
        Palindrome {
            value: *palindromes.last().unwrap(),
            factors: HashSet::from_iter(
                palindrome_factors
                    .iter()
                    .filter(|(e1, e2)| e1 * e2 == *palindromes.last().unwrap())
                    .cloned(),
            ),
        },
    ))
}

pub fn is_palindrome(number: u64) -> bool {
    let number_string = number.to_string();

    number_string
        .char_indices()
        .all(|(i, c)| c == number_string.chars().collect::<Vec<_>>()[number_string.len() - 1 - i])
}
