pub mod graph {
    use std::collections::HashMap;

    use graph_items::edge::Edge;
    use graph_items::node::Node;

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

    pub mod graph_items {
        pub mod edge {
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
        }

        pub mod node {
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
        }
    }
}
