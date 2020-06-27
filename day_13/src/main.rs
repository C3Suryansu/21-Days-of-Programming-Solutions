/*
DAY 13:
Given a positive integer n( 1 <= n <= 1015). Find the largest prime factor of a number.	

Details:
The program can be used to find the largest prime factor of a number.

Inputs:
Input any number.

Outputs:
Print largest prime factor of the number.

Sample:

Input:
6

Output:
3
*/

#[macro_use]
extern crate text_io;

fn check_prime(num: i64) -> bool{
    let mut c: i64 = 0;
    for i in 2..(num/2 + 1) {
        if num % i == 0 {
            c = 1;
            break;
        }
    }
    if c == 0{
        true
    } else{
        false
    }
}

fn main() {
    let num:i64 = read!();
    let mut max:i64 = 0;

    for _i in 2..(num + 1){
        if check_prime(_i){
            if num % _i == 0 && _i > max{
                max = _i;
            }
        }
    }
    println!("{}",max);
}

