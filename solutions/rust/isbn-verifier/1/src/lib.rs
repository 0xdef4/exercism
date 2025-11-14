/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.chars().any(|e| e.is_ascii_alphabetic() && e != 'X') || isbn.is_empty() {
        return false;
    }

    if isbn.chars().filter(|c| c.is_digit(10) || *c == 'X').count() != 10 {
        return false;
    }

    if isbn
        .chars()
        .filter(|c| c.is_digit(10) || *c == 'X')
        .any(|c| c == 'X')
        && isbn
            .chars()
            .filter(|c| c.is_digit(10) || *c == 'X')
            .last()
            .unwrap()
            != 'X'
    {
        return false;
    }

    (isbn
        .chars()
        .filter(|c| c.is_digit(10) || *c == 'X')
        .zip((1..11).rev())
        .map(|(e, n)| {
            if e.is_digit(10) {
                e.to_digit(10).unwrap() * n
            } else if e == 'X' {
                10 * n
            } else {
                todo!()
            }
        })
        .sum::<u32>())
        % 11
        == 0
}
