use std::collections::HashMap;

use graph_items::edge::Edge;
use graph_items::node::Node;

pub mod graph_items;
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        for node in nodes {
            self.nodes.push(node.clone());
        }
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        for edge in edges {
            self.edges.push(edge.clone());
        }
        self
    }

    pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
        for attr in attrs {
            self.attrs.insert(attr.0.to_string(), attr.1.to_string());
        }
        self
    }

    pub fn node(&self, node_name: &'static str) -> Option<Node> {
        self.nodes.iter().find(|&x| x.name == node_name).cloned()
    }
}
