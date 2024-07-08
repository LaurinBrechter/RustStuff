struct TicketProblem {
    n_periods: u32,
    wtp: Vec<f32>,
    wtps: Vec<f32>,
    occurrence_prob: Vec<Vec<f32>>,
    capacity: u32,
    group_sizes: Vec<u32>,
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
