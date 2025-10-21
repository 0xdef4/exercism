use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");
    let mut output = vec![];

    // let chars: Vec<char> = input.chars().collect();
    let strs: Vec<&str> = input.graphemes(true).collect();

    // println!("{:?}", chars);
    println!("{:?}", strs);

    for i in 0..strs.len() {
        let element = input.graphemes(true).collect::<Vec<&str>>()[strs.len() - 1 - i];
        output.push(element);
    }

    output.join("")
}
