pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string().len();
    // if digits < 2 {
    //     return true;
    // }

    num
        == num
            .to_string()
            .chars()
            .map(|c| (c.to_digit(10).unwrap()).pow(digits as u32))
            .sum()
}
