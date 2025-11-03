pub fn collatz(n: u64) -> Option<u64> {
    let mut steps = Vec::new();
    let mut num = n;
    if num == 0 { return None;}

    while num > 1 {
        match num % 2 {
            0 => {
                num = num / 2;
                steps.push(num)
            }            
            _ => {
                num = 3 * num + 1;
                steps.push(num)
            }
        }
    }
    match steps.len() {
        length => Some(length as u64),
        0 => None
    }
}
