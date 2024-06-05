use rand::prelude::*;
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
            for element in row.iter() {
                flattened.push(element);
            }
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
}

#[derive(Clone)]
struct Individual {
    solution: Vec<Vec<bool>>,
    score: f32,
    shape: (usize, usize),
}

impl Individual {
    fn init<T>(&mut self, problem: &Matrix<T>) {
        self.solution = vec![];
        let mut idx = 0;
        for row in problem.data.iter() {
            self.solution.insert(idx, vec![]);

            let mut row_idx = 0;
            for _ in row.iter() {
                self.solution[idx].insert(row_idx, rand::random());
                row_idx += 1;
            }
            row_idx = 0;
            idx += 1;
        }
    }

    fn mutate(&mut self) {}

    fn crossover(&mut self, other: &Individual) -> (Individual, Individual) {
        let row = rand::random::<usize>() % self.solution.len();

        let mut child = self.clone();
        let mut child_other = other.clone();

        let interm = child.solution[row].clone();

        child.solution[row] = child_other.solution[row].clone();
        child_other.solution[row] = interm;

        return (child, child_other);
    }

    fn is_valid(&self) -> bool {
        // row sums equal to one
        let row_sums = vec![];
        let col_sums = vec![];
    }
}

fn main() {
    let problem = Matrix {
        data: vec![
            vec![4, 2, 6, 5],
            vec![7, 6, 5, 10],
            vec![4, 8, 8, 10],
            vec![2, 5, 9, 9],
        ],
        flattened: None,
    };

    let mut a = Individual {
        solution: vec![vec![]],
        score: 0.0,
        shape: (4, 4),
    };
    a.init(&problem);

    let mut b = Individual {
        solution: vec![vec![]],
        score: 0.0,
        shape: (4, 4),
    };
    b.init(&problem);

    // i.init(&problem);

    println!("{:?}", a.crossover(&b).0.solution)
}
