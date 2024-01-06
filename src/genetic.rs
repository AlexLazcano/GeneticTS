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

        print!("{:?}", pop);
        return pop;
    }
}

#[derive(Debug)]
struct Individual {
    gene: Vec<usize>,
}
trait IndividualFunctions {
    fn fitness(&self, graph: Graph) -> usize;
    fn cross_over(&self, other: &Self) -> Self;
    fn mutate(&self);
}

impl IndividualFunctions for Individual {
    fn fitness(&self, graph: Graph) -> usize {
        let mut current: usize = 0;
        let nodes = graph.get_nodes();
        let mut fitness = 0;
        for i in self.gene.iter() {
            println!("{}", i)
        }

        fitness
    }

    fn cross_over(&self, other: &Self) -> Self {
        todo!()
    }

    fn mutate(&self) {
        todo!()
    }
}
