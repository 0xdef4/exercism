pub fn brackets_are_balanced(string: &str) -> bool {
    let filtered_string = string
        .chars()
        .filter(|char| "{}[]()".contains(*char))
        .collect::<String>();
    let is_balanced = check_balanced(&filtered_string);

    is_balanced
}

fn check_balanced(filtered_string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for char in filtered_string.chars() {
        match char {
            '(' | '{' | '[' => stack.push(char),
            ')' | '}' | ']' => {
                let expected = match char {
                    ')' => '(',
                    '}' => '{',
                    ']' => '[',
                    _ => unreachable!(),
                };
                if stack.pop() != Some(expected) {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}
