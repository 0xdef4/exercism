use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct Edge {
    pub node_1: &'static str,
    pub node_2: &'static str,
    pub attrs: HashMap<&'static str, &'static str>,
}

impl Edge {
    pub fn new(node_1: &'static str, node_2: &'static str) -> Self {
        Self {
            node_1,
            node_2,
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
