use rand::Rng;
use rand_distr::{Normal, Uniform};
use rand::prelude::*;

pub fn mutate_price(price: &mut Vec<f32>) {
    // let mut price_new = price.clone();
    let t = price.len();
    let mut rng = rand::thread_rng();
    let price_idx = rng.gen_range(0..t);

    let mean: f32 = price.iter().sum::<f32>() / t as f32;

    let normal = Normal::new(0.0, mean as f32).unwrap();
    let random_value = normal.sample(&mut rng);

    price[price_idx] = (price[price_idx] + random_value).max(0.0);
}

pub fn mutate_price_discrete(price: &mut Vec<f32>, max_price:f32, step_size:i32) {
    // let mut price_new = price.clone();
    let t = price.len();
    let mut rng = rand::thread_rng();
    let price_idx = rng.gen_range(0..t);

    let n_steps = (max_price) as i32 / step_size;

    let price_dist = Uniform::new(0, n_steps);
    // let price = (0..problem.n_periods).map(|_| (price_dist.sample(rng) * step_size) as f32).collect::<Vec<f32>>();

    // let price_dist = Uniform::new(0.0, max_price);
    // let price = (0..problem.n_periods).map(|_| price_dist.sample(rng)).collect::<Vec<f32>>();

    price[price_idx] = (price_dist.sample(&mut rng)*step_size) as f32;
}



pub fn mutate_n_tickets_offered(n_offered: &mut Vec<i32>, max_n_offered:i32) {
    let t = n_offered.len();
    let mut rng = rand::thread_rng();
    let n_offered_idx = rng.gen_range(0..t);

    n_offered[n_offered_idx] = rng.gen_range(0..max_n_offered);

}