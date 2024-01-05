mod graph;

use crate::graph::{Graph, Node};


fn main() {
    println!("Hello, world!");
    
    let filepath = "src/graphs/graph1.graph";
    
    if let Err(err) = Graph::new_from_file(filepath) {
        println!("Error: {}", err);
    } else { 


    }


    


}
