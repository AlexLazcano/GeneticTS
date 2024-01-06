mod genetic;
mod graph;

use crate::genetic::GeneticAlgorithm;
use crate::graph::Graph;

fn run_genetic(graph: &Graph) {
    let ga = GeneticAlgorithm::new(graph);
    let fitnesses = ga.calculate_fitnesses();
    println!("fitness: {:?}", fitnesses)

}

fn main() {
    println!("Hello, world!");

    let filepath = "src/graphs/graph1.graph";

    match Graph::new_from_file(filepath) {
        Ok(g) => {
            print!("{}", g);
            run_genetic(&g);
        }

        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
