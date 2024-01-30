use crate::graph::Graph;
use crate::individual::Individual;
use crate::individual::IndividualFunctions;
use rand::rngs::mock::StepRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
// use rand::Rng;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;

pub struct GeneticAlgorithm<'a> {
    graph: &'a Graph,
    population: Vec<Individual>,
    // population_size: usize,
}

impl<'a> GeneticAlgorithm<'a> {
    pub fn new(graph: &'a Graph, pop_size: usize) -> Self {
        let pop = GeneticAlgorithm::init_population(graph, pop_size);

        GeneticAlgorithm {
            graph,
            population: pop,
            // population_size: pop_size,
        }
    }

    fn init_population(graph: &'a Graph, population_size: usize) -> Vec<Individual> {
        let nodes = graph.get_nodes();
        let mut sorted_keys: Vec<_> = nodes.keys().copied().collect();
        sorted_keys.sort();
        sorted_keys.remove(0);
        let mut rng = StepRng::new(2, 13);
        let mut irs: Irs<_> = Irs::default();
        let individual = Individual::new(sorted_keys.clone());
        let mut population = Vec::new();
        population.push(individual);

        for _ in 0..population_size - 1 {
            let res = irs.shuffle(&mut sorted_keys, &mut rng);
            match res {
                Ok(_) => {
                    if !population.contains(&Individual::new(sorted_keys.clone())) {
                        population.push(Individual::new(sorted_keys.clone()));
                    }
                }
                Err(error) => {
                    println!("{:?}", error);
                }
            }
        }

        return population;
    }

    pub fn print_population(&self) {
        for (i, ind) in self.population.iter().enumerate() {
            println!("{i}: {:?} = fit {}", ind.gene, ind.fitness)
        }
    }
    pub fn calculate_fitnesses(&mut self) {
        let g = self.graph;
        for i in &mut self.population {
            i.calculate_fitness(g);
        }
    }
    pub fn evaluation(&mut self) {
        self.calculate_fitnesses();
        self.population.sort()
    }
    pub fn selection_reproduction(&mut self) {
        let mut winners = self.tournament_selection();
        self.reproduce(&mut winners);
    }
   

    pub fn replacement() {
        todo!()
    }
    pub fn termination() {
        todo!()
    }

    fn tournament_selection(&self) -> Vec<Individual> {
        let mut rng = thread_rng();
        let mut remaining: Vec<usize> = (0..self.population.len()).collect();
        let mut winners = Vec::new();

        remaining.shuffle(&mut rng);

        while remaining.len() >= 2 {
            let index1 = remaining.pop().unwrap();
            let index2 = remaining.pop().unwrap();

            winners.push(std::cmp::min(self.population[index1].clone(), self.population[index2].clone()));
        }
        winners
    }
    fn reproduce(&mut self, parents: &mut Vec<Individual>){ 
        parents.sort();

        while parents.len() >= 2 { 
            let parent1 = parents.pop().unwrap();
            let parent2 = parents.pop().unwrap();
            let offspring1 = parent1.cross_over(&parent2);
            let offspring2 = parent2.cross_over(&parent1);

            self.population.push(offspring1);
            self.population.push(offspring2);

        }

    }
}
