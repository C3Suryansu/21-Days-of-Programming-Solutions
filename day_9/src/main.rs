/*
DAY 9:
Given a number n, the task is to find the odd factor sum.

Details:
The program is used to find the sum of all the odd factors for the given number.

Inputs:
Take any number as the input.

Outputs:
Print the sum of all the odd factors of that number and display it on the screen.

Sample:

Inputs:
30

Outputs:
24

(Hint : The odd factors of 30 are 1,3,5,15, so their sum Is 1+3+5+15 = 24)
*/

#[macro_use]
extern crate text_io;

fn main() {
    println!("Enter the number whose odd factors you want to find out: ");
    let num:i32 = read!();
    let mut sum:i32 = 0;
    for i in 1..(num/2 + 1){
        if (num % i == 0) && (i % 2 == 1){
            sum += i;
        }
    }
    println!("Sum of odd factors is : {}",sum);
}