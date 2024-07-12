use rand::Rng;

pub fn recombine_price(price1: &Vec<f32>, price2: &Vec<f32>) -> Vec<f32> {
    let mut price_new = price1.clone();
    let t = price1.len();
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..t);

    for i in idx..t {
        price_new[i] = price2[i];
    }

    price_new
}

pub fn recombine_n_offered(n_offered1: &Vec<i32>, n_offered2: &Vec<i32>) -> Vec<i32> {
    let mut n_offered_new = n_offered1.clone();
    let t = n_offered1.len();
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..t);

    for i in idx..t {
        n_offered_new[i] = n_offered2[i];
    }

    n_offered_new
}