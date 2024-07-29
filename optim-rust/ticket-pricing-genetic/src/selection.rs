use rand::prelude::*;

use crate::Individual;




pub fn roulette_selection(population:&Vec<Individual>, obj_val_sum:f32, rng: &mut ThreadRng) -> (Individual, Individual) {


    let mut parents: Vec<Individual> = Vec::new();

    while parents.len() < 2{
        for ind in population {
            if rng.gen::<f32>() < ind.val / obj_val_sum {
                parents.push(ind.clone())
            }
        }
    }

    (parents[0].clone(), parents[1].clone())

}

pub fn tournament_selection(population:&Vec<Individual>, tournament_size:usize, rng: &mut ThreadRng) -> (Individual, Individual) {

    let mut parents: Vec<Individual> = Vec::new();

    while parents.len() < 2{
        let mut tournament: Vec<Individual> = Vec::new();
        for _ in 0..tournament_size {
            let ind = population[rng.gen_range(0..population.len())].clone();
            tournament.push(ind);
        }

        let best = tournament.iter().max_by(|a, b| a.val.partial_cmp(&b.val).unwrap()).unwrap();
        parents.push(best.clone());
    }

    (parents[0].clone(), parents[1].clone())

}