pub mod graph {
    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Graph {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Graph {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Graph {
            self.attrs.extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == name)
        }

        pub fn get_attr(&self, attr: &str) -> Option<&str> {
            self.attrs.get(attr).map(String::as_str)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Eq, PartialEq, Debug)]
            pub struct Edge {
                pub a: String,
                pub b: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(a: &str, b: &str) -> Edge {
                    Edge {
                        a: a.to_string(),
                        b: b.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Edge {
                    self.attrs.extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Eq, PartialEq, Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Node {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Node {
                    self.attrs.extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
                    self
                }

                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(String::as_str)
                }
            }
        }
    }
}
