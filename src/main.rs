use crate::vector::Vector;

mod btree;
mod vector;

fn main() {
    // println!("Hello, world!");

    println!("Result: {}", Vector::remove_duplicates(&mut vec![1,1,2]));
}
