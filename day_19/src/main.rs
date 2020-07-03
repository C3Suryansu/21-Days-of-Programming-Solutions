/*
DAY 19:
Find the probability of a perfect square occuring when a number is randomly choosen between numbers a and b over n iterations.
Details:
Given 2 numbers, a and b, you have to find that, over n iterations,
what is the probability that if a number is picked randomly in between a and b, it turns out to be a perfect square.
Inputs:
Input numbers a, b and n.
Outputs:
Print the probability in decimal
*/


#[macro_use]
extern crate text_io;
use rand::Rng;

fn main() {
    let a:i64 = read!();
    let b:i64 = read!();
    let iter:i64 = read!();
    let mut count:f64 = 1.0;

    for _i in 0..(iter + 1){
        let num = rand::thread_rng().gen_range(a, b + 1);
        if ((num as f64).sqrt().floor() - (num as f64).sqrt()) == 0.0{
            print!("{} ", num);
            count += 1.0;
        }
    }
    println!("\n {}", count/iter as f64)
}
