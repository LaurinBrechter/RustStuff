use rand::Rng;
use rand_distr::Normal;
use rand::prelude::*;

pub fn mutate_price(price: &Vec<f64>) -> Vec<f64> {
    let mut price_new = price.clone();
    let t = price.len();
    let mut rng = rand::thread_rng();
    let price_idx = rng.gen_range(0..t);

    let mean: f64 = price.iter().sum::<f64>() / t as f64;

    let normal = Normal::new(0.0, mean as f64).unwrap();
    let random_value = normal.sample(&mut rng);

    price_new[price_idx] = (price[price_idx] + random_value).max(0.0);

    return price_new

}

pub fn mutate_n_tickets_offered(n_offered: &Vec<i32>, max_n_offered:i32) -> Vec<i32> {
    let mut n_offered_new = n_offered.clone();
    let t = n_offered.len();
    let mut rng = rand::thread_rng();
    let n_offered_idx = rng.gen_range(0..t);

    n_offered_new[n_offered_idx] = rng.gen_range(0..max_n_offered);

    return n_offered_new

}