mod graph;

use crate::graph::{Graph, Node};


fn main() {
    println!("Hello, world!");
    
    let filepath = "src/graphs/graph1.graph";

    
    match Graph::new_from_file(filepath) {

        Ok(g) => { 
            print!("{}", g);
            
        },

        Err(err) => { 
            println!("Error: {}", err);
        }
        
    } 


    


}
