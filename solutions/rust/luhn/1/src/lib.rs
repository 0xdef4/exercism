/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.len() <= 1 {
        return false
    }
    let replaced_str = code.replace(" ", "");
    if replaced_str.len() <=1 {
        return false
    }
    for char in replaced_str.chars() {
        if !char.is_numeric() {
            return false
        }
    }
    let sum: usize = replaced_str.chars().rev().enumerate().map(|(i,c)| {
        if i % 2 == 1 {
            if (c.to_digit(10).unwrap() as usize * 2) > 9 {
                c.to_digit(10).unwrap() as usize * 2 - 9
            } else {
                c.to_digit(10).unwrap() as usize * 2
            }
        } else {
            c.to_digit(10).unwrap() as usize
        }
    }).sum();

    if sum % 10 == 0 {
        return true
    } else {
        return false
    }
}
