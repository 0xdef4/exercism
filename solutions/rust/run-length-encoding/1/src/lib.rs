pub fn encode(source: &str) -> String {
    let mut target = source;
    let mut pointer = 0;
    let mut output = String::new();

    while pointer != source.len() {
        if let Some(found) = target.find(|c| c != source.chars().nth(pointer).unwrap()) {
            if found != 1 {
                output.push_str((found).to_string().as_str());
            }
            output.push(source.chars().nth(pointer).unwrap());
            pointer += found;
            target = &source[pointer..];
        } else {
            if source.len() - pointer != 1 {
                output.push_str((source.len() - pointer).to_string().as_str());
            }
            output.push(source.chars().nth(pointer).unwrap());
            pointer = source.len();
        }
    }
    output
}

pub fn decode(source: &str) -> String {
    let mut target = source;
    let mut pointer = 0;
    let mut output = String::new();

    while pointer < source.len() {
        if let Some(found) =
            target.find(|c: char| c.is_ascii_alphabetic() || c.is_ascii_whitespace())
        {
            let times;
            if found == 0 {
                times = 1;
            } else {
                times = target[0..found].parse::<usize>().unwrap();
            }
            output.push_str(&target.chars().nth(found).unwrap().to_string().repeat(times));
            pointer += found + 1;
            target = &source[pointer..];
        }
    }
    output
}
