struct Item {
    value: i32,
    weight: i32,
}

struct Backpack {
    items: Vec<Item>,
    capacity: i32,
}

impl Backpack {
    fn total_weight(&self) -> i32 {
        let mut total = 0;
        for item in self.items.iter() {
            total += item.weight;
        }

        return total;
    }
    fn is_valid(&self) -> bool {
        return self.total_weight() <= self.capacity;
    }

    fn total_value(&self) -> i32 {
        let mut total = 0;
        for item in self.items.iter() {
            total += item.value;
        }

        return total;
    }

    fn new(values: Vec<i32>, weights: Vec<i32>, capacity: i32) -> Self {
        let mut items = Vec::new();
        for i in 0..values.len() {
            items.push(Item {
                value: values[i],
                weight: weights[i],
            });
        }

        Self { items, capacity }
    }
}

// struct Iteration {
//     taboo_list: Vec<i32>,
//     candidate_list: Vec<i32>,
// }

struct Solution<'a> {
    backpack: &'a Backpack,
    encoding: Vec<i32>,
    candidate_list: Vec<i32>,
    taboo_list: Vec<i32>,
}

impl<'a> Solution<'a> {
    fn new(backpack: &'a Backpack, encoding: Option<Vec<i32>>) -> Option<Self> {
        match encoding {
            None => {
                let mut encoding = Vec::new();
                for _ in 0..backpack.items.len() {
                    encoding.push(0);
                }
                return Some(Self {
                    backpack,
                    encoding,
                    taboo_list: [].to_vec(),
                    candidate_list: [].to_vec(),
                });
            }
            Some(e) => {
                if e.len() == backpack.items.len() {
                    return Some(Self {
                        backpack,
                        encoding: e,
                        taboo_list: [].to_vec(),
                        candidate_list: [].to_vec(),
                    });
                } else {
                    return None;
                }
            }
        }
    }

    fn generate_candidates(&self, taboo_remove: &Vec<i32>, taboo_add: &Vec<i32>) -> Vec<Vec<i32>> {
        // fn generate_candidates(&self) -> Vec<Vec<i32>> {
        let mut allowed_elements = [].to_vec();
        let mut can_add = [].to_vec();
        let mut can_remove = [].to_vec();
        let mut candidates = [].to_vec();

        for i in 0..self.encoding.len() {
            let idx = i as i32;

            if self.encoding[i] == 1 {
                allowed_elements.push(idx);
                if !taboo_remove.contains(&idx) {
                    can_remove.push(idx)
                }
            } else {
                if !taboo_add.contains(&idx) {
                    can_add.push(idx)
                }
            }
        }

        for el in can_add.iter() {
            let mut a = allowed_elements.to_owned();
            a.push(el.to_owned());
            candidates.push(a)
        }

        for el in can_remove.iter() {
            let mut a = allowed_elements.to_owned();
            a.push(el)
        }

        return candidates;
    }

    fn is_valid(&self) -> (bool, i32, i32) {
        let mut total_weight = 0;
        let mut total_value = 0;
        for i in 0..self.encoding.len() {
            if self.encoding[i] == 1 {
                total_weight += self.backpack.items[i].weight;
                total_value += self.backpack.items[i].value;
            }
        }

        return (
            total_weight <= self.backpack.capacity,
            total_value,
            total_weight,
        );
    }
}

fn main() {
    println!("Hello, world!");

    let b = Backpack {
        items: vec![
            Item {
                value: 6,
                weight: 9,
            },
            Item {
                value: 7,
                weight: 10,
            },
            Item {
                value: 5,
                weight: 14,
            },
            Item {
                value: 8,
                weight: 11,
            },
            Item {
                value: 5,
                weight: 8,
            },
            Item {
                value: 3,
                weight: 8,
            },
        ],
        capacity: 50,
    };

    let solution = Solution::new(&b, Some([1, 0, 1, 0, 1, 1].to_vec())).unwrap();

    println!("Total weight: {}", b.total_weight());
    println!("Total value: {}", b.total_value());
    let (is_valid, tv, tw) = solution.is_valid();
    println!("Solution Is valid: {}", is_valid);

    println!("Solution encoding : {:?}", solution.encoding);

    let taboo_remove = vec![0];
    let taboo_add = vec![2];

    print!(
        "Candidates: {:?}",
        solution.generate_candidates(&taboo_remove, &taboo_add)
    )
}
