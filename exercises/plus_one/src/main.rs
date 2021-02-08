mod solution_one;

use crate::solution_one::plus_one;

fn main() {
    println!("Input {:?}, output: {:?}", vec![1,2,3], plus_one(vec![1,2,3]));
    println!("Input {:?}, output: {:?}", vec![9,8,7,6,5,4,3,2,1,1], plus_one(vec![9,8,7,6,5,4,3,2,1,1]));
    println!("Input {:?}, output: {:?}", vec![0,0], plus_one(vec![0,0]));
    println!("Input {:?}, output: {:?}", vec![0], plus_one(vec![0]));
    println!("Input {:?}, output: {:?}", vec![0,0,0], plus_one(vec![0,0,0]));
    println!("Input {:?}, output: {:?}", vec![7,2,8,5,0,9,1,2,9,5,3,6,6,7,3,2,8,4,3,7,9,5,7,7,4,7,4,9,4,7,0,1,1,1,7,4,0,0,6], plus_one(vec![7,2,8,5,0,9,1,2,9,5,3,6,6,7,3,2,8,4,3,7,9,5,7,7,4,7,4,9,4,7,0,1,1,1,7,4,0,0,6]));
}
