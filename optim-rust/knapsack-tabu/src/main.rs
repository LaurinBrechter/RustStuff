use std::collections::{HashMap, HashSet};

struct Item {
    value: i32,
    weight: i32,
}

struct Backpack {
    items: Vec<Item>,
    capacity: i32,
}

impl Backpack {
    fn total_weight(&self) -> i32 {
        let mut total = 0;
        for item in self.items.iter() {
            total += item.weight;
        }

        return total;
    }
    fn is_valid(&self) -> bool {
        return self.total_weight() <= self.capacity;
    }

    fn total_value(&self) -> i32 {
        let mut total = 0;
        for item in self.items.iter() {
            total += item.value;
        }

        return total;
    }

    fn new(values: Vec<i32>, weights: Vec<i32>, capacity: i32) -> Self {
        let mut items = Vec::new();
        for i in 0..values.len() {
            items.push(Item {
                value: values[i],
                weight: weights[i],
            });
        }

        Self { items, capacity }
    }
}


fn generate_candidates(n_items:i32, solution: &HashSet<i32>, taboo_remove: &HashSet<i32>, taboo_add: &HashSet<i32>) -> Vec<HashSet<i32>> { // 

    let mut candidates: Vec<HashSet<i32>> = [].to_vec();

    let items = (0..n_items).collect::<Vec<i32>>();

    let can_remove = solution.difference(taboo_remove).collect::<HashSet<&i32>>();
    
    let can_add = HashSet::from_iter(items.iter().cloned())
        .difference(&solution).cloned().collect::<HashSet<i32>>()
        .difference(&taboo_add).cloned().collect::<HashSet<i32>>();

    for el in can_add.iter() {
        let mut candidate = solution.clone();
        candidate.insert(*el);
        candidates.push(candidate);
    }

    for el in can_remove.iter() {
        let mut candidate = solution.clone();
        candidate.remove(*el);
        candidates.push(candidate);
    }

    return candidates

} 


fn is_valid(encoding: &HashSet<i32>, backpack: &Backpack) -> (bool, i32, i32) {
    let mut total_weight = 0;
    let mut total_value = 0;
    for i in encoding.iter() {
        let idx = *i as usize;

        total_weight += backpack.items[idx].weight;
        total_value += backpack.items[idx].value;
    }

    return (total_weight <= backpack.capacity, total_value, total_weight);
}

fn main() {
    println!("Hello, world!");

    let b = Backpack {
        items: vec![
            Item {
                value: 6,
                weight: 9,
            },
            Item {
                value: 7,
                weight: 10,
            },
            Item {
                value: 5,
                weight: 14,
            },
            Item {
                value: 8,
                weight: 11,
            },
            Item {
                value: 5,
                weight: 8,
            },
            Item {
                value: 3,
                weight: 8,
            },
        ],
        capacity: 50,
    };

    // number of iterations an item is kept in the taboo list
    const TABOO_EXPIRY: u32 = 4;
    const MAX_IT: u32 = 5;

    // the algorithm will terminate if there has been no improvement in this number of iterations
    const MAX_IT_NO_IMPROV: u32 = 4;

    // println!("Candidate 0 Is valid: {}", is_valid);
    let mut solution = HashSet::from([0,1,2]);
    let mut taboo_remove: HashSet<i32> = HashSet::new();
    let mut taboo_add: HashSet<i32> = HashSet::new();

    let (_, mut best_value, _) = is_valid(&solution, &b);
    let mut best_solution = solution.clone();


    let n_items = b.items.len();

    let mut tr: HashMap<i32, i32> = HashMap::new();
    for i in 0..n_items {
        tr.insert(i.try_into().unwrap(), 0);
    }

    let x = tr.iter().filter(|&(_, v)| *v == 0);


    let mut ta: HashMap<i32, i32> = HashMap::new();
    for i in 0..n_items {
        ta.insert(i.try_into().unwrap(), 0);
    }

    let mut it_no_improv = 0;

    for it in 0..MAX_IT {
        let candidates = generate_candidates(6, &solution, &taboo_remove, &taboo_add);

        // print!("Candidates: {:?}", candidates);

        let mut best_value_it = 0;
        let mut best_candidate_it: HashSet<i32> = HashSet::new();
        

        for candidate in candidates.iter() {
            let (valid, tv, tw) = is_valid(&candidate, &b);
            if valid {
                if tv > best_value_it {
                    best_value_it = tv;
                    best_candidate_it = candidate.clone();
                    if tv > best_value {
                        it_no_improv = 0;
                        best_value = tv;
                        best_solution = candidate.clone();
                        println!("Best solution: {:?}", best_solution);
                        println!("Best value: {:?}", best_value);

                    }
                }
            }
        }



        // solution = best_candidate_it;

        let added = solution.difference(&best_candidate_it).cloned().collect::<HashSet<i32>>();
        let removed = best_candidate_it.difference(&solution).cloned().collect::<HashSet<i32>>();


        if it_no_improv == MAX_IT_NO_IMPROV {
            break
        }
    }
}
