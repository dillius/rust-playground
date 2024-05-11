use crate::vector::Vector;

mod btree;
mod vector;

fn main() {
    // println!("Hello, world!");

    // println!("Result: {}", Vector::remove_duplicates(&mut vec![1,1,2]));
    println!("Result: {}", Vector::max_profit2(vec![7,1,5,3,6,4]));
}
