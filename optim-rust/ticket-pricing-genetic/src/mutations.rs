use rand::Rng;
use rand_distr::Normal;
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

pub fn mutate_n_tickets_offered(n_offered: &mut Vec<i32>, max_n_offered:i32) {
    let t = n_offered.len();
    let mut rng = rand::thread_rng();
    let n_offered_idx = rng.gen_range(0..t);

    n_offered[n_offered_idx] = rng.gen_range(0..max_n_offered);

}