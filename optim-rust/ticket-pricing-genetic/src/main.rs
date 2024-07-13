mod recombinations;
mod mutations;
mod init_sim;
use recombinations::{recombine_price, recombine_n_offered};
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
    period_occ: Option<u32>,
    wtp: f32,
    bought: bool,
    id: u32
}


fn objective_fn(rng: &mut ThreadRng, problem: &TicketProblem, customers: &Vec<Customer>, ind:&Individual) -> f32 {

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



fn recombine(rng: &mut ThreadRng, problem: &TicketProblem, ind1: &Individual, ind2: &Individual, customers: &Vec<Customer>) -> Individual {
    let new_price = recombine_price(&ind1.price, &ind2.price);
    let new_n_offered = recombine_n_offered(&ind1.n_offered, &ind2.n_offered);

    let mut new_ind =  Individual {
        price: new_price,
        n_offered: new_n_offered,
        val: 0.0
    };

    // new_ind.val = objective_fn(rng, problem, customers, &new_ind);

    return new_ind;
}


fn mutate(problem: &TicketProblem, ind: &mut Individual) {
    mutations::mutate_price(&mut ind.price);
    mutations::mutate_n_tickets_offered(&mut ind.n_offered, problem.capacity as i32);
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

    new_ind.val = objective_fn(rng, problem, customers, &new_ind);
    new_ind
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


struct GAAgrs {
    pop_size: u32,
    n_iter: u32,
    n_resample: u32,
    n_children: u32,
    mutation_rate: f32,
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

    let new_ind = recombine(&mut rng, &problem, &ind1, &ind2, &customers);

    println!("New Ind: {:?}", &new_ind);




    let ga_args = GAAgrs {
        pop_size: 100,
        n_iter: 10,
        n_resample: 10,
        n_children: 100,
        mutation_rate: 0.1,
    };

    let mut population = (0..ga_args.pop_size).map(|_| init_individual(&mut rng, &problem, &customers)).collect::<Vec<Individual>>();


    for _ in 0..10 {
        
        // sum up objective values
        let mut obj_val_sum = 0.0;
        for ind in &population {
            obj_val_sum += ind.val;
        }

        println!("Avg fitness: {}", obj_val_sum / ga_args.pop_size as f32);

        

        let mut n_valid_children = 0;
        while n_valid_children < ga_args.n_children {

            let mut parents: Vec<&Individual> = Vec::new();

            while parents.len() < 2{
                for ind in &population {
                    if rng.gen::<f32>() < ind.val / obj_val_sum {
                        parents.push(&ind)
                    }
                }
            }

            let ind1 = parents[0];
            let ind2 = parents[1];
            let mut new_ind = recombine(&mut rng, &problem, &ind1.clone(), &ind2.clone(), &customers);
            
            if rng.gen::<f32>() < ga_args.mutation_rate {
                mutate(&problem, &mut new_ind);
            }

            new_ind.val = objective_fn(&mut rng, &problem, &customers, &new_ind);

            n_valid_children += 1;

            population.push(new_ind);

        }

        println!("Population size: {}", population.len());

        population.sort_by(|a, b| b.val.partial_cmp(&a.val).unwrap());

        population = population.iter().take(ga_args.pop_size as usize).map(|x| x.clone()).collect::<Vec<Individual>>();

        // print obj values
        // for ind in &population {
        //     println!("{}", ind.val);
        // }

    }   

}
