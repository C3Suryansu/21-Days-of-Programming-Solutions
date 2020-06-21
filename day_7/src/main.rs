/*
DAY 7:
Write a program to check if a given string is palindrome or not.

Details:
The program can be used to reverse a string or verify if it is a palindrome or not.

Inputs:
Input any string.

Outputs:
Print if the string is palindrome or not.

Sample:

Inputs
ABCDCBA

Outputs:
Yes

*/

#[macro_use]
extern crate text_io;

fn main() {
    let foo:String = read!();
    let new:&str = &foo;
    if new == new.chars().rev().collect::<String>(){
        println!("Palindrome");
    }else{
        println!("Not a palindrome");
    }
}