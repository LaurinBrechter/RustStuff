


fn total_distance(dist_mat: &Vec<Vec<i32>>, solution: &Vec<i32>) -> i32 {
    
    let mut cost = 0;
    
    let mut idx = 0;
    for node in solution {
        idx +=1;
        if idx == solution.len() {
            cost += dist_mat[node.clone() as usize][solution[0] as usize];
        } else {
            cost += dist_mat[node.clone() as usize][solution[idx] as usize];
        }
        println!("{}", cost)
    }

    return cost
}

fn get_swap_candidates(dist_mat: &Vec<Vec<i32>>) {
     let candidates = vec![];
}


fn main() {

    let dist_mat = vec![
        vec![0,2,1,8],
        vec![2,0,22,6],
        vec![1,22,0,12],
        vec![8,6,12,0]
    ];

    let n_nodes = dist_mat.len();

    let mut taboo_mat = vec![
        vec![0;n_nodes];n_nodes
    ];

    let mut solution = vec![0,1,2,3];


    let cost = total_distance(&dist_mat, &solution);

    println!("Hello, world! {}", cost);
}
