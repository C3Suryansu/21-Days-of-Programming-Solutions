/*
DAY 8:
Given the participants' score sheet for your University Sports Day, you are required to find the runner-up score. 
You are given  n scores. Store them in a list and find the score of the runner-up.

Details:

Input Format:
The first line contains n, the number of scores.
 The second line contains an array A[]  of n  integers each separated by a space.

Output Format
Print the runner-up score.

Sample:

Inputs:
5
 1 2 3 4 5

Outputs\:
4



*/
use std::io;

fn main() {
    let mut num = String::new();
    let mut line = String::new();
    io::stdin().read_line(&mut num).expect("input");
    io::stdin().read_line(&mut line).expect("input");
    let mut nums = line.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();

    nums.sort();
    nums.reverse();
    let max_value = nums[0];

    for num in nums{
        if num < max_value{
            print!("{}", num);
            break;
        }
    }
    //println!("");
    //println!("{}",max_value);

}