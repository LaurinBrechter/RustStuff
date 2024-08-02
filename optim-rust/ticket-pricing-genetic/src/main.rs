use std::fmt;
// use rayon::prelude::*;
mod init;
use std::fs::File;
use std::io::{Write, Read};
mod recombinations;
mod mutations;
mod init_sim;
mod selection;
use recombinations::{recombine_price, recombine_n_offered};
use init::init_individual;
use selection::{roulette_selection, tournament_selection};
use rand_distr::{Binomial, Distribution, Normal, Uniform};
use rand::prelude::*;
use init_sim::{sample_group_sizes, init_occurence_probs, sample_halfnormal};



#[derive(Clone)]
#[derive(Debug)]
struct Individual {
    price: Vec<f32>,
    n_offered: Vec<i32>,
    val: f32,
}


#[derive(Clone)]
struct Customer {
    group: u32,
    // period_occ: Option<u32>,
    wtp: f32,
    bought: bool,
    id: u32
}


fn objective_fn(rng: &mut ThreadRng, problem: &TicketProblem, customers: &Vec<Customer>, ind:&Individual, n_runs:u32) -> f32 {

    let mut revenues = vec![0.0 as f32; n_runs as usize];

    for run in 0..n_runs {

        let mut tickets_left = problem.capacity;
        let mut revenue: f32 = 0.0;
        let mut customers_c = customers.clone();

        for t in 0..problem.n_periods {
            
            let t_idx = t as usize;

            let mut willing_to_pay: Vec<Customer> = Vec::new();
            let mut appearing: Vec<&Customer> = Vec::new();


            for g in 0..problem.n_groups {
                let g_idx = g as usize;

                if problem.occurrence_prob[g_idx][t_idx] == 0.0 {
                    continue;
                }

                let binom_dist = Binomial::new(problem.group_sizes[g_idx] as u64, problem.occurrence_prob[g_idx][t_idx] as f64);

                if binom_dist.is_err() {
                    println!("{}", problem.occurrence_prob[g_idx][t_idx] as f64);
                }

                
                // sample n_occ individuals from group g from customers
                let mut group_customers = customers_c.iter().filter(|&c| c.group == g && c.bought == false).collect::<Vec<&Customer>>();
                let n_occ = std::cmp::min(group_customers.len(), binom_dist.unwrap().sample(rng).try_into().unwrap());
                group_customers.shuffle(rng);

                for i in 0..n_occ {
                    let customer = group_customers[i as usize];
                    appearing.push(customer);

                    if customer.wtp >= ind.price[t_idx] {
                        willing_to_pay.push(customer.clone());
                    }
                }
            }

            let n_tickets_sold = std::cmp::min(
                std::cmp::min(
                    tickets_left,
                    willing_to_pay.len() as u32),
                ind.n_offered[t_idx] as u32
            );
            tickets_left -= n_tickets_sold;
            revenue += n_tickets_sold as f32 * ind.price[t_idx];

            

            // set the bough attribute to true for the customers that bought a ticket
            willing_to_pay.shuffle(rng);


            for i in 0..n_tickets_sold {
                let customer = &willing_to_pay[i as usize];
                customers_c.iter_mut().find(|c| c.id == customer.id && c.group == customer.group).unwrap().bought = true;
            }

            if tickets_left == 0 {
                break;
            }

        }

        revenues[run as usize] = revenue;
        
    }
    return revenues.iter().sum::<f32>() / n_runs as f32;
}



fn recombine(rng: &mut ThreadRng, problem: &TicketProblem, ind1: &Individual, ind2: &Individual, customers: &Vec<Customer>, n_resample:u32) -> Individual {
    let new_price = recombine_price(&ind1.price, &ind2.price);
    let new_n_offered = recombine_n_offered(&ind1.n_offered, &ind2.n_offered);

    let mut new_ind =  Individual {
        price: new_price,
        n_offered: new_n_offered,
        val: 0.0
    };

    new_ind.val = objective_fn(rng, problem, customers, &new_ind, n_resample);

    return new_ind;
}


fn mutate(problem: &TicketProblem, ind: &mut Individual) {
    mutations::mutate_price(&mut ind.price);
    mutations::mutate_n_tickets_offered(&mut ind.n_offered, problem.capacity as i32);
}




fn run(ga_args: &GAAgrs, problem: &TicketProblem, customers: &Vec<Customer>, mut population: Vec<Individual>, rng: &mut ThreadRng) -> Vec<f32> {
    let mut avg_fitness_t: Vec<f32> = Vec::new();
    
    for _ in 0..ga_args.n_iter {
        
        // sum up objective values
        let mut obj_val_sum = 0.0;
        for ind in &population {
            obj_val_sum += ind.val;
        }

        let avg_fitness = obj_val_sum / ga_args.pop_size as f32;

        avg_fitness_t.push(avg_fitness);

        println!("Avg fitness: {}", avg_fitness);

        let mut n_valid_children = 0;
        while n_valid_children < ga_args.n_children {

            let (ind1, ind2): (Individual, Individual);

            match ga_args.selection_strategy {
                SelectionStrategy::Roulette => {
                    (ind1, ind2) = roulette_selection(&population, obj_val_sum, rng);
                },
                SelectionStrategy::Tournament => {
                    (ind1, ind2) = tournament_selection(&population, 5, rng);
                }
            }


            let mut new_ind = recombine(rng, &problem, &ind1.clone(), &ind2.clone(), &customers, ga_args.n_resample);
            
            if rng.gen::<f32>() < ga_args.mutation_rate {
                mutate(&problem, &mut new_ind);
            }

            new_ind.val = objective_fn(rng, &problem, &customers, &new_ind, ga_args.n_resample);

            n_valid_children += 1;

            population.push(new_ind);

        }

        population.sort_by(|a, b| b.val.partial_cmp(&a.val).unwrap());

        population = population.iter().take(ga_args.pop_size as usize).map(|x| x.clone()).collect::<Vec<Individual>>();

    };

    return avg_fitness_t;

}


struct TicketProblem {
    n_periods: u32,
    wtp: Vec<f32>,
    wtps: Vec<f32>,
    occurrence_prob: Vec<Vec<f32>>,
    capacity: u32,
    group_sizes: Vec<u32>,
    total_individuals: u32,
    n_groups: u32,
}


enum SelectionStrategy {
    Roulette,
    Tournament
}


impl fmt::Display for SelectionStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SelectionStrategy::Roulette => write!(f, "Roulette"),
            SelectionStrategy::Tournament => write!(f, "Tournament"),
        }
    }
}

struct GAAgrs {
    pop_size: u32,
    n_iter: u32,
    n_resample: u32,
    n_children: u32,
    mutation_rate: f32,
    selection_strategy: SelectionStrategy
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut problem = TicketProblem {
        group_sizes: vec![1, 2, 3],
        capacity: 100,
        n_periods: 60,
        wtp: vec![0.2],
        wtps: vec![0.4],
        occurrence_prob: vec![vec![0.2, 0.4]],
        total_individuals: 1000,
        n_groups: 6,
    };

    let group_sizes = sample_group_sizes(problem.n_groups, problem.total_individuals);
    problem.group_sizes = group_sizes;
    
    let wtp_g_dist = Normal::new(0.0, 1.0).unwrap();
    let wtps_g_dist = Normal::new(0.0, 1.0).unwrap();
    
    let occ_probs = init_occurence_probs(problem.n_periods as usize, &problem.group_sizes.iter().map(|&x| x as usize).collect());
    problem.occurrence_prob = occ_probs;

    // sample wtp for each group
    let wtp_g = (0..problem.n_groups).map(|_| (wtp_g_dist.sample(&mut rng) as f32).abs() * 4.0 + 4.0).collect::<Vec<f32>>();
    let wtps_g: Vec<f32> = (0..problem.n_groups).map(|_| (wtps_g_dist.sample(&mut rng) as f32).abs() * 2.0 + 2.0).collect::<Vec<f32>>();
    
    let mut customers = Vec::new();

    

    for g in 0..problem.n_groups {

        let g_idx = g as usize;

        for i in 0..problem.group_sizes[g as usize] {
            let customer = Customer {
                group: g as u32,
                // period_occ: None,
                wtp: sample_halfnormal(&mut rng, wtp_g[g_idx], wtps_g[g_idx]),
                bought: false,
                id: i as u32
            };
            customers.push(customer);
        }
    }


    let mut ga_args = GAAgrs {
        pop_size: 100,
        n_iter: 10,
        n_resample: 3,
        n_children: 100,
        mutation_rate: 0.3,
        selection_strategy: SelectionStrategy::Roulette
    };

    
    
    // let ind = init_individual(&mut rng, &problem, &customers, ga_args.n_resample);
    
    
    
    

    let mut results_file = File::create("results_experiment_tau.csv").unwrap();

    
    // let avg_fitness_t = run(&ga_args, problem, customers, population, &mut rng);
    // let gain_per_eval = (avg_fitness_t[ga_args.n_iter as usize - 1] - avg_fitness_t[0])/n_evals as f32;
    
    
    // 
    // println!("Number of evaluations: {}", n_evals);
    
    results_file.write("n_evals,parameter,iteration,avg_fitness\n".as_bytes()).unwrap();

    let mut tau = 3.0;
    let n_resample = 3;

    
    // for mutation_rate in 2..7 {
    // for strategy in vec![SelectionStrategy::Roulette, SelectionStrategy::Tournament] {
    for tau in vec![1, 3, 5] {
                // ga_args.mutation_rate = mutation_rate as f32 / 10.0;
        ga_args.n_resample = n_resample;

        println!("Parameter: {}", tau);
        let n_evals = ga_args.pop_size * n_resample + ga_args.n_children * ga_args.n_resample * ga_args.n_iter;
        
        for run_t in 0..3 {
            let mut it = 0;
            let population = (0..ga_args.pop_size).map(|_| init_individual(&mut rng, &problem, &customers, ga_args.n_resample, tau as f32)).collect::<Vec<Individual>>();
            let avg_fitness_t = run(&ga_args, &problem, &customers, population.clone(), &mut rng);
            for fitness in avg_fitness_t.iter() {

                let row = format!("{},{},{},{}\n", n_evals, tau, it, fitness);

                results_file.write_all(row.as_bytes()).unwrap();

                it += 1;
            }
        }
    }

}
