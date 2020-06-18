/*
DAY 4:
Write a program to print all the prime numbers in an interval using function.

Details:
The program can be used to find the prime numbers in between two numbers. Write a function to check whether a number is prime or not.

Inputs:
Input the starting number.
Input the ending number.

Outputs:
Print all the prime numbers in between the two numbers.

Sample:

Inputs
20
50

Outputs: 
23 29 31 37 41 43 47
*/

#[macro_use]
extern crate text_io;

fn check_prime(num:i64){
    if ((num as f64).sqrt().floor() - (num as f64).sqrt()) == 0.0{
        print!("");
    }
    else if num == 2 || num == 3 {

        print!("{} ",num);
    }
    else if ((num - 1) % 6 == 0) || ((num + 1) % 6 == 0) {
        print!("{} ",num);
    }
}

fn main() {
    let start:i64 = read!();
    let end:i64 = read!();
    
    for _i in start..end{
        check_prime(_i);
    }
}

