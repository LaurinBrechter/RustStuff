use rand::prelude::*;
use std::iter::zip;
use std::{clone, vec};

fn mutate(a: &Vec<i32>, b: &Vec<i32>) {}

#[derive(Debug)]
struct Matrix<'a, T> {
    data: Vec<Vec<T>>,
    flattened: Option<Vec<&'a T>>,
}

impl<'a, T> Matrix<'a, T> {
    fn flatten(&mut self) {
        let mut flattened: Vec<&'a T> = vec![];
        for row in self.data.iter() {
            for element in row.iter() {}
        }

        self.flattened = Some(flattened);
    }
}

struct Item {
    value: i32,
    weight: i32,
}

struct Backpack {
    items: Vec<Item>,
    capacity: i32,
}

#[derive(Clone)]
struct Individual {
    solution: Vec<bool>,
    score: f32,
}

impl Backpack {
    fn is_valid(&self, ind: &Individual) -> bool {
        let mut weight = 0;

        for item in zip(&self.items, &ind.solution) {
            if *item.1 {
                weight += item.0.weight;
            }
        }
        if weight >= self.capacity {
            return true;
        } else {
            return false;
        }
    }

    fn total_value(&self) -> i32 {
        let mut total = 0;
        for item in self.items.iter() {
            total += item.value;
        }

        return total;
    }
}

impl Individual {
    fn init(backpack: &Backpack, rng: &mut ThreadRng) -> Self {
        let mut solution = vec![];

        let mut weight = 0;
        for item in backpack.items.iter() {
            if rand::random() {
                if weight <= backpack.capacity {
                    solution.push(true)
                } else {
                    solution.push(false)
                }
            } else {
                solution.push(false)
            }
        }
        let initial_score = score_individual(&solution, &backpack);

        Individual {
            solution,
            score: initial_score.0,
        }
    }

    fn mutate(&mut self, rng: &mut ThreadRng) {
        let swap_index = rng.gen_range(0..self.solution.len());

        if let Some(element) = self.solution.get_mut(swap_index) {
            if *element {
                *element = false
            } else {
                *element = true
            }
        }
    }

    fn crossover(&mut self, other: &Individual) -> (Individual, Individual) {
        let row = rand::random::<usize>() % self.solution.len();

        let mut child = self.clone();
        let mut child_other = other.clone();

        let interm = child.solution[row].clone();

        child.solution[row] = child_other.solution[row].clone();
        child_other.solution[row] = interm;

        return (child, child_other);
    }
}

fn score_individual(encoding: &Vec<bool>, b: &Backpack) -> (f32, f32) {
    let mut score = 0;
    let mut weight = 0;
    for item in zip(encoding, b.items.iter()) {
        if *item.0 == true {
            score += item.1.value;
            weight += item.1.weight;
        }
    }

    return (score as f32, weight as f32);
}

fn main() {
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
    // let mut c = Individual::init {&b};
    // i.init(&problem)
    //
    //
    // ;
    let mut rng = thread_rng();
    let mut a = Individual::init(&b, &mut rng);
    let mut c = Individual::init(&b, &mut rng);

    println!("{:?}", a.crossover(&c).0.solution);
    println!("{:?}", a.solution);
    a.mutate(&mut rng);
    println!("{:?}", a.solution);
}
