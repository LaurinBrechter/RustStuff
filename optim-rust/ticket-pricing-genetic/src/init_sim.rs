use ndarray_rand::RandomExt;
use rand_distr::{Beta, Distribution, Normal, Uniform};
use rand::prelude::*;
use ndarray::Array1;



pub fn sample_group_sizes(n_groups: u32, n_individuals:u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut group_sizes = Vec::new();
    let mut left = n_individuals;

    for g in 0..n_groups {
        if g == n_groups - 1 {
            group_sizes.push(left);
        } else  {
            let size = rng.gen_range(1..left + g - n_groups);
            group_sizes.push(size);
            left -= size;
        } 
    }
    return group_sizes;
}


pub fn init_occurence_probs(n_periods: usize, group_sizes: Vec<usize>) -> Vec<Vec<f32>> {
    let mut rng = rand::thread_rng();
    let uniform_dist = Uniform::from(0..(n_periods-1));
    
    let mut occurrences: Vec<Vec<f32>> = Vec::new();

    for _g in group_sizes {
        let start = uniform_dist.sample(&mut rng);
        let end = rng.gen_range((start + 1)..n_periods);
        
        let beta_dist = Beta::new(1.0, 1.0).unwrap();
        let mut probs: Array1<f32> = Array1::random(end - start, beta_dist);
        let probs_sum: f32 = probs.sum();
        
        for val in probs.iter_mut() {
            *val = f32::min(1.0, 2.0 * (*val) / probs_sum);
        }

        let mut occurence = vec![0.0; start];
        occurence.extend(probs.to_vec());
        occurence.extend(vec![0.0; n_periods - end]);
        
        occurrences.push(occurence);
    }

    occurrences
}


pub fn sample_halfnormal(rng: &mut ThreadRng, loc: f32, scale: f32) -> f32 {
    let normal_dist = Normal::new(0.0, 1.0).unwrap();
    let mut val = normal_dist.sample(rng);
    while val < 0.0 {
        val = normal_dist.sample(rng);
    }
    return val * scale + loc;
}