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
    if min > max {
        return None;
    }

    let mut min_pal: Option<(u64, HashSet<(u64, u64)>)> = None;
    let mut max_pal: Option<(u64, HashSet<(u64, u64)>)> = None;

    for i in min..=max {
        for j in i..=max {
            let prod = i * j;
            if is_palindrome(prod) {
                // 최소 palindrome 갱신
                if min_pal.as_ref().map_or(true, |(val, _)| prod < *val) {
                    min_pal = Some((prod, HashSet::from([(i, j)])));
                } else if min_pal.as_ref().map_or(false, |(val, _)| prod == *val) {
                    min_pal.as_mut().unwrap().1.insert((i, j));
                }

                // 최대 palindrome 갱신
                if max_pal.as_ref().map_or(true, |(val, _)| prod > *val) {
                    max_pal = Some((prod, HashSet::from([(i, j)])));
                } else if max_pal.as_ref().map_or(false, |(val, _)| prod == *val) {
                    max_pal.as_mut().unwrap().1.insert((i, j));
                }
            }
        }
    }

    match (min_pal, max_pal) {
        (Some((min_val, min_factors)), Some((max_val, max_factors))) => Some((
            Palindrome {
                value: min_val,
                factors: min_factors,
            },
            Palindrome {
                value: max_val,
                factors: max_factors,
            },
        )),
        _ => None,
    }
}

pub fn is_palindrome(number: u64) -> bool {
    let number_string = number.to_string();

    number_string
        .char_indices()
        .all(|(i, c)| c == number_string.chars().collect::<Vec<_>>()[number_string.len() - 1 - i])
}
