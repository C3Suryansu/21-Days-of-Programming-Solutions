/*
DAY 6:
Write a program to find the frequency of each digit in a given integer.

Details:
The program can be used to count the number of digits in a given number.

Inputs:
Input any number.

Outputs:
Print the respective frequency of a digit.

Sample:

Inputs
122345

Outputs:
The frequency of 0 = 0
The frequency of 1 = 1
The frequency of 2 = 2
The frequency of 3 = 1
The frequency of 4 = 1
The frequency of 5 = 1
The frequency of 6 = 0
The frequency of 7 = 0
The frequency of 8 = 0
The frequency of 9 = 0
*/

#[macro_use]
extern crate text_io;

fn main() {
    let num:String = read!();

    let zero = num.matches('0').count();
    println!("The frequency of 0 = {}",zero);

    let one = num.matches('1').count();
    println!("The frequency of 1 = {}",one);

    let two = num.matches('2').count();
    println!("The frequency of 2 = {}",two);

    let three = num.matches('3').count();
    println!("The frequency of 3 = {}",three);

    let four = num.matches('4').count();
    println!("The frequency of 4 = {}",four);

    let five = num.matches('5').count();
    println!("The frequency of 5 = {}",five);

    let six = num.matches('6').count();
    println!("The frequency of 6 = {}",six);

    let seven = num.matches('7').count();
    println!("The frequency of 7 = {}",seven);

    let eight = num.matches('8').count();
    println!("The frequency of 8 = {}",eight);

    let nine = num.matches('9').count();
    println!("The frequency of 9 = {}",nine);
}