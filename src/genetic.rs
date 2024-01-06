use crate::graph::Graph;


pub struct GeneticAlgorithm<'a> {
    graph: &'a Graph,
    population: Vec<Individual>,
}

impl<'a> GeneticAlgorithm<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        let pop = GeneticAlgorithm::init_pop(graph);

        GeneticAlgorithm {
            graph,
            population: pop,
        }
    }

    fn init_pop(graph: &'a Graph) -> Vec<Individual> {
        let nodes = graph.get_nodes();
        let mut sorted_keys: Vec<_> = nodes.keys().copied().collect();
        sorted_keys.sort();
        sorted_keys.remove(0);

        let individual = Individual { gene: sorted_keys };

        let mut pop = Vec::new();
        pop.push(individual);

        println!("{:?}", pop);
        return pop;
    }

    pub fn calculate_fitnesses(&self) -> Vec<isize> { 
        let g = self.graph;
        self.population.iter().map(|gene| gene.fitness(g)).collect()
    }

}

#[derive(Debug)]
struct Individual {
    gene: Vec<usize>,
}
trait IndividualFunctions {
    fn fitness(&self, graph: &Graph) -> isize;
    fn cross_over(&self, other: &Self) -> Self;
    fn mutate(&self);
}

impl IndividualFunctions for Individual {
    fn fitness(&self, graph: &Graph) -> isize {
        
        let nodes = graph.get_nodes();
        let mut fitness = 0;

        println!("{} f: {}", 0, fitness);
        let mut prev = nodes.get(&0).unwrap();

        for i in self.gene.iter() {
            fitness += prev.get_weight_to(i);
            println!("{} f: {}", i, fitness);
            prev = nodes.get(i).unwrap();
        }

        fitness += prev.get_weight_to(&0);
        println!("{} f: {}", 0, fitness);

        fitness
    }

    fn cross_over(&self, other: &Self) -> Self {
        todo!()
    }

    fn mutate(&self) {
        todo!()
    }
}
