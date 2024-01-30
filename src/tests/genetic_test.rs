#[cfg(test)]

use crate::individual::Individual;
use crate::{graph::{Graph, self}, individual::IndividualFunctions};


#[test]
pub fn sample_test() { 
    println!("Hello");
    assert_eq!(1, 1)
}

#[test]
pub fn cross_over_test() {

    let gene1 = vec![1,2,3,4,5];
    let gene2 = vec![4,3,2,1,5];
    let mut p1 = Individual::new(gene1);
    let mut p2 = Individual::new(gene2);

    let mut offspring1 = Individual::order_x_over(&p1, &p2);

    let mut offspring2 = Individual::order_x_over(&p2, &p1);
    
   

    let filepath = "src/graphs/graph1.graph";

    
    match Graph::new_from_file(filepath) {
        Ok(g) => {
            print!("{}", g); 
            p1.calculate_fitness(&g);
            p2.calculate_fitness(&g);
            offspring1.calculate_fitness(&g);
            offspring2.calculate_fitness(&g);

            println!("{} {:?} {}", "p1", p1, p1.get_fitness());
            println!("{} {:?} {}", "p2", p2, p2.get_fitness());
            println!("{} {:?} {}", "o1", offspring1, offspring1.get_fitness());
            println!("{} {:?} {}", "o2", offspring2, offspring2.get_fitness());
        }

        Err(err) => {
            println!("Error: {}", err);
        }
    }

}