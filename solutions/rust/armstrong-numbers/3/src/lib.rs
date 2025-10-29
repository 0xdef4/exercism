pub fn is_armstrong_number(num: u32) -> bool {
    let digits: u32 = num.to_string().len().try_into().unwrap();

    num == num
        .to_string()
        .chars()
        .fold(0, |acc, c| acc + c.to_digit(10).unwrap().pow(digits))
}
