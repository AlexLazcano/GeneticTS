mod genetic;
mod graph;
mod tests;

use crate::genetic::GeneticAlgorithm;
use crate::graph::Graph;

fn run_genetic(graph: &Graph) {
    let mut ga = GeneticAlgorithm::new(graph, 20);
    ga.evaluation();
    ga.print_population();
    ga.selection();

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
