use crate::{Individual, Customer, TicketProblem, objective_fn};
use rand_distr::{Distribution, Normal, Uniform};
use rand::prelude::*;

pub fn init_individual(rng: &mut ThreadRng, problem: &TicketProblem, customers: &Vec<Customer>, n_resample:u32, tau:f32) -> Individual {
    
    let max_wtp = customers.iter().map(|c| c.wtp).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    let price_dist = Uniform::new(0.0, max_wtp*tau);
    let price = (0..problem.n_periods).map(|_| price_dist.sample(rng)).collect::<Vec<f32>>();
    
    let n_offered_dist = Uniform::new(0, 100);
    let n_offered = (0..problem.n_periods).map(|_| n_offered_dist.sample(rng)).collect::<Vec<i32>>();
    
    let mut new_ind =  Individual {
        price: price,
        n_offered: n_offered,
        val: 0.0
    };

    new_ind.val = objective_fn(rng, problem, customers, &new_ind, n_resample);
    new_ind
}

pub fn init_individual_discrete(rng: &mut ThreadRng, problem: &TicketProblem, customers: &Vec<Customer>, n_resample:u32, tau:f32) -> Individual {
    
    let max_wtp = customers.iter().map(|c| c.wtp).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() as i32;

    let price_dist = Uniform::new(0, max_wtp*tau as i32);
    let price = (0..problem.n_periods).map(|_| price_dist.sample(rng) as f32).collect::<Vec<f32>>();
    
    let n_offered_dist = Uniform::new(0, 100);
    let n_offered = (0..problem.n_periods).map(|_| n_offered_dist.sample(rng)).collect::<Vec<i32>>();
    
    let mut new_ind =  Individual {
        price: price,
        n_offered: n_offered,
        val: 0.0
    };

    new_ind.val = objective_fn(rng, problem, customers, &new_ind, n_resample);
    new_ind
}