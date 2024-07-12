use ndarray_rand::RandomExt;
use rand_distr::{Beta, Binomial, Distribution, Normal, Uniform};
use rand::prelude::*;
use ndarray::Array1;

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

#[derive(Clone)]
#[derive(Debug)]
struct Individual {
    price: Vec<f32>,
    n_offered: Vec<i32>,
    val: f32,
}

fn sample_group_sizes(n_groups: u32, n_individuals:u32) -> Vec<u32> {
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


fn init_occurence_probs(n_periods: usize, group_sizes: Vec<usize>) -> Vec<Vec<f32>> {
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


fn sample_halfnormal(rng: &mut ThreadRng, loc: f32, scale: f32) -> f32 {
    let normal_dist = Normal::new(0.0, 1.0).unwrap();
    let mut val = normal_dist.sample(rng);
    while val < 0.0 {
        val = normal_dist.sample(rng);
    }
    return val * scale + loc;
}


#[derive(Clone)]
struct Customer {
    group: u32,
    period_occ: Option<u32>,
    wtp: f32,
    bought: bool,
    id: u32
}


fn objective_fn(rng: &mut ThreadRng, problem: &TicketProblem, customers: &Vec<Customer>, ind:Individual) -> f32 {

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
            customers_c.iter_mut().find(|c| c.id == customer.id).unwrap().bought = true;
        }

        if tickets_left == 0 {
            break;
        }

    }
    return revenue;
}


fn recombine_price(price1: &Vec<f32>, price2: &Vec<f32>) -> Vec<f32> {
    let mut price_new = price1.clone();
    let t = price1.len();
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..t);

    for i in idx..t {
        price_new[i] = price2[i];
    }

    price_new
}

fn recombine_n_offered(n_offered1: &Vec<i32>, n_offered2: &Vec<i32>) -> Vec<i32> {
    let mut n_offered_new = n_offered1.clone();
    let t = n_offered1.len();
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..t);

    for i in idx..t {
        n_offered_new[i] = n_offered2[i];
    }

    n_offered_new
}


fn recombine(rng: &mut ThreadRng, problem: &TicketProblem, ind1: Individual, ind2: Individual, customers: &Vec<Customer>) -> Individual {
    let new_price = recombine_price(&ind1.price, &ind2.price);
    let new_n_offered = recombine_n_offered(&ind1.n_offered, &ind2.n_offered);

    let mut new_ind =  Individual {
        price: new_price,
        n_offered: new_n_offered,
        val: 0.0
    };

    new_ind.val = objective_fn(rng, problem, customers, new_ind.clone());

    return new_ind;
}

fn init_individual(rng: &mut ThreadRng, problem: &TicketProblem, customers: &Vec<Customer>) -> Individual {
    
    let price_dist = Normal::new(2.0, 2.0).unwrap();
    let price = (0..problem.n_periods).map(|_| price_dist.sample(rng)).collect::<Vec<f32>>();
    
    let n_offered_dist = Uniform::new(0, 100);
    let n_offered = (0..problem.n_periods).map(|_| n_offered_dist.sample(rng)).collect::<Vec<i32>>();
    
    let mut new_ind =  Individual {
        price: price,
        n_offered: n_offered,
        val: 0.0
    };

    new_ind.val = objective_fn(rng, problem, customers, new_ind.clone());
    new_ind
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
        total_individuals: 200,
        n_groups: 6,
    };

    let group_sizes = sample_group_sizes(problem.n_groups, problem.total_individuals);
    problem.group_sizes = group_sizes.clone();
    let wtp_g_dist = Normal::new(0.0, 1.0).unwrap();
    let wtps_g_dist = Normal::new(0.0, 1.0).unwrap();
    
    let occ_probs = init_occurence_probs(problem.n_periods as usize, group_sizes.iter().map(|&x| x as usize).collect());
    problem.occurrence_prob = occ_probs;

    // sample wtp for each group
    let wtp_g = (0..problem.n_groups).map(|_| (wtp_g_dist.sample(&mut rng) as f32).abs() * 4.0 + 4.0).collect::<Vec<f32>>();
    let wtps_g: Vec<f32> = (0..problem.n_groups).map(|_| (wtps_g_dist.sample(&mut rng) as f32).abs() * 2.0 + 2.0).collect::<Vec<f32>>();
    
    let mut customers = Vec::new();


    for g in 0..problem.n_groups {

        let g_idx = g as usize;

        for i in 0..group_sizes[g as usize] {
            let customer = Customer {
                group: g as u32,
                period_occ: None,
                wtp: sample_halfnormal(&mut rng, wtp_g[g_idx], wtps_g[g_idx]),
                bought: false,
                id: i as u32
            };
            customers.push(customer);
        }
    }

    let ind1 = init_individual(&mut rng, &problem, &customers);
    let ind2 = init_individual(&mut rng, &problem, &customers);

    let new_ind = recombine(&mut rng, &problem, ind1, ind2, &customers);

    println!("New Ind: {:?}", &new_ind);

    // sample objective 1000 times
    // let mut revs = Vec::new();
    // for _ in 0..1000 {
    //     revs.push(objective_fn(&mut rng, &problem, &customers, Individual {
    //         price: vec![0.2; problem.n_periods as usize],
    //         n_offered: vec![10; problem.n_periods as usize],
    //         val: 0.0
    //     }));
    // }




}
