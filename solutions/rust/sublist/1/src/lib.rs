#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if check_equality(first_list, second_list) == Comparison::Equal {
        return Comparison::Equal
    } else {
        if first_list.len() < second_list.len() {
            for i in 0..=second_list.len().saturating_sub(first_list.len()) {
                if check_equality(first_list, &second_list[i..(i + first_list.len())]) == Comparison::Equal
                {
                    return Comparison::Sublist;
                }
            }
            return Comparison::Unequal;
        } else {
            for i in 0..=first_list.len().saturating_sub(second_list.len()) {
                if check_equality(second_list, &first_list[i..(i + second_list.len())]) == Comparison::Equal
                {
                    return Comparison::Superlist;
                }
            }
            return Comparison::Unequal;
        }
    }
}

fn check_equality(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.len() == second_list.len() {
        for (i, el) in first_list.iter().enumerate() {
            if el == &second_list[i] {
                continue;
            } else {
                return Comparison::Unequal;
            }
        }
        return Comparison::Equal;
    }
    return Comparison::Unequal;
}
