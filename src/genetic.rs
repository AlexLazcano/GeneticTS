use crate::graph::Graph;
use rand::Rng;

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
pub struct Individual {
    gene: Vec<usize>,
}

impl Individual { 
    pub fn order_x_over(p1: &Individual, p2: &Individual) -> Individual  {
        // let mut rng = rand::thread_rng();
        // let gene_length = p1.gene.len();
        // let first = rng.gen_range(0..gene_length);
        // let second = rng.gen_range(first..gene_length);
        // FIX: Need to make it a permutation !!!
        let first: usize = 2;
        let second:usize  = 6;

        let mut p1_gene = p1.gene.clone();
        
        for i in first..second { 

            p1_gene[i] = p2.gene[i];
        }

        Individual { 
            gene: p1_gene
        }
    }

    pub fn new(gene: Vec<usize>) -> Self { 
        Individual { 
            gene
        }
    }
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
        Individual::order_x_over(self, other)
    }

    fn mutate(&self) {
        todo!()
    }
}
