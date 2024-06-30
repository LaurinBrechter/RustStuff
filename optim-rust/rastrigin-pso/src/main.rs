use std::{collections::btree_set::Intersection, process::exit};

use rand::{rngs::ThreadRng, Rng};


fn rastrigin(x: Vec<f64>) -> f64{

    const A: f64 = 10.0;

    let l = x.iter().map(|x| x*x - A*f64::cos(2.0*std::f64::consts::PI*x));
    
    return (x.len() as f64)*A + l.sum::<f64>()
}

struct Swarm {
    global_best: Vec<f32>,
    inertia: f32,
    c1: f32,
    c2: f32,
}


impl Vec<T> {
    fn add()
}



fn add_vectors(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
    a.iter().zip(b.iter()).map(|(x,y)| x+y).collect()
}

fn scalar_multiply(a: &Vec<f32>, b:f32) -> Vec<f32> {
    a.iter().map(|x| x*b).collect()
}


struct Particle<'a> {
    swarm: &'a Swarm,
    position: Vec<f32>,
    speed: Vec<f32>,
    local_best: Vec<f32>,
}

impl<'a> Particle<'a> {
    fn new_position(&self)  -> Vec<f32> {
        return add_vectors(&self.speed, &self.position)
    }
    fn new_speed(&self, rng: & mut ThreadRng) -> Vec<f32> {

        let inertia = self.swarm.inertia;
        let r1 = rng.gen::<f32>();

        let r2 = rng.gen::<f32>();

        let negated_pos: Vec<f32> = self.local_best.iter().map(|x| -x).collect();


        return scalar_multiply(&self.speed, inertia) +



    }
}


fn main() {
    println!("Hello, world!");
    

    let mut rng = rand::thread_rng();

    println!("{}", rng.gen::<f32>());
    

    exit(0);

    // let x = vec![];
    // let min_x = vec![0.0;10000000];

    // use std::time::Instant;
    // let now = Instant::now();

    // let y = rastrigin(min_x);

    // let elapsed = now.elapsed();


    // println!("Result: {}, Elapsed: {:?}", y, elapsed)


}
