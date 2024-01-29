use crate::graph::Graph;
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
        for ind in &self.population {
            println!("{:?} = fit {}", ind.gene, ind.fitness)
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
    pub fn selection(&mut self) {
        let winners = self.tournament_selection();
    }
    pub fn cross_over() {
        todo!()
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
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Individual {
    gene: Vec<usize>,
    fitness: isize,
}

impl Individual {
    pub fn get_fitness(&self) -> isize {
        self.fitness
    }
    pub fn calculate_fitness(&mut self, graph: &Graph) {
        let nodes = graph.get_nodes();
        let mut fitness = 0;

        // println!("{} f: {}", 0, fitness);
        let mut prev = nodes.get(&0).unwrap();

        for i in self.gene.iter() {
            fitness += prev.get_weight_to(i);
            // println!("{} f: {}", i, fitness);
            prev = nodes.get(i).unwrap();
        }

        fitness += prev.get_weight_to(&0);
        // println!("{} f: {}", 0, fitness);

        self.fitness = fitness;
    }

    pub fn order_x_over(p1: &Individual, p2: &Individual) -> Individual {
        // let mut rng = rand::thread_rng();
        // let gene_length = p1.gene.len();
        // let first = rng.gen_range(0..gene_length);
        // let second = rng.gen_range(first..gene_length);
        let length = p1.gene.len();
        let first: usize = 1;
        let last: usize = length;

        let p1_slice = &p1.gene[first..last];
        let mut p1_gene = p1.gene.clone();
        // println!("{:?}", p1_slice);
        let mut j = 0;
        for i in 0..length {
            if i >= first && i < last {
                // copy elements from 1
                // p1_gene[i] = 1;
            } else {
                // get elements that are not in slice from 2
                while p1_slice.contains(&p2.gene[j]) {
                    j += 1
                }

                p1_gene[i] = p2.gene[j];
                j += 1;
            }
        }

        Individual {
            gene: p1_gene,
            fitness: 0,
        }
    }

    pub fn new(gene: Vec<usize>) -> Self {
        Individual { gene, fitness: 0 }
    }
}

impl PartialOrd for Individual {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

impl Ord for Individual {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fitness.cmp(&other.fitness)
    }
}
pub trait IndividualFunctions {
    fn cross_over(&self, other: &Self) -> Self;
    fn mutate(&self);
}

impl IndividualFunctions for Individual {
    fn cross_over(&self, other: &Self) -> Self {
        Individual::order_x_over(self, other)
    }

    fn mutate(&self) {
        todo!()
    }
}
