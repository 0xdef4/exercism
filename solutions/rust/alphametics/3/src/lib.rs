use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // find number of letters in the string
    let mut set = HashSet::new();
    for char in input.replace(" + ", "").replace(" == ", "").chars() {
        set.insert(char);
    }
    let number_of_letters = set.len();

    // find permutations
    let perm = (0..=9).permutations(number_of_letters);

    // find the correct permutation
    let mut correct_map = HashMap::new();
    'outer: for vec in perm {
        let mut map = HashMap::new();
        for (i, key) in set.iter().enumerate() {
            map.insert(*key, vec[i].to_string());
        }

        // map filled, so change input's letters to numbers
        let mut input_replaced = Vec::new();
        for char in input.chars() {
            if map.contains_key(&char) && !char.is_whitespace() {
                input_replaced.push(map.get(&char).unwrap().to_string());
            } else {
                input_replaced.push(char.to_string());
            }
        }
        let new_string = input_replaced.iter().join("");

        let mut new_string_vec = new_string
            .split(" + ")
            .flat_map(|e| e.split(" == "))
            .collect::<Vec<_>>();

        // leading digit in a multi digit should not be zero
        for word in &new_string_vec {
            if (*word).chars().nth(0) == Some('0') {
                continue 'outer;
            }
        }

        let the_sum = new_string_vec.pop().unwrap();

        // check if both sides are equal
        if new_string_vec
            .iter()
            .map(|e| (*e).parse::<i128>().unwrap())
            .sum::<i128>()
            == the_sum.parse::<i128>().unwrap()
        {
            correct_map = map;
            break;
        }
    }

    if correct_map.len() == 0 {
        return None;
    }

    let mut output_map = HashMap::new();
    for (k, v) in correct_map.iter() {
        output_map.insert(*k, v.parse().unwrap());
    }

    Some(output_map)
}
