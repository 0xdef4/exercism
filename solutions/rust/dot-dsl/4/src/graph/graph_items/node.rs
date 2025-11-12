use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    pub name: &'static str,
    pub attrs: HashMap<&'static str, &'static str>,
}

impl Node {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
        for attr in attrs {
            self.attrs.insert(attr.0, attr.1);
        }
        self
    }

    pub fn attr(&self, key: &'static str) -> Option<&'static str> {
        self.attrs.get(key).map(|v| *v)
    }
}
