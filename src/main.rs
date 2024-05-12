use crate::vector::Vector;

mod btree;
mod vector;

fn main() {
    // println!("Hello, world!");

    // println!("Result: {}", Vector::remove_duplicates(&mut vec![1,1,2]));
    // println!("Result: {}", Vector::max_profit2(vec![7,1,5,3,6,4]));
    let to_rotate = &mut vec![1,2,3,4,5,6,7];
    // println!("Result: {:?}", { Vector::rotate(to_rotate, 3); to_rotate });
    // println!("Result: {:?}", { Vector::rotate2(to_rotate, 3); to_rotate });
    println!("Result: {:?}", { Vector::rotate3(to_rotate, 3); to_rotate });
}
