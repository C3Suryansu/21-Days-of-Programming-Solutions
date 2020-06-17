/*
DAY 3:
Write a program to find the Sum of GP series.

Details:
The program can be used to find the sum of elements of Geometric Progression series.

Inputs:
Input the starting number of the G.P. series.
Input the number of items for the G.P. series.
Input the common ratio of G.P. series:

Outputs:
The numbers for the G.P. series.
The Sum of the G.P. series.

Sample:
Inputs
3
5
2

Outputs:
3 6 12 24 48
93

*/
#[macro_use]
extern crate text_io;

fn main() {
    let mut a : i64 = read!();
    let n : i64 = read!();
    let r : i64 = read!();
    let mut s : i64 = 0;

    for _i in 0..n{
        print!("{} ",a);
        s += a;
        a *=  r;
    }
    
    println!("");
    println!("{}", s);
}
