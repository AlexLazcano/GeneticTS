#[cfg(test)]

use crate::genetic::Individual;
use crate::{graph::{Graph, self}, genetic::IndividualFunctions};


#[test]
pub fn sample_test() { 
    println!("Hello");
    assert_eq!(1, 1)
}

#[test]
pub fn cross_over_test() {

    let gene1 = vec![1,2,3,4,5];
    let gene2 = vec![4,3,2,1,5];
    let p1 = Individual::new(gene1);
    let p2 = Individual::new(gene2);

    let offspring1 = Individual::order_x_over(&p1, &p2);

    let offspring2 = Individual::order_x_over(&p2, &p1);
    
   

    let filepath = "src/graphs/graph1.graph";

    
    match Graph::new_from_file(filepath) {
        Ok(g) => {
            print!("{}", g);
            println!("{} {:?} {}", "p1", p1, p1.fitness(&g));
            println!("{} {:?} {}", "p2", p2, p2.fitness(&g));
            println!("{} {:?} {}", "o1", offspring1, offspring1.fitness(&g));
            println!("{} {:?} {}", "o2", offspring2, offspring2.fitness(&g));
        }

        Err(err) => {
            println!("Error: {}", err);
        }
    }

}