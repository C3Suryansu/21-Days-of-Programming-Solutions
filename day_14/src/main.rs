/*
DAY 14:
Write a program to replace all occurences of a substring in a string.

Details:
Inputs:
Input any string.
Input the substring you want to replace.
Input the string you want to replace it with.

Outputs:
Print the final string after replacing the substring with the string entered by the user.

Sample:
Inputs
The original string is : My name is Rohan
The substring you want to replace : Rohan
The string you want to use instead : Rahul

Outputs:
My name is Rahul

*/
#[macro_use]
extern crate text_io;
use std::io;

fn main() {
    println!("Enter the original String: ");
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence)
    .expect("Failed to read line");

    println!("Enter the substring you want to replace: ");
    let first_word:String = read!();
    let start_word = &first_word;

    println!("Enter the string you want to use instead: ");
    let second_word:String = read!();
    let end_word = &second_word;

    println!("\n{}",sentence.replace(start_word,end_word));
}
