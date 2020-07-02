/*
DAY 18:
Write a program to count the minimum  number of steps to make a number divisible by another number.

Details:
Given 2 numbers, a and b, count the minimum number of steps you need to increment or decrement a to make it divisible by b. 
Each step means you can either add or subtract 1 from a.
That is, a+1 or a-1 only

Inputs:
First line will have the number of test cases, n.
In the next n lines, for each test case, Input two numbers a and b.

Outputs:
N lines showing number of steps for each test case

Sample:
Inputs
2
7 3
26 17
Outputs:
1
8

Explanation:-
In the first case, we can subtract 1 from 7 once, to get 6 which is divisible by 3.
In the second case, we can add 8 to 26 to make it 34, to make it divisible by 17.
*/


#[macro_use]
extern crate text_io;
use std::cmp::min;

fn main() {
    let n:i32 = read!();
    for _i in 0..n{
        let a:i32 = read!();
        let b:i32 = read!();
        println!("{}", min(a % b, b-(a % b)));

    }
}