/*
DAY 5:
Write a program to convert binary number to a character.

Details:
The program can be used to convert binary number to decimal number. 
Then the decimal number can be converted to the character whose ASCII value is same as the obtained decimal number.

Inputs:
Input a binary number.

Outputs:
Print the respective Character.

Sample:

Inputs
01000111

Outputs:
G

EASTER EGG:
Find the binary codes present in the poster of this event. Convert those using this program. 
Add the message obtained as a comment in the program.
*/

#[macro_use]
extern crate text_io;

fn main() {
    let start:String = read!();

    let new:&str = &start;
    let intval = isize::from_str_radix(new, 2).unwrap();
    println!("{}", intval as u8 as char);
}
//Easter Egg code: EUREKA