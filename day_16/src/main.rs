/*
DAY 16:
Write a program to find the character which appears the most in a string, 
and then replace all occurrences of this character with the second most frequently appearing character in this string.

Details:
The program should first find the character which appears the most in the string, 
and then replace the character throughout the string with the next most frequently appearing character.

Inputs:
Input the string for working on.

Outputs:
The output should contain 2 lines:-
First line will be the character which appears the most
Second will be the new string with its most frequent character replaced by the next most frequent character in the string.
*/
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
    .expect("Failed to read line");
    line.pop();
    line.pop();

    let mut first_char:String = 'a'.to_string();
    let mut second_char:String = 'b'.to_string();
    let mut first_count = 0;
    let mut second_count = 0;
    
    for i in line.chars(){
        let mut count = 0;
        for j in line.chars(){
            if j == i{
                count += 1;
            }
        }
        if count >= first_count{
            first_count = count;
            first_char = i.to_string();
        }
        else if count > second_count{
            second_count = count;
            second_char = i.to_string();
        }
    }

    line = line.replace(&first_char, &second_char);
    println!("{}\n{}", first_char, line);
}
