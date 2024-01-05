use core::fmt;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Node {
    data: usize,
    out_going_edges: Vec<usize>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.data, self.out_going_edges)
    }
}

impl Node {
    pub fn new(data: usize) -> Self {
        Node {
            data,
            out_going_edges: Vec::new(),
        }
    }

    pub fn add_to_edge_list(&mut self, neigh: usize) {
        self.out_going_edges.push(neigh)
    }
}

#[derive(Debug)]
pub struct Graph {
    nodes: HashMap<usize, Node>,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_keys: Vec<_> = self.nodes.keys().copied().collect();
        sorted_keys.sort();

        write!(f, "Graph:\n")?;
        for key in sorted_keys {
            if let Some(node) = self.nodes.get(&key) {
                write!(f, "{}\n", node)?;
            };
        }
        Ok(())
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn new_from_file(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);
        let mut graph = Graph::new();
        for line in reader.lines() {
            let line = line?;

            let mut parts = line.split_whitespace();
            if let (Some(source_str), Some(dest_str)) = (parts.next(), parts.next()) {
                if let (Ok(source), Ok(dest)) = (source_str.parse(), dest_str.parse()) {
                    graph.add_node(source);
                    graph.add_node(dest);
                    graph.add_edge(source, dest)
                }
            }
        }
        print!("{}", graph);

        Ok(())
    }

    pub fn add_node(&mut self, id: usize) {
        if !self.nodes.contains_key(&id) {
            self.nodes.insert(id, Node::new(id));
        }
    }

    pub fn add_edge(&mut self, src: usize, dest: usize) {
        if let Some(node) = self.nodes.get_mut(&src) {
            node.add_to_edge_list(dest)
        } else {
            println!("Source {} not fount", src)
        }
    }
}
