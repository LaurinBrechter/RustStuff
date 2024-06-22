
use std::hash::{DefaultHasher, Hash};

use sha256::digest;


#[derive(Hash)]
struct Transaction {
    tx:i32,
    rx:i32,
    amount:i32
}

struct Chain {}


#[derive(Hash)]
struct Block {
    prev_hash: String,
    // previous_block: &'a Block<'a>,
    transactions: Vec<Transaction>,
    hash:String,
}

impl Block {

    fn new(transactions:Vec<Transaction>, prev_hash:String, &mut hasher) -> Self {
        transactions.hash(hasher);
        Self {
            transactions:
        }
    }
}

fn main() {

    let mut hasher = DefaultHasher::new();

    let inp = String::from("hello");
    let val = digest(inp);

    let t = Transaction { tx: 1, rx: 2, amount:10 };

}