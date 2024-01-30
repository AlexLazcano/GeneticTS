use crate::graph::Graph;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Individual {
    pub gene: Vec<usize>,
    pub fitness: isize,
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
