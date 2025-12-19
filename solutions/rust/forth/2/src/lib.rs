use std::collections::HashMap;
use std::rc::Rc;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Rc<Vec<Op>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Clone)]
enum Op {
    Number(Value),
    Plus,
    Minus,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    Sequence(Rc<Vec<Op>>),
}

impl Forth {
    pub fn new() -> Self {
        let mut words = HashMap::new();

        words.insert("+".into(), Rc::new(vec![Op::Plus]));
        words.insert("-".into(), Rc::new(vec![Op::Minus]));
        words.insert("*".into(), Rc::new(vec![Op::Mul]));
        words.insert("/".into(), Rc::new(vec![Op::Div]));
        words.insert("dup".into(), Rc::new(vec![Op::Dup]));
        words.insert("drop".into(), Rc::new(vec![Op::Drop]));
        words.insert("swap".into(), Rc::new(vec![Op::Swap]));
        words.insert("over".into(), Rc::new(vec![Op::Over]));

        Self {
            stack: Vec::new(),
            words,
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let tokens = input
            .split_ascii_whitespace()
            .map(|s| s.to_ascii_lowercase())
            .collect::<Vec<_>>();

        let ops = self.compile(tokens)?;
        for op in ops {
            self.apply(&op)?;
        }
        Ok(())
    }

    fn compile(&mut self, tokens: Vec<String>) -> std::result::Result<Vec<Op>, Error> {
        let mut res = Vec::new();
        let mut it = tokens.into_iter();

        while let Some(tok) = it.next() {
            match tok.as_str() {
                ":" => {
                    let name = it.next().ok_or(Error::InvalidWord)?;
                    if name.parse::<i32>().is_ok() {
                        return Err(Error::InvalidWord);
                    }

                    let mut def = Vec::new();
                    loop {
                        let t = it.next().ok_or(Error::InvalidWord)?;
                        if t == ";" {
                            break;
                        }
                        def.push(self.compile_token(&t)?);
                    }

                    self.words.insert(name, Rc::new(def));
                }
                ";" => return Err(Error::InvalidWord),
                _ => res.push(self.compile_token(&tok)?),
            }
        }

        Ok(res)
    }

    fn compile_token(&self, tok: &str) -> std::result::Result<Op, Error> {
        if let Ok(n) = tok.parse::<Value>() {
            return Ok(Op::Number(n));
        }

        if let Some(seq) = self.words.get(tok) {
            return Ok(Op::Sequence(seq.clone()));
        }

        Err(Error::UnknownWord)
    }

    fn apply(&mut self, op: &Op) -> Result {
        match op {
            Op::Number(n) => {
                self.stack.push(*n);
                Ok(())
            }
            Op::Plus => self.binary_op(|a, b| a.checked_add(b)),
            Op::Minus => self.binary_op(|a, b| a.checked_sub(b)),
            Op::Mul => self.binary_op(|a, b| a.checked_mul(b)),
            Op::Div => {
                if self.stack.last() == Some(&0) {
                    return Err(Error::DivisionByZero);
                }
                self.binary_op(|a, b| a.checked_div(b))
            }
            Op::Dup => {
                let v = *self.stack.last().ok_or(Error::StackUnderflow)?;
                self.stack.push(v);
                Ok(())
            }
            Op::Drop => {
                self.stack.pop().ok_or(Error::StackUnderflow)?;
                Ok(())
            }
            Op::Swap => {
                let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(b);
                self.stack.push(a);
                Ok(())
            }
            Op::Over => {
                let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(a);
                self.stack.push(b);
                self.stack.push(a);
                Ok(())
            }
            Op::Sequence(seq) => {
                for op in seq.iter() {
                    self.apply(op)?;
                }
                Ok(())
            }
        }
    }

    fn binary_op<F>(&mut self, f: F) -> Result
    where
        F: Fn(Value, Value) -> Option<Value>,
    {
        let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let r = f(a, b).ok_or(Error::DivisionByZero)?;
        self.stack.push(r);
        Ok(())
    }
}
