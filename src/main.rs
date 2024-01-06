mod genetic;
mod graph;

use crate::genetic::GeneticAlgorithm;
use crate::graph::Graph;

fn runGenetic(graph: &Graph) {
    let g = GeneticAlgorithm::new(graph);
}

fn main() {
    println!("Hello, world!");

    let filepath = "src/graphs/graph1.graph";

    match Graph::new_from_file(filepath) {
        Ok(g) => {
            print!("{}", g);
            runGenetic(&g);
        }

        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
