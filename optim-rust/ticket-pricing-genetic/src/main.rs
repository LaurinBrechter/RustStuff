use rand::prelude::*;

struct TicketProblem {
    n_periods: u32,
    wtp: Vec<f32>,
    wtps: Vec<f32>,
    occurrence_prob: Vec<Vec<f32>>,
    capacity: u32,
    group_sizes: Vec<u32>,
}

struct Individual {
    price: Vec<f32>,
    n_offered: Vec<i32>,
    val: f32,
}

fn random_list(m: i32, n: i32, min_val: i32) {
    let mut rng = rand::thread_rng();

    let mut arr = vec![min_val, m];
    let left = n - 1 - m * min_val;

    while left >= 0 {
        if rng.gen::<f32>() < 0.3 {
            arr[rng.gen_range(0..n) % m] += 1
        }
    }
}

fn main() {
    const G: i32 = 6;
    const T: u32 = 60;

    println!("Hello, world!");

    let problem = TicketProblem {
        group_sizes: vec![1, 2, 3],
        capacity: 100,
        n_periods: T,
        wtp: vec![0.2],
        wtps: vec![0.4],
        occurrence_prob: vec![vec![0.2, 0.4]],
    };
}
