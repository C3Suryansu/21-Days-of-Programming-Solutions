/*
DAY 10:
Given a list of numbers, write a program to swap first and last element of the list.

Details:
The program can be used to swap the first and last element in a list.

Inputs:
Input a list of n integers.

Outputs:
Print the list with the first and last elements swapped.

Sample:

Inputs
1  2  3  4 

Outputs:
4  2  3  1
*/
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("input");
    let mut nums = line.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();

    let first_val = nums[0];
    let length = nums.len();
    let last_val = nums[length - 1];

    nums[length - 1] = first_val;
    nums[0] = last_val;

    for i in nums{
        print!("{} ",i);
    }

}