
fn rastrigin(x: Vec<f64>) -> f64{

    const A: f64 = 10.0;

    let l = x.iter().map(|x| x*x - A*f64::cos(2.0*std::f64::consts::PI*x));
    
    return (x.len() as f64)*A + l.sum::<f64>()
}



fn main() {
    println!("Hello, world!");

    let x = vec![1.2, 3.0];
    let min_x = vec![0.0;10000000];

    use std::time::Instant;
    let now = Instant::now();

    let y = rastrigin(min_x);

    let elapsed = now.elapsed();


    println!("Result: {}, Elapsed: {:?}", y, elapsed)


}
