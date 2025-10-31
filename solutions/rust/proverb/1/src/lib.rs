pub fn build_proverb(list: &[&str]) -> String {
    let mut output = String::new();
    if list.len() == 0 {return output;}
    if list.len() > 1 {
        for i in 0..list.len()-1 {
            let s = format!("For want of a {} the {} was lost.\n",list[i], list[i+1]);
            output.push_str(&s);
        }
    }
    output.push_str(&format!("And all for the want of a {}.", list[0]));
    output
}
