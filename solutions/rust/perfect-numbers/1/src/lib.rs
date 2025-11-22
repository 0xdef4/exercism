use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {return None;}
    let sum = (1..=num)
        .filter(|e| num % e == 0)
        .take((1..=num).filter(|e| num % e == 0).count() - 1)
        .sum::<u64>();

    match num.cmp(&sum) {
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Less => Some(Classification::Abundant),
        Ordering::Greater => Some(Classification::Deficient),
    }
}
