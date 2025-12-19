use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<i32>,
    words: HashMap<String, Vec<String>>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

const INTEGER_ARITHMETIC: [&str;4] = ["+", "-", "*", "/"];
const STACK_MANIPULATION: [&str;4] = ["dup", "drop", "swap", "over"];

impl Forth {
    pub fn new() -> Forth {
        Self {
            stack: vec![],
            words: HashMap::new()
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut input_iter = input.split(" ").map(|e| e.to_string()).collect::<Vec<String>>();
        input_iter = input_iter.iter().map(|e| e.to_ascii_lowercase()).collect();

        // save user defined words
        if input_iter.first() == Some(&":".to_string()) && input_iter.last() == Some(&";".to_string()) {

            if input_iter[1].parse::<i32>().is_ok() {
                return Err(Error::InvalidWord);
            }
            
            // replace user defined words if there is any
            if input_iter[2..].iter().any(|x| self.words.contains_key(x)) {
                let input_replaced = input_iter[2..input_iter.len() - 1]
                    .iter()
                    .flat_map(|e| {
                        self.words
                            .get(e)
                            .map(|v| Box::new(v.iter()) as Box<dyn Iterator<Item=&String>>)
                            .unwrap_or_else(|| Box::new(std::iter::once(e)))
                    })
                    .cloned()
                    .collect::<Vec<String>>();

                self.words.insert(input_iter[1].clone(), input_replaced);    
            } else {
                self.words.insert(input_iter[1].clone(), input_iter[2..input_iter.len()-1].to_vec());
            }
        } else {
            if input_iter.iter().any(|x| self.words.contains_key(x)) {
                // replace user defined words
                input_iter = input_iter
                    .iter()
                    .map(|e| e.to_ascii_lowercase())
                    .flat_map(|e| {
                        self.words
                            .get(&e)
                            .cloned()
                            .unwrap_or(vec![e])
                    })
                    .collect();
            }

            // if the word doesnt exist error out
            for el in &input_iter {
                if !STACK_MANIPULATION.contains(&el.to_ascii_lowercase().as_str()) && !INTEGER_ARITHMETIC.contains(&el.to_ascii_lowercase().as_str()) && !self.words.contains_key(el) && el.parse::<i32>().is_err() {
                    return Err(Error::UnknownWord);
                }
            }

            // do the operations
            if input_iter.iter().any(|e| {STACK_MANIPULATION.contains(&e.as_str())}) {
                for _ in 0..input_iter.iter().filter(|e| STACK_MANIPULATION.contains(&e.as_str())).count() {
                    let index = input_iter.iter().position(|e| STACK_MANIPULATION.contains(&e.as_str())).unwrap();
                    let _ = match input_iter[index].as_str() {
                        "dup" => {
                            if input_iter[0..index].len() < 1 {
                                return Err(Error::StackUnderflow);
                            }
                            input_iter.splice(index..index+1, [input_iter[index-1].parse::<i32>().unwrap().to_string()]);
                        },
                        "over" => {
                            if input_iter[0..index].len() < 2 {
                                return Err(Error::StackUnderflow);
                            }
                            input_iter.splice(index..index+1, [input_iter[index-2].parse::<i32>().unwrap().to_string()]);
                        },
                        "drop" => {
                            if input_iter[0..index].len() < 1 {
                                return Err(Error::StackUnderflow);
                            }
                            input_iter.drain(index-1..index+1);
                        }
                        "swap" => {
                            if input_iter[0..index].len() < 2 {
                                return Err(Error::StackUnderflow);
                            }
                            input_iter.splice(index-2..index+1, [input_iter[index-1].clone(), input_iter[index-2].clone()]);
                        }
                        _ => {unreachable!()}
                    };
                }
                    self.stack = input_iter.iter().map(|e| e.parse().unwrap()).collect();
            } else if input_iter.iter().any(|e| {INTEGER_ARITHMETIC.contains(&e.as_str())}) {
                for _ in 0..input_iter.iter().filter(|e| INTEGER_ARITHMETIC.contains(&e.as_str())).count() {
                    let index = input_iter.iter().position(|e| INTEGER_ARITHMETIC.contains(&e.as_str())).unwrap();
                    if input_iter[0..index].len() < 2 {
                        return Err(Error::StackUnderflow);
                    }
                    let result_of_arithmetic = match input_iter[index].as_str() {
                        "+" => {
                            input_iter[index-2].parse::<i32>().unwrap() + input_iter[index-1].parse::<i32>().unwrap()
                        },
                        "-" => {
                            input_iter[index-2].parse::<i32>().unwrap() - input_iter[index-1].parse::<i32>().unwrap()
                        },
                        "*" => {
                            input_iter[index-2].parse::<i32>().unwrap() * input_iter[index-1].parse::<i32>().unwrap()
                        },
                        "/" => {
                            if input_iter[index-1].parse::<i32>().unwrap() == 0 {
                                return Err(Error::DivisionByZero);
                            }
                            input_iter[index-2].parse::<i32>().unwrap() / input_iter[index-1].parse::<i32>().unwrap()
                        },
                        _ => {unreachable!()}
                    };
                    input_iter.splice(index-2..index+1, [result_of_arithmetic.to_string()]);
                }
                self.stack = input_iter.iter().map(|e| e.parse().unwrap()).collect();
            } else {
                for el in input_iter {
                    self.stack.push(el.parse().unwrap());
                }
            }
        }
        Ok(())
    }
}
