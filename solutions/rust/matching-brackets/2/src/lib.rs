pub fn brackets_are_balanced(filtered_string: &str) -> bool {
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
